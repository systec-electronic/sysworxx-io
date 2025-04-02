// SPDX-License-Identifier: LGPL-3.0-or-later
//
// (c) SYSTEC electronic AG, D-08468 Heinsdorfergrund, Am Windrad 2
//     www.systec-electronic.com

use std::collections::HashMap;
use std::fs::{write, File};
use std::io;
use std::io::prelude::*;
use std::path::Path;
use std::sync::{Arc, Mutex};

use crate::error::*;
use crate::ffi;
use crate::{DigitalInput, DigitalOutput, IoChannel, PwmOutput};

#[derive(Copy, Clone)]
enum Direction {
    In,
    Out,
    OutHigh,
}

#[derive(Debug)]
pub struct GpioSysFs {
    gpionum: usize,
    file: Option<File>,
}

impl GpioSysFs {
    const fn new(gpionum: usize) -> GpioSysFs {
        GpioSysFs {
            gpionum,
            file: None,
        }
    }

    fn is_exported(&self) -> bool {
        let value_file_path = self.value_file_path();
        Path::new(&value_file_path).exists()
    }

    fn export(&mut self) -> Result<()> {
        if let Err(e) = write("/sys/class/gpio/export", format!("{}", self.gpionum)) {
            // in case multiple processes try to export the same GPIO at the same time
            warn!("Failed to export gpio: {}", self.gpionum);
            std::thread::sleep(std::time::Duration::from_millis(100));
            if !std::path::Path::new(&format!("/sys/class/gpio/gpio{}", self.gpionum)).exists() {
                return Err(From::from(e));
            }
        }
        Ok(())
    }

    fn set_polarity(&mut self, polarity: &Polarity) -> Result<()> {
        let content = match polarity {
            Polarity::ActiveLow => "1",
            Polarity::ActiveHigh => "0",
        };
        write(
            format!("/sys/class/gpio/gpio{}/active_low", self.gpionum),
            content,
        )?;
        Ok(())
    }

    fn set_direction(&self, dir: Direction) -> Result<()> {
        let content = match dir {
            Direction::In => "in",
            Direction::Out => "out",
            Direction::OutHigh => "high",
        };
        write(
            format!("/sys/class/gpio/gpio{}/direction", self.gpionum),
            content,
        )?;
        Ok(())
    }

    fn value_file_path(&self) -> String {
        format!("/sys/class/gpio/gpio{}/value", self.gpionum)
    }

    fn open_value_file(&mut self, dir: Direction) -> Result<()> {
        let value_file_path = self.value_file_path();

        match dir {
            Direction::In => {
                self.file = Some(File::open(value_file_path)?);
            }
            Direction::Out | Direction::OutHigh => {
                self.file = Some(File::create(value_file_path)?);
            }
        };
        Ok(())
    }

    fn set(&mut self, val: bool) -> Result<()> {
        let v = if val { b"1" } else { b"0" };

        match &mut self.file {
            None => {
                return Err(Error::generic_access_error());
            }
            Some(f) => {
                f.write_all(v)?;
            }
        }

        Ok(())
    }

    fn get(&mut self) -> Result<bool> {
        let mut buffer = [0; 1];
        match &mut self.file {
            None => Err(Error::generic_access_error()),
            Some(f) => {
                f.seek(io::SeekFrom::Start(0))?;
                f.read_exact(&mut buffer)?;
                match buffer[0] {
                    b'0' => Ok(false),
                    _ => Ok(true),
                }
            }
        }
    }
}

#[derive(Debug)]
pub struct PwmSysFs {
    chip: usize,
    channel: usize,
}

impl PwmSysFs {
    const fn new(chip: usize, channel: usize) -> PwmSysFs {
        PwmSysFs { chip, channel }
    }

    fn is_exported(&self) -> bool {
        let base_path = self.base_path();
        Path::new(&base_path).exists()
    }

    fn export(&mut self) -> Result<()> {
        write(
            format!("/sys/class/pwm/pwmchip{}/export", self.chip),
            format!("{}", self.channel),
        )?;
        Ok(())
    }

    fn update(&mut self, period: u32, duty_cycle: u32) -> Result<()> {
        let base_path = self.base_path();
        // The PWM cannot be used, if the period and duty_cycle are not initialized
        debug!("Write Duty Cycle: {}", format!("{}/duty_cycle", base_path));
        write(format!("{}/duty_cycle", base_path), "0").ok(); // Ignore any error

        write(format!("{}/period", base_path), format!("{}", period))?;
        write(
            format!("{}/duty_cycle", base_path),
            format!("{}", duty_cycle),
        )?;
        Ok(())
    }

    fn base_path(&self) -> String {
        format!("/sys/class/pwm/pwmchip{}/pwm{}", self.chip, self.channel)
    }

    fn enable(&mut self, enable: bool) -> Result<()> {
        debug!("Setting PWM {} to {}", self.channel, enable);
        let base_path = self.base_path();
        let v = if enable { b"1" } else { b"0" };
        write(format!("{}/enable", base_path), v)?;
        Ok(())
    }
}

#[derive(Debug)]
enum Polarity {
    ActiveHigh,
    ActiveLow,
}

#[derive(Debug)]
enum InitValue {
    Low,
    High,
}

#[derive(Debug)]
pub struct Do {
    init: InitValue,
    polarity: Polarity,
    sysfs: GpioSysFs,
}

impl Do {
    pub const fn new(gpionum: usize) -> Do {
        Do {
            init: InitValue::Low,
            polarity: Polarity::ActiveHigh,
            sysfs: GpioSysFs::new(gpionum),
        }
    }

    pub const fn active_low_init_high(gpionum: usize) -> Do {
        Do {
            init: InitValue::High,
            polarity: Polarity::ActiveLow,
            sysfs: GpioSysFs::new(gpionum),
        }
    }
}

impl IoChannel for Do {
    fn init(&mut self, _chan_number: usize) -> Result<()> {
        let direction = match self.init {
            InitValue::Low => Direction::Out,
            InitValue::High => Direction::OutHigh,
        };

        if !self.sysfs.is_exported() {
            self.sysfs.export()?;
            self.sysfs.set_polarity(&self.polarity)?;
            self.sysfs.set_direction(direction)?;
        }

        self.sysfs.open_value_file(direction)
    }

    fn shutdown(&mut self) -> Result<()> {
        /* we do not want to unexport, other applications may also use it */
        Ok(())
    }
}

impl DigitalOutput for Do {
    fn set(&mut self, val: bool) -> Result<()> {
        self.sysfs.set(val)
    }
}

#[derive(Debug)]
pub struct Di {
    polarity: Polarity,
    sysfs: GpioSysFs,
}

impl Di {
    pub const fn new(gpionum: usize) -> Di {
        Di {
            polarity: Polarity::ActiveHigh,
            sysfs: GpioSysFs::new(gpionum),
        }
    }

    pub const fn active_low(gpionum: usize) -> Di {
        Di {
            polarity: Polarity::ActiveLow,
            sysfs: GpioSysFs::new(gpionum),
        }
    }
}

impl IoChannel for Di {
    fn init(&mut self, _chan_number: usize) -> Result<()> {
        if !self.sysfs.is_exported() {
            self.sysfs.export()?;
            self.sysfs.set_polarity(&self.polarity)?;
            self.sysfs.set_direction(Direction::In)?;
        }

        self.sysfs.open_value_file(Direction::In)
    }

    fn shutdown(&mut self) -> Result<()> {
        Ok(())
    }
}

impl DigitalInput for Di {
    fn get(&mut self) -> Result<bool> {
        self.sysfs.get()
    }
}

#[derive(Debug)]
pub struct PwmInner {
    sysfs: PwmSysFs,
    timebase: u32,
    period: u32,
    duty_cycle: u32,
    update_needed: bool,
    is_gpio: bool,
}

type PwmInnerSync = Arc<Mutex<PwmInner>>;

#[derive(Debug)]
pub struct Pwm {
    inner: PwmInnerSync,
}
#[derive(PartialEq, Eq, Hash)]
struct PwmAddress {
    chip: usize,
    channel: usize,
}

lazy_static! {
    static ref PWM_MAP: Mutex<HashMap<PwmAddress, PwmInnerSync>> = Mutex::new(HashMap::new());
}

impl Pwm {
    pub fn new(chip: usize, channel: usize) -> Pwm {
        let mut map = PWM_MAP.lock().unwrap();
        let pwm_address = PwmAddress { chip, channel };
        match map.get(&pwm_address) {
            None => {
                let inner = Arc::new(Mutex::new(PwmInner {
                    sysfs: PwmSysFs::new(chip, channel),
                    timebase: 800,
                    period: 1,
                    duty_cycle: 1,
                    update_needed: true,
                    is_gpio: true,
                }));

                map.insert(pwm_address, inner.clone());
                Pwm { inner }
            }
            Some(inner) => Pwm {
                inner: Arc::clone(inner),
            },
        }
    }
}

impl IoChannel for Pwm {
    fn init(&mut self, _chan_number: usize) -> Result<()> {
        let mut inner = self.inner.lock().map_err(|_| Error::GenericError)?;
        if !inner.sysfs.is_exported() {
            inner.sysfs.export()?;
        }
        Ok(())
    }

    fn shutdown(&mut self) -> Result<()> {
        /* we do not want to unexport, other applications may also use it */
        Ok(())
    }
}

impl PwmOutput for Pwm {
    fn enable(&mut self, state: bool) -> Result<()> {
        let mut inner = self.inner.lock().map_err(|_| Error::GenericError)?;
        inner.sysfs.enable(state)?;
        Ok(())
    }

    fn setup(&mut self, period: u16, duty_cycle: u16) -> Result<()> {
        let mut inner = self.inner.lock().map_err(|_| Error::GenericError)?;
        inner.period = period as u32;
        inner.duty_cycle = duty_cycle as u32;

        let period_new = inner.period * inner.timebase;
        let duty_cycle_new = inner.duty_cycle * inner.timebase;
        inner.sysfs.update(period_new, duty_cycle_new)?;
        inner.update_needed = true;
        Ok(())
    }

    fn set_timebase(&mut self, timebase: ffi::IoPwmTimebase) -> Result<()> {
        let mut inner = self.inner.lock().map_err(|_| Error::GenericError)?;
        inner.timebase = match timebase {
            ffi::IoPwmTimebase::Ms1 => 1000 * 1000,
            ffi::IoPwmTimebase::Ns800 => 800,
        };
        inner.update_needed = true;
        Ok(())
    }
}

impl DigitalOutput for Pwm {
    fn set(&mut self, val: bool) -> Result<()> {
        let mut inner = self.inner.lock().map_err(|_| Error::GenericError)?;
        if inner.update_needed {
            let period: u32;
            let duty_cycle: u32;

            if inner.is_gpio {
                period = 100;
                duty_cycle = 100;
            } else {
                period = inner.period * inner.timebase;
                duty_cycle = inner.duty_cycle * inner.timebase;
            }
            inner.sysfs.update(period, duty_cycle)?;
            inner.update_needed = false;
        }
        if inner.is_gpio {
            drop(inner);
            self.enable(val)?;
        } else if val {
            drop(inner);
            self.enable(true)?;
        }

        Ok(())
    }
}
