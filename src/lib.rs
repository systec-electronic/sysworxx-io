// SPDX-License-Identifier: LGPL-3.0-or-later
// SPDX-FileCopyrightText: 2025 SYS TEC electronic AG <https://www.systec-electronic.com/>

#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate log;

#[cfg(test)]
#[macro_use]
extern crate float_cmp;

#[macro_use]
pub mod macros;
pub mod convert;
pub mod definition;
pub mod error;
pub mod ffi;
pub mod hw_rev;
pub mod io;
pub mod labeled;
pub mod periodic;
pub mod shm;
pub mod signal;

use crate::error::{Error, Result};
use std::{fmt, fs::File, io::Write};

pub trait IoChannel {
    fn init(&mut self, chan_number: usize) -> Result<()>;
    fn shutdown(&mut self) -> Result<()>;
    fn is_dummy(&self) -> bool {
        false
    }
    fn label(&self) -> Option<&'static str> {
        None
    }
}

pub trait DigitalOutput: fmt::Debug + Send + IoChannel {
    fn set(&mut self, val: bool) -> Result<()>;
}

pub trait DigitalInput: fmt::Debug + Send + IoChannel {
    fn get(&mut self) -> Result<bool>;

    fn register_callback(
        &mut self,
        _callback: ffi::IoInputCallback,
        _trigger: ffi::IoInputTrigger,
    ) -> Result<()> {
        Err(Error::NotImplemented)
    }

    fn unregister_callback(&mut self) -> Result<()> {
        Err(Error::NotImplemented)
    }
}

pub trait AnalogInput: fmt::Debug + Send + IoChannel {
    fn get(&mut self) -> Result<i64>;
    fn set_mode(&mut self, _mode: ffi::IoAnalogMode) -> Result<()> {
        Err(Error::NotImplemented)
    }
}

pub trait AnalogOutput: fmt::Debug + Send + IoChannel {
    fn set(&mut self, value: i64) -> Result<()>;
}

pub trait TempSensor<T>: fmt::Debug + Send + IoChannel {
    fn get(&mut self) -> Result<T>;
    fn set_mode(
        &mut self,
        _mode: ffi::IoTmpMode,
        _sensor_type: ffi::IoTmpSensorType,
    ) -> Result<()> {
        Err(Error::NotImplemented)
    }
}

pub trait Watchdog: fmt::Debug + Send {
    fn enable(&mut self, monitor: bool) -> Result<()>;
    fn service(&mut self) -> Result<()>;
}

pub trait CounterInput: fmt::Debug + Send + IoChannel {
    fn enable(&mut self, state: bool) -> Result<()>;
    fn setup(
        &mut self,
        mode: ffi::IoCntMode,
        trigger: ffi::IoCntTrigger,
        direction: ffi::IoCntDirection,
    ) -> Result<()>;
    fn set_preload(&mut self, preload: i32) -> Result<()>;
    fn get(&mut self) -> Result<i32>;
}

pub trait PwmOutput: fmt::Debug + Send + IoChannel {
    fn enable(&mut self, state: bool) -> Result<()>;
    fn setup(&mut self, period: u16, duty_cycle: u16) -> Result<()>;
    fn set_timebase(&mut self, timebase: ffi::IoPwmTimebase) -> Result<()>;
}

#[derive(Debug)]
pub struct Io {
    watchdog: Box<dyn Watchdog>,
    run_led: Box<dyn DigitalOutput>,
    err_led: Box<dyn DigitalOutput>,
    run_switch: Box<dyn DigitalInput>,
    config_switch: Box<dyn DigitalInput>,
    outputs: Vec<Box<dyn DigitalOutput>>,
    inputs: Vec<Box<dyn DigitalInput>>,
    analog_inputs: Vec<Box<dyn AnalogInput>>,
    analog_outputs: Vec<Box<dyn AnalogOutput>>,
    temp_sensors: Vec<Box<dyn TempSensor<f64>>>,
    counter_input: Vec<Box<dyn CounterInput>>,
    relay_offset: Option<u8>,
    pwm_outputs: Vec<Box<dyn PwmOutput>>,
}

pub struct IoChannelInfo<'a> {
    pub inputs: &'a Vec<Box<dyn DigitalInput>>,
    pub outputs: &'a Vec<Box<dyn DigitalOutput>>,
    pub run_led: &'a dyn DigitalOutput,
    pub err_led: &'a dyn DigitalOutput,
    pub run_switch: &'a dyn DigitalInput,
    pub config_switch: &'a dyn DigitalInput,
    pub analog_inputs: &'a Vec<Box<dyn AnalogInput>>,
    pub temp_sensors: &'a Vec<Box<dyn TempSensor<f64>>>,
    pub counter_input: &'a Vec<Box<dyn CounterInput>>,
}

impl Io {
    pub fn init(&mut self) -> Result<()> {
        self.run_led.init(0)?;
        self.err_led.init(0)?;
        self.run_switch.init(0)?;
        self.config_switch.init(0)?;

        channels_init!(self.outputs);
        channels_init!(self.inputs);
        channels_init!(self.analog_inputs);
        channels_init!(self.analog_outputs);
        channels_init!(self.temp_sensors);
        channels_init!(self.counter_input);
        channels_init!(self.pwm_outputs);
        Ok(())
    }

    pub fn shutdown(&mut self) -> Result<()> {
        self.run_led.shutdown()?;
        self.err_led.shutdown()?;
        self.run_switch.shutdown()?;
        self.config_switch.shutdown()?;

        channels_shutdown!(self.outputs);
        channels_shutdown!(self.inputs);
        channels_shutdown!(self.analog_inputs);
        channels_shutdown!(self.analog_outputs);
        channels_shutdown!(self.temp_sensors);
        channels_shutdown!(self.counter_input);
        Ok(())
    }

    pub fn get_ticks(&mut self) -> Result<u32> {
        lazy_static! {
            static ref START: std::time::Instant = std::time::Instant::now();
        }

        Ok(START.elapsed().as_millis() as u32)
    }

    pub fn watchdog_enable(&mut self, monitor: bool) -> Result<()> {
        self.watchdog.enable(monitor)
    }

    pub fn watchdog_service(&mut self) -> Result<()> {
        self.watchdog.service()
    }

    pub fn get_channel_info(&self) -> IoChannelInfo<'_> {
        IoChannelInfo {
            inputs: &self.inputs,
            outputs: &self.outputs,
            run_led: self.run_led.as_ref(),
            err_led: self.err_led.as_ref(),
            run_switch: self.run_switch.as_ref(),
            config_switch: self.config_switch.as_ref(),
            analog_inputs: &self.analog_inputs,
            temp_sensors: &self.temp_sensors,
            counter_input: &self.counter_input,
        }
    }

    pub fn get_hardware_info(&mut self, hwinfo: &mut ffi::IoHwInfo) -> Result<()> {
        hwinfo.m_uPcbRevision = hw_rev::get_hardware_revision().unwrap_or(0xff);
        hwinfo.m_uDiChannels = self.inputs.len() as u8;
        hwinfo.m_uDoChannels = self.outputs.len() as u8;
        hwinfo.m_uAiChannels = self.analog_inputs.len() as u8;
        hwinfo.m_uAoChannels = self.analog_outputs.len() as u8;
        hwinfo.m_uTmpChannels = self.temp_sensors.len() as u8;
        hwinfo.m_uCntChannels = self.counter_input.len() as u8;
        match self.relay_offset {
            None => {
                hwinfo.m_uLegacyRelayOffset = 0;
                hwinfo.m_uLegacyRelayChannels = 0;
            }
            Some(relay_offset) => {
                hwinfo.m_uLegacyRelayOffset = relay_offset;
                hwinfo.m_uLegacyRelayChannels = self
                    .outputs
                    .iter()
                    .skip(relay_offset as usize)
                    .take_while(|x| !x.is_dummy())
                    .count() as u8;
            }
        }
        hwinfo.m_uPwmChannels = self.pwm_outputs.len() as u8;
        hwinfo.m_uLegacyDoChannels = self
            .outputs
            .iter()
            .take(hwinfo.m_uLegacyRelayOffset as usize)
            .filter(|x| !x.is_dummy())
            .count() as u8;
        hwinfo.m_uLegacyDiChannels = self
            .inputs
            .iter()
            .take(32)
            .filter(|x| !x.is_dummy())
            .count() as u8;
        Ok(())
    }

    // FIXME: actually the reference does not need to be mutable
    //        but for now make it to avoid compiler warnings
    pub fn write_json_info(&mut self, path: &str) -> Result<()> {
        let mut obj = json::object! {
            outputs: { },
            inputs: { },
            watchdog: { },
            run_led: { },
            run_switch: { },
            config_switch: { },
            analog_inputs: { },
            analog_outputs: { },
            temp_sensors: { },
            counter_inputs: { },
            pwm_outputs: { },
        };

        fn add_to_json<T: IoChannel + ?Sized>(
            json: &mut json::JsonValue,
            key: &str,
            channels: &[Box<T>],
        ) -> Result<()> {
            for (i, channel) in channels.iter().enumerate() {
                if let Some(label) = channel.label() {
                    json[key]
                        .insert(&i.to_string(), label)
                        .map_err(|_| Error::GenericError)?;
                }
            }

            Ok(())
        }

        add_to_json(&mut obj, "outputs", &self.outputs)?;
        add_to_json(&mut obj, "inputs", &self.inputs)?;
        add_to_json(&mut obj, "analog_inputs", &self.analog_inputs)?;
        add_to_json(&mut obj, "temp_sensors", &self.temp_sensors)?;
        add_to_json(&mut obj, "counter_inputs", &self.counter_input)?;
        add_to_json(&mut obj, "analog_outputs", &self.analog_outputs)?;
        let obj_str = obj.dump();
        let mut file = File::create(path).expect("Error creating file to write information!");
        file.write(obj_str.as_bytes())
            .map_err(|_| Error::GenericError)?;

        Ok(())
    }

    pub fn set_run_led(&mut self, value: bool) -> Result<()> {
        self.run_led.set(value)
    }

    pub fn set_err_led(&mut self, value: bool) -> Result<()> {
        self.err_led.set(value)
    }

    pub fn get_run_switch(&mut self) -> Result<bool> {
        self.run_switch.get()
    }

    pub fn get_config_switch(&mut self) -> Result<bool> {
        self.config_switch.get()
    }

    pub fn output_set(&mut self, channel: usize, value: bool) -> Result<()> {
        self.outputs
            .get_mut(channel)
            .ok_or(Error::InvalidChannel)?
            .set(value)
    }

    pub fn input_get(&mut self, channel: usize) -> Result<bool> {
        self.inputs
            .get_mut(channel)
            .ok_or(Error::InvalidChannel)?
            .get()
    }

    pub fn input_register_callback(
        &mut self,
        channel: usize,
        callback: ffi::IoInputCallback,
        trigger: ffi::IoInputTrigger,
    ) -> Result<()> {
        self.inputs
            .get_mut(channel)
            .ok_or(Error::InvalidChannel)?
            .register_callback(callback, trigger)
    }

    pub fn input_unregister_callback(&mut self, channel: usize) -> Result<()> {
        self.inputs
            .get_mut(channel)
            .ok_or(Error::InvalidChannel)?
            .unregister_callback()
    }

    pub fn analog_input_get(&mut self, channel: usize) -> Result<i64> {
        self.analog_inputs
            .get_mut(channel)
            .ok_or(Error::InvalidChannel)?
            .get()
    }

    pub fn analog_mode_set(&mut self, channel: usize, mode: ffi::IoAnalogMode) -> Result<()> {
        self.analog_inputs
            .get_mut(channel)
            .ok_or(Error::InvalidChannel)?
            .set_mode(mode)
    }

    pub fn analog_output_set(&mut self, channel: usize, value: i64) -> Result<()> {
        self.analog_outputs
            .get_mut(channel)
            .ok_or(Error::InvalidChannel)?
            .set(value)
    }

    pub fn tmp_set_mode(
        &mut self,
        channel: usize,
        mode: ffi::IoTmpMode,
        sensor_type: ffi::IoTmpSensorType,
    ) -> Result<()> {
        self.temp_sensors
            .get_mut(channel)
            .ok_or(Error::InvalidChannel)?
            .set_mode(mode, sensor_type)
    }

    pub fn tmp_input_get(&mut self, channel: usize) -> Result<f64> {
        self.temp_sensors
            .get_mut(channel)
            .ok_or(Error::InvalidChannel)?
            .get()
    }

    pub fn cnt_enable(&mut self, channel: usize, state: bool) -> Result<()> {
        self.counter_input
            .get_mut(channel)
            .ok_or(Error::InvalidChannel)?
            .enable(state)
    }

    pub fn cnt_setup(
        &mut self,
        channel: usize,
        mode: ffi::IoCntMode,
        trigger: ffi::IoCntTrigger,
        direction: ffi::IoCntDirection,
    ) -> Result<()> {
        self.counter_input
            .get_mut(channel)
            .ok_or(Error::InvalidChannel)?
            .setup(mode, trigger, direction)
    }

    pub fn cnt_set_preload(&mut self, channel: usize, preload: i32) -> Result<()> {
        self.counter_input
            .get_mut(channel)
            .ok_or(Error::InvalidChannel)?
            .set_preload(preload)
    }

    pub fn cnt_get(&mut self, channel: usize) -> Result<i32> {
        self.counter_input
            .get_mut(channel)
            .ok_or(Error::InvalidChannel)?
            .get()
    }

    pub fn pwm_enable(&mut self, channel: usize, state: bool) -> Result<()> {
        self.pwm_outputs
            .get_mut(channel)
            .ok_or(Error::InvalidChannel)?
            .enable(state)
    }

    pub fn pwm_setup(&mut self, channel: usize, period: u16, duty_cycle: u16) -> Result<()> {
        self.pwm_outputs
            .get_mut(channel)
            .ok_or(Error::InvalidChannel)?
            .setup(period, duty_cycle)
    }

    pub fn pwm_set_timebase(&mut self, channel: usize, timebase: ffi::IoPwmTimebase) -> Result<()> {
        self.pwm_outputs
            .get_mut(channel)
            .ok_or(Error::InvalidChannel)?
            .set_timebase(timebase)
    }
}
