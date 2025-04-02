// SPDX-License-Identifier: LGPL-3.0-or-later
//
// (c) SYSTEC electronic AG, D-08468 Heinsdorfergrund, Am Windrad 2
//     www.systec-electronic.com

use ini::Ini;

use crate::error::Result;
use crate::ffi;
use crate::{AnalogInput, DigitalOutput, IoChannel, TempSensor};

#[derive(Debug)]
pub struct DoOnly<T: DigitalOutput> {
    inner: T,
    expected: bool,
}

impl<T: DigitalOutput> DoOnly<T> {
    pub fn new(expected: bool, inner: T) -> DoOnly<T> {
        DoOnly { inner, expected }
    }
}

impl<T: DigitalOutput> IoChannel for DoOnly<T> {
    fn init(&mut self, chan_number: usize) -> Result<()> {
        self.inner.init(chan_number)
    }

    fn shutdown(&mut self) -> Result<()> {
        self.inner.shutdown()
    }
}

impl<T: DigitalOutput> DigitalOutput for DoOnly<T> {
    fn set(&mut self, val: bool) -> Result<()> {
        if val == self.expected {
            self.inner.set(val)
        } else {
            Ok(())
        }
    }
}

/// Struct which wraps an analog input and makes it configurable via two digital outputs
#[derive(Debug)]
pub struct AiSwitch<T: AnalogInput, U: DigitalOutput> {
    inner: T,
    sw_voltage: U,
    sw_current: U,
}

impl<T: AnalogInput, U: DigitalOutput> AiSwitch<T, U> {
    pub fn new(inner: T, sw_voltage: U, sw_current: U) -> AiSwitch<T, U> {
        let mut sw_voltage = sw_voltage;
        let mut sw_current = sw_current;

        sw_voltage.init(0).unwrap();
        sw_current.init(1).unwrap();

        AiSwitch {
            inner,
            sw_voltage,
            sw_current,
        }
    }
}

impl<T: AnalogInput, U: DigitalOutput> IoChannel for AiSwitch<T, U> {
    fn init(&mut self, chan_number: usize) -> Result<()> {
        self.inner.init(chan_number)
    }

    fn shutdown(&mut self) -> Result<()> {
        self.inner.shutdown()
    }
}

impl<T: AnalogInput, U: DigitalOutput> AnalogInput for AiSwitch<T, U> {
    fn get(&mut self) -> Result<i64> {
        self.inner.get()
    }

    fn set_mode(&mut self, mode: ffi::IoAnalogMode) -> Result<()> {
        match mode {
            ffi::IoAnalogMode::Voltage => {
                self.sw_current.set(false)?;
                self.sw_voltage.set(true)?;
            }
            ffi::IoAnalogMode::Current => {
                self.sw_voltage.set(false)?;
                self.sw_current.set(true)?;
            }
        }
        Ok(())
    }
}

/// Struct which wraps an analog input and makes it configurable via two digital outputs
#[derive(Debug)]
pub struct AiIniCalib<T: AnalogInput> {
    inner: T,
    mode: ffi::IoAnalogMode,
    voltage_gain: f64,
    voltage_offset: f64,
    current_gain: f64,
    current_offset: f64,
    shifter: Shifter,
}

impl<T: AnalogInput> AiIniCalib<T> {
    pub fn new_shift(ini: &Ini, section: &str, inner: T, shifter: Shifter) -> AiIniCalib<T> {
        let get_val = |entry, default| {
            ini.get_from(Some(section), entry)
                .and_then(|value| value.parse::<f64>().ok())
                .unwrap_or(default)
        };

        let mut voltage_gain = get_val("VoltageGain", 1.0);
        let voltage_offset = get_val("VoltageOffset", 0.0);
        let mut current_gain = get_val("CurrentGain", 1.0);
        let current_offset = get_val("CurrentOffset", 0.0);

        // if gains are above 2.0,
        // assume fixed-point gains where decimal point is at 10^5
        if voltage_gain > 2.0 {
            voltage_gain /= 10000.0;
        }
        if current_gain > 2.0 {
            current_gain /= 10000.0;
        }

        AiIniCalib {
            inner,
            voltage_offset,
            voltage_gain,
            current_offset,
            current_gain,
            mode: ffi::IoAnalogMode::Voltage,
            shifter,
        }
    }

    pub fn new(ini: &Ini, section: &str, inner: T) -> AiIniCalib<T> {
        AiIniCalib::new_shift(ini, section, inner, Shifter::new(Shift::Up(0)))
    }
}

impl<T: AnalogInput> IoChannel for AiIniCalib<T> {
    fn init(&mut self, chan_number: usize) -> Result<()> {
        self.inner.init(chan_number)
    }

    fn shutdown(&mut self) -> Result<()> {
        self.inner.shutdown()
    }
}

impl<T: AnalogInput> AnalogInput for AiIniCalib<T> {
    fn get(&mut self) -> Result<i64> {
        let value = self.inner.get()?;
        let value = self.shifter.shift(value) as f64;

        match self.mode {
            ffi::IoAnalogMode::Voltage => {
                Ok(((value * self.voltage_gain) + self.voltage_offset) as i64)
            }
            ffi::IoAnalogMode::Current => {
                Ok(((value * self.current_gain) + self.current_offset) as i64)
            }
        }
    }

    fn set_mode(&mut self, mode: ffi::IoAnalogMode) -> Result<()> {
        self.mode = mode;
        self.inner.set_mode(mode)
    }
}

/// Struct which wraps an analog input for calibration
#[derive(Debug)]
pub struct TmpRtdIniCalib<T: TempSensor<f64>> {
    inner: T,
    mode: ffi::IoTmpMode,
    fourwire_gain: f64,
    fourwire_offset: f64,
    threewire_gain: f64,
    threewire_offset: f64,
}

impl<T: TempSensor<f64>> TmpRtdIniCalib<T> {
    pub fn new(ini: &Ini, section: &str, inner: T) -> TmpRtdIniCalib<T> {
        let get_val = |entry, default| {
            ini.get_from(Some(section), entry)
                .and_then(|value| value.parse::<f64>().ok())
                .unwrap_or(default)
        };

        let fourwire_gain = get_val("FourWireGain", 1.0);
        let fourwire_offset = get_val("FourWireOffset", 0.0);
        let threewire_gain = get_val("ThreeWireGain", 1.0);
        let threewire_offset = get_val("ThreeWireOffset", 0.0);

        TmpRtdIniCalib {
            inner,
            mode: ffi::IoTmpMode::RtdFourWire,
            fourwire_gain,
            fourwire_offset,
            threewire_gain,
            threewire_offset,
        }
    }
}

impl<T: TempSensor<f64>> IoChannel for TmpRtdIniCalib<T> {
    fn init(&mut self, chan_number: usize) -> Result<()> {
        self.inner.init(chan_number)
    }

    fn shutdown(&mut self) -> Result<()> {
        self.inner.shutdown()
    }
}

impl<T: TempSensor<f64>> TempSensor<f64> for TmpRtdIniCalib<T> {
    fn get(&mut self) -> Result<f64> {
        let value = self.inner.get()?;

        use ffi::IoTmpMode::*;
        let (gain, offset) = match self.mode {
            RtdTwoWire | RtdFourWire => (self.fourwire_gain, self.fourwire_offset),
            RtdThreeWire => (self.threewire_gain, self.threewire_offset),
        };

        Ok((value * gain) + offset)
    }

    fn set_mode(&mut self, mode: ffi::IoTmpMode, _sensor_type: ffi::IoTmpSensorType) -> Result<()> {
        use ffi::IoTmpMode::*;

        match mode {
            RtdTwoWire | RtdThreeWire | RtdFourWire => self.mode = mode,
        }

        Ok(())
    }
}

/// Helper structure which clips a value to a specfied range
#[derive(Debug, Copy, Clone)]
pub struct Clip<T: Ord + Copy> {
    min: T,
    max: T,
}

impl<T: Ord + Copy> Clip<T> {
    pub fn new(min: T, max: T) -> Clip<T> {
        Clip { min, max }
    }

    pub fn clip(&self, value: T) -> T {
        if value < self.min {
            self.min
        } else if value > self.max {
            self.max
        } else {
            value
        }
    }
}

/// Enum which specifies number and direction of bits to shift
#[derive(Debug, Copy, Clone)]
pub enum Shift {
    Down(u8),
    Up(u8),
}

/// Helper struct which can be used to shift a value by a number of bits
#[derive(Debug, Copy, Clone)]
pub struct Shifter {
    shift: Shift,
}

impl Shifter {
    pub const fn new(shift: Shift) -> Shifter {
        Shifter { shift }
    }

    pub fn shift(self, value: i64) -> i64 {
        match self.shift {
            Shift::Down(shift) => value >> shift,
            Shift::Up(shift) => value << shift,
        }
    }
}

/// Tuple based alternative to HashMap
/// We only use very small channel numbers around 2 to 10. Using something more lightweight than
/// HashMaps should improve performance.
#[derive(Default)]
pub struct PairMap<K, V>
where
    K: Eq + Copy,
{
    pairs: Vec<(K, V)>,
}

impl<K, V> PairMap<K, V>
where
    K: Eq + Copy,
{
    pub fn set(&mut self, k: K, v: V) {
        if let Some(value) = self.get_mut(k) {
            *value = v;
        } else {
            self.pairs.push((k, v))
        }
    }

    pub fn get(&self, k: K) -> Option<&V> {
        self.pairs
            .iter()
            .find(|pair| pair.0 == k)
            .map(|pair| &pair.1)
    }

    pub fn get_mut(&mut self, k: K) -> Option<&mut V> {
        self.pairs
            .iter_mut()
            .find(|pair| pair.0 == k)
            .map(|pair| &mut pair.1)
    }

    pub fn contains(&self, k: K) -> bool {
        self.get(k).is_some()
    }

    pub fn iter(&self) -> impl Iterator<Item = &(K, V)> {
        self.pairs.iter()
    }

    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut (K, V)> {
        self.pairs.iter_mut()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn clip_test() {
        let clip = Clip::new(2, 8);

        assert_eq!(2, clip.clip(1));
        assert_eq!(2, clip.clip(2));
        assert_eq!(3, clip.clip(3));

        assert_eq!(7, clip.clip(7));
        assert_eq!(8, clip.clip(8));
        assert_eq!(8, clip.clip(9));
    }

    #[test]
    fn test_shifter_test() {
        let shifter = Shifter::new(Shift::Down(3));
        assert_eq!(1, shifter.shift(8));
        let shifter = Shifter::new(Shift::Up(3));
        assert_eq!(8, shifter.shift(1));
    }

    #[test]
    fn pairmap_test() {
        let mut pm: PairMap<u16, u64> = PairMap::default();
        pm.set(0, 123);
        pm.set(1, 456);
        pm.set(2, 789);

        assert_eq!(pm.get(0), Some(&123));
        assert_eq!(pm.get(1), Some(&456));
        assert_eq!(pm.get(2), Some(&789));
        assert_eq!(pm.get(3), None);

        let v = pm.get_mut(0).unwrap();
        *v = 321;
        let v = pm.get_mut(1).unwrap();
        *v = 654;
        let v = pm.get_mut(2).unwrap();
        *v = 987;

        assert_eq!(pm.get(0), Some(&321));
        assert_eq!(pm.get(1), Some(&654));
        assert_eq!(pm.get(2), Some(&987));
        assert_eq!(pm.get(3), None);

        let mut iter = pm.iter();
        assert_eq!(iter.next(), Some(&(0, 321)));
        assert_eq!(iter.next(), Some(&(1, 654)));
        assert_eq!(iter.next(), Some(&(2, 987)));
        assert_eq!(iter.next(), None);
        drop(iter);

        let mut iter_mut = pm.iter_mut();
        *iter_mut.next().unwrap() = (0, 111);
        *iter_mut.next().unwrap() = (1, 222);
        *iter_mut.next().unwrap() = (2, 333);
        assert_eq!(iter_mut.next(), None);
        drop(iter_mut);

        assert_eq!(pm.get(0), Some(&111));
        assert_eq!(pm.get(1), Some(&222));
        assert_eq!(pm.get(2), Some(&333));
    }

    #[test]
    fn pairmap_test_struct() {
        #[derive(Default, Debug, PartialEq, Eq)]
        struct Foo(u32);
        let mut pm: PairMap<u16, Foo> = PairMap::default();

        pm.set(0, Foo(123));
        pm.set(1, Foo(456));
        pm.set(2, Foo(789));

        assert_eq!(pm.get(0), Some(&Foo(123)));
        assert_eq!(pm.get(1), Some(&Foo(456)));
        assert_eq!(pm.get(2), Some(&Foo(789)));

        let v = pm.get_mut(0).unwrap();
        *v = Foo(321);
        let v = pm.get_mut(1).unwrap();
        *v = Foo(654);
        let v = pm.get_mut(2).unwrap();
        *v = Foo(987);

        assert_eq!(pm.get(0), Some(&Foo(321)));
        assert_eq!(pm.get(1), Some(&Foo(654)));
        assert_eq!(pm.get(2), Some(&Foo(987)));
    }
}
