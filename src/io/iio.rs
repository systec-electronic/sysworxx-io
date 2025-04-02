// SPDX-License-Identifier: LGPL-3.0-or-later
//
// (c) SYSTEC electronic AG, D-08468 Heinsdorfergrund, Am Windrad 2
//     www.systec-electronic.com

use std::fmt;
use std::sync::mpsc::{sync_channel, SyncSender, TrySendError};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

use industrial_io as iio;
use ini::Ini;

use crate::convert::tc;
use crate::error::{Error, Result};
use crate::ffi;
use crate::io::util;
use crate::io::util::PairMap;
use crate::periodic::Periodic;
use crate::shm;
use crate::{AnalogInput, AnalogOutput, IoChannel, TempSensor};

pub enum AttrValue {
    F64(f64),
    I64(i64),
}

pub enum DevAttr {
    SamplingFrequency(AttrValue),
}

pub enum ChanAttr {
    VoltageScale(AttrValue),
}

pub fn lookup_id_for_spi(bus: usize, slave: usize) -> Result<String> {
    use std::path::Path;

    let spi_path = format!("/sys/bus/spi/devices/spi{}.{}", bus, slave);
    let spi_path = Path::new(&spi_path);

    for entry in spi_path.read_dir()? {
        let entry = entry.map_err(Error::AccessFailed)?;
        let path = entry.path();

        if path.is_dir() {
            let dirname = path
                .file_name()
                .map(|os_name| os_name.to_string_lossy())
                .ok_or_else(Error::generic_access_error)?;

            if dirname.contains("iio:device") {
                return Ok(dirname.into());
            }
        }
    }

    Err(Error::generic_access_error())
}

pub struct Sampler<T: Getter + Send + Copy> {
    inner: Arc<Mutex<SamplerInner<T>>>,
}

impl<T: 'static + Getter<Out = T> + Send + Copy + std::fmt::Debug> Sampler<T> {
    pub fn new(name: &str, poll_time: Duration) -> Sampler<T> {
        Sampler {
            inner: Arc::new(Mutex::new(SamplerInner::new(name, poll_time))),
        }
    }

    pub fn get_notifier(&self) -> crossbeam_channel::Receiver<shm::Event> {
        let inner = self.inner.lock().unwrap();
        inner.get_notifier()
    }

    pub fn from_spi(bus: usize, slave: usize, poll_time: Duration) -> Sampler<T> {
        let adc_name = lookup_id_for_spi(bus, slave).unwrap();
        Sampler::new(&adc_name, poll_time)
    }

    pub fn attr_write(&mut self, attribute: DevAttr) -> Result<()> {
        let mut inner = self.inner.lock().unwrap();
        inner.attr_write(attribute)
    }
}

struct SamplerInner<T: Getter + Send + Copy> {
    values: Arc<Mutex<PairMap<usize, Option<T>>>>,
    notifier: crossbeam_channel::Receiver<shm::Event>,
    name: String,
}

impl<T: 'static + Getter<Out = T> + Send + Copy + std::fmt::Debug> SamplerInner<T> {
    pub fn new(name: &str, poll_time: Duration) -> SamplerInner<T> {
        let values = Arc::new(Mutex::new(PairMap::default()));
        let values_cloned = Arc::clone(&values);

        let (tx, notifier) = crossbeam_channel::unbounded();

        let name_cloned = name.to_owned();

        thread::Builder::new()
            .name(name_cloned.to_owned())
            .spawn(move || {
                let ctx = iio::Context::create_local().unwrap();
                let dev = ctx.find_device(&name_cloned).unwrap();

                // this iio device cannot be accessed via iio index (libiio API) therefore we sort
                // the channels by their name (e.g. 'voltage0', 'voltage1', ...)
                let mut channels: Vec<iio::channel::Channel> = dev.channels().collect();

                // Some devices behave different than others. This is a heuristic
                // workaround to try to be compatible with all iio device implementations
                // and should be investigated.
                if channels[0].id().is_some() {
                    channels
                        .sort_by(|x, y| human_sort::compare(&x.id().unwrap(), &y.id().unwrap()));
                } else {
                    channels.sort_by_key(|x| x.index().unwrap());
                }

                let mut interval = Periodic::new(poll_time);
                loop {
                    interval.next();

                    for (index, value) in values_cloned.lock().unwrap().iter_mut() {
                        if let Some(channel) = channels.get(*index) {
                            trace!("{} => {:?}", index, value);
                            *value = Some(T::read(channel));
                        } else {
                            error!("invalid channel");
                        }
                    }

                    tx.send(shm::Event::Update).ok();

                    debug!("{}: {:?}", &name_cloned, interval.elapsed())
                }
            })
            .unwrap();

        SamplerInner {
            values,
            notifier,
            name: name.to_owned(),
        }
    }

    fn register(&mut self, index: usize) {
        let mut values = self.values.lock().unwrap();
        values.set(index, None);
    }

    fn get(&mut self, index: usize) -> Option<T> {
        let values = self.values.lock().unwrap();
        *values.get(index).unwrap()
    }

    fn attr_write(&mut self, attribute: DevAttr) -> Result<()> {
        let ctx = iio::Context::create_local().map_err(|_| Error::GenericError)?;
        let dev = ctx.find_device(&self.name).ok_or(Error::GenericError)?;

        use DevAttr::*;
        let (name, value) = match attribute {
            SamplingFrequency(value) => ("sampling_frequency", value),
        };

        use AttrValue::*;
        match value {
            F64(v) => dev
                .attr_write_float(name, v)
                .map_err(|_| Error::GenericError),
            I64(v) => dev.attr_write_int(name, v).map_err(|_| Error::GenericError),
        }
    }

    fn chan_attr_write(&mut self, index: usize, attribute: ChanAttr) -> Result<()> {
        let ctx = iio::Context::create_local().map_err(|_| Error::GenericError)?;
        let dev = ctx.find_device(&self.name).ok_or(Error::GenericError)?;
        let chan = dev.get_channel(index).map_err(|_| Error::GenericError)?;

        use ChanAttr::*;
        let (name, value) = match attribute {
            VoltageScale(value) => ("scale", value),
        };

        use AttrValue::*;
        match value {
            F64(v) => chan
                .attr_write_float(name, v)
                .map_err(|_| Error::GenericError),
            I64(v) => chan
                .attr_write_int(name, v)
                .map_err(|_| Error::GenericError),
        }
    }

    pub fn get_notifier(&self) -> crossbeam_channel::Receiver<shm::Event> {
        self.notifier.clone()
    }
}

pub trait Getter {
    type Out;
    fn read(_channel: &iio::Channel) -> Self::Out;
}

impl Getter for i64 {
    type Out = i64;
    fn read(channel: &iio::Channel) -> i64 {
        match channel.attr_read_int("raw") {
            Ok(value) => value,
            Err(e) => {
                error!("iio read failed: {} [{:?}]", e, thread::current().name());
                0
            }
        }
    }
}

impl Getter for f64 {
    type Out = f64;
    fn read(channel: &iio::Channel) -> f64 {
        match channel.attr_read_float("input") {
            Ok(value) => value,
            Err(e) => {
                error!("iio read failed: {} [{:?}]", e, thread::current().name());
                0.0
            }
        }
    }
}

pub struct Writer<T: Setter + Send + Copy + std::fmt::Debug> {
    inner: Arc<Mutex<WriterInner<T>>>,
}

impl<T: 'static + Setter<In = T> + Send + Copy + std::fmt::Debug> Writer<T> {
    pub fn new(name: &str) -> Writer<T> {
        Writer {
            inner: Arc::new(Mutex::new(WriterInner::new(name))),
        }
    }

    pub fn from_spi(bus: usize, slave: usize) -> Writer<T> {
        let name = lookup_id_for_spi(bus, slave).unwrap();
        Writer::new(&name)
    }
}

struct WriterInner<T: Setter + Send + Copy + std::fmt::Debug> {
    values: Arc<Mutex<PairMap<usize, Option<T>>>>,
    notifier: SyncSender<()>,
}

impl<T: 'static + Setter<In = T> + Send + Copy + std::fmt::Debug> WriterInner<T> {
    pub fn new(name: &str) -> WriterInner<T> {
        let name_cloned = name.to_owned();

        let values = Arc::new(Mutex::new(PairMap::default()));
        let values_cloned = values.clone();

        // this channel will allow only one item in it, which is what we want.
        // if buffered we may get issues, when values are written really fast.
        let (ping_tx, ping_rx) = sync_channel(1);

        thread::Builder::new()
            .name(name_cloned.to_owned())
            .spawn(move || {
                let ctx = iio::Context::create_local().unwrap();
                let dev = ctx.find_device(&name_cloned).unwrap();

                // this iio device cannot be accessed via iio index (libiio API) therefore we build
                // our own index here
                let mut channels: Vec<iio::channel::Channel> = dev.channels().collect();
                channels.sort_by(|x, y| human_sort::compare(&x.id().unwrap(), &y.id().unwrap()));

                loop {
                    // wait for some channel to change
                    ping_rx.recv().unwrap();

                    {
                        let mut values = values_cloned.lock().unwrap();

                        for (index, value) in values.iter_mut() {
                            if let Some(v) = value {
                                if let Some(channel) = channels.get(*index) {
                                    T::write(channel, *v);
                                } else {
                                    error!("invalid channel");
                                }
                            }

                            *value = None;
                        }
                    }
                }
            })
            .unwrap();

        WriterInner {
            values,
            notifier: ping_tx,
        }
    }

    fn write(&mut self, index: usize, value: T) -> Result<()> {
        let mut values = self.values.lock().map_err(|_| Error::GenericError)?;
        values.set(index, Some(value));

        self.notifier
            .try_send(())
            .or_else(|e| match e {
                // if the writer thread got already notified, we don't care and iognore this error
                TrySendError::Full(_) => Ok(()),
                _ => Err(e),
            })
            .map_err(|_| Error::GenericError)
    }
}

pub trait Setter {
    type In;
    fn write(channel: &iio::Channel, value: Self::In);
}

impl Setter for i64 {
    type In = i64;
    fn write(channel: &iio::Channel, value: i64) {
        match channel.attr_write_int("raw", value) {
            Ok(_) => {}
            Err(e) => error!("iio write failed: {}", e),
        }
    }
}

pub struct Ai {
    index: usize,
    sampler: Arc<Mutex<SamplerInner<i64>>>,
}

impl fmt::Debug for Ai {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "iio/Ai {}", self.index)
    }
}

impl Ai {
    pub fn new(sampler: &Sampler<i64>, index: usize) -> Ai {
        Ai {
            sampler: sampler.inner.clone(),
            index,
        }
    }
}

impl IoChannel for Ai {
    fn init(&mut self, _chan_number: usize) -> Result<()> {
        let mut sampler = self.sampler.lock().unwrap();
        sampler.register(self.index);
        Ok(())
    }

    fn shutdown(&mut self) -> Result<()> {
        Ok(())
    }
}

impl AnalogInput for Ai {
    fn get(&mut self) -> Result<i64> {
        let mut sampler = self.sampler.lock().unwrap();

        match sampler.get(self.index) {
            Some(val) => Ok(val),
            None => Ok(0),
        }
    }
}

pub struct Ao {
    index: usize,
    writer: Arc<Mutex<WriterInner<i64>>>,
    gain: f64,
    offset: f64,
    shifter: util::Shifter,
    clipper: util::Clip<i64>,
    last_value: i64,
}

impl fmt::Debug for Ao {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "iio/Ao {}", self.index)
    }
}

impl Ao {
    pub fn new(
        writer: &Writer<i64>,
        index: usize,
        shifter: util::Shifter,
        clipper: util::Clip<i64>,
        ini: &Ini,
        section: &str,
    ) -> Ao {
        let get_val = |entry, default| {
            ini.get_from(Some(section), entry)
                .and_then(|value| value.parse::<f64>().ok())
                .unwrap_or(default)
        };

        let gain = get_val("Gain", 1.0);
        let offset = get_val("Offset", 0.0);

        Ao {
            writer: writer.inner.clone(),
            index,
            shifter,
            gain,
            offset,
            clipper,
            last_value: std::i64::MAX,
        }
    }
}

impl IoChannel for Ao {
    fn init(&mut self, _chan_number: usize) -> Result<()> {
        Ok(())
    }

    fn shutdown(&mut self) -> Result<()> {
        Ok(())
    }
}

impl AnalogOutput for Ao {
    fn set(&mut self, value: i64) -> Result<()> {
        if value == self.last_value {
            return Ok(());
        }

        self.last_value = value;

        let value = self.shifter.shift(value);
        let value = ((value as f64) * self.gain + self.offset).round() as i64;

        let value = self.clipper.clip(value);

        let mut writer = self.writer.lock().unwrap();
        writer.write(self.index, value)
    }
}

pub struct TempRtd {
    index: usize,
    sampler: Arc<Mutex<SamplerInner<f64>>>,
    mode: ffi::IoTmpMode,
    sensor_type: ffi::IoTmpSensorType,
}

impl fmt::Debug for TempRtd {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "iio/Rtd {}", self.index)
    }
}

impl TempRtd {
    pub fn new(sampler: &Sampler<f64>, index: usize) -> TempRtd {
        TempRtd {
            sampler: sampler.inner.clone(),
            index,
            mode: ffi::IoTmpMode::RtdTwoWire,
            sensor_type: ffi::IoTmpSensorType::PT100,
        }
    }
}

impl IoChannel for TempRtd {
    fn init(&mut self, _chan_number: usize) -> Result<()> {
        let mut sampler = self.sampler.lock().unwrap();
        sampler.register(self.index);
        Ok(())
    }

    fn shutdown(&mut self) -> Result<()> {
        Ok(())
    }
}

impl TempSensor<f64> for TempRtd {
    fn get(&mut self) -> Result<f64> {
        let mut sampler = self.sampler.lock().unwrap();

        if let Some(ohms) = sampler.get(self.index) {
            Ok(ohms)
        } else {
            Ok(std::f64::MIN)
        }
    }

    fn set_mode(&mut self, mode: ffi::IoTmpMode, sensor_type: ffi::IoTmpSensorType) -> Result<()> {
        use ffi::IoTmpMode::*;
        use ffi::IoTmpSensorType::*;

        match mode {
            RtdTwoWire | RtdThreeWire | RtdFourWire => self.mode = mode,
        }

        match sensor_type {
            PT100 | PT1000 => self.sensor_type = sensor_type,
        }

        Ok(())
    }
}

// TODO: Move calibration values to separate structure for better reusability
pub struct TempTc {
    index: usize,
    index_ambient: usize,
    sampler: Arc<Mutex<SamplerInner<f64>>>,
    gain: f64,
    offset: f64,
    cj_offset: f64,
}

impl fmt::Debug for TempTc {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "iio/Tc {}", self.index)
    }
}

impl TempTc {
    pub fn new(
        sampler: &Sampler<f64>,
        ini: &Ini,
        section: &str,
        index_ambient: usize,
        index: usize,
    ) -> TempTc {
        let get_val = |entry, default| {
            ini.get_from(Some(section), entry)
                .and_then(|value| value.parse::<f64>().ok())
                .unwrap_or(default)
        };

        let gain = get_val("Gain", 1.0);
        let offset = get_val("Offset", 0.0);
        let cj_offset = get_val("ColdJunctionOffset", 0.0);

        TempTc {
            sampler: sampler.inner.clone(),
            index,
            index_ambient,
            gain,
            offset,
            cj_offset,
        }
    }

    pub fn attr_write(&mut self, attribute: ChanAttr) -> Result<()> {
        let mut sampler = self.sampler.lock().unwrap();
        sampler.chan_attr_write(self.index, attribute)
    }
}

impl IoChannel for TempTc {
    fn init(&mut self, _chan_number: usize) -> Result<()> {
        let mut sampler = self.sampler.lock().unwrap();
        sampler.register(self.index);
        sampler.register(self.index_ambient);
        Ok(())
    }

    fn shutdown(&mut self) -> Result<()> {
        Ok(())
    }
}

impl TempSensor<f64> for TempTc {
    fn get(&mut self) -> Result<f64> {
        let mut sampler = self.sampler.lock().unwrap();

        if let Some(mvolts) = sampler.get(self.index) {
            if let Some(ambient) = sampler.get(self.index_ambient) {
                let ambient = ambient + self.cj_offset;
                let mvolts = mvolts * self.gain + self.offset;
                Ok(tc::calc_temperature(ambient, mvolts))
            } else {
                Ok(std::f64::MIN)
            }
        } else {
            Ok(std::f64::MIN)
        }
    }
}
