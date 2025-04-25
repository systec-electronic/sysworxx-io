// SPDX-License-Identifier: LGPL-3.0-or-later
// SPDX-FileCopyrightText: 2025 SYS TEC electronic AG <https://www.systec-electronic.com/>

use crate::error::Result;
use crate::ffi;
use crate::{
    AnalogInput, AnalogOutput, CounterInput, DigitalInput, DigitalOutput, IoChannel, PwmOutput,
    TempSensor, Watchdog,
};

#[derive(Debug)]
pub struct Labeled<T> {
    label: &'static str,
    inner: T,
}

impl<T> Labeled<T> {
    pub fn new(label: &'static str, inner: T) -> Labeled<T> {
        Labeled { label, inner }
    }
}

impl<T> IoChannel for Labeled<T>
where
    T: IoChannel,
{
    fn init(&mut self, chan_number: usize) -> Result<()> {
        self.inner.init(chan_number)
    }

    fn shutdown(&mut self) -> Result<()> {
        self.inner.shutdown()
    }

    fn is_dummy(&self) -> bool {
        self.inner.is_dummy()
    }

    fn label(&self) -> Option<&'static str> {
        Some(self.label)
    }
}

impl<T> DigitalOutput for Labeled<T>
where
    T: DigitalOutput,
{
    fn set(&mut self, val: bool) -> Result<()> {
        self.inner.set(val)
    }
}

impl<T> DigitalInput for Labeled<T>
where
    T: DigitalInput,
{
    fn get(&mut self) -> Result<bool> {
        self.inner.get()
    }

    fn register_callback(
        &mut self,
        callback: ffi::IoInputCallback,
        trigger: ffi::IoInputTrigger,
    ) -> Result<()> {
        self.inner.register_callback(callback, trigger)
    }

    fn unregister_callback(&mut self) -> Result<()> {
        self.inner.unregister_callback()
    }
}

impl<T> AnalogInput for Labeled<T>
where
    T: AnalogInput,
{
    fn get(&mut self) -> Result<i64> {
        self.inner.get()
    }
    fn set_mode(&mut self, mode: ffi::IoAnalogMode) -> Result<()> {
        self.inner.set_mode(mode)
    }
}

impl<T> AnalogOutput for Labeled<T>
where
    T: AnalogOutput,
{
    fn set(&mut self, value: i64) -> Result<()> {
        self.inner.set(value)
    }
}

impl<T, U> TempSensor<U> for Labeled<T>
where
    T: TempSensor<U>,
{
    fn get(&mut self) -> Result<U> {
        self.inner.get()
    }
    fn set_mode(&mut self, mode: ffi::IoTmpMode, sensor_type: ffi::IoTmpSensorType) -> Result<()> {
        self.inner.set_mode(mode, sensor_type)
    }
}

impl<T> Watchdog for Labeled<T>
where
    T: Watchdog,
{
    fn enable(&mut self, monitor: bool) -> Result<()> {
        self.inner.enable(monitor)
    }
    fn service(&mut self) -> Result<()> {
        self.inner.service()
    }
}

impl<T> CounterInput for Labeled<T>
where
    T: CounterInput,
{
    fn enable(&mut self, state: bool) -> Result<()> {
        self.inner.enable(state)
    }
    fn setup(
        &mut self,
        mode: ffi::IoCntMode,
        trigger: ffi::IoCntTrigger,
        direction: ffi::IoCntDirection,
    ) -> Result<()> {
        self.inner.setup(mode, trigger, direction)
    }
    fn set_preload(&mut self, preload: i32) -> Result<()> {
        self.inner.set_preload(preload)
    }
    fn get(&mut self) -> Result<i32> {
        self.inner.get()
    }
}

impl<T> PwmOutput for Labeled<T>
where
    T: PwmOutput,
{
    fn enable(&mut self, state: bool) -> Result<()> {
        self.inner.enable(state)
    }
    fn setup(&mut self, period: u16, duty_cycle: u16) -> Result<()> {
        self.inner.setup(period, duty_cycle)
    }
    fn set_timebase(&mut self, timebase: ffi::IoPwmTimebase) -> Result<()> {
        self.inner.set_timebase(timebase)
    }
}
