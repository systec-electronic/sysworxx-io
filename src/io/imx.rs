// SPDX-License-Identifier: LGPL-3.0-or-later
//
// (c) SYSTEC electronic AG, D-08468 Heinsdorfergrund, Am Windrad 2
//     www.systec-electronic.com

use std::fs;
use std::path;

use crate::error::{Error, Result};
use crate::ffi;
use crate::{CounterInput, DigitalInput, IoChannel};

#[derive(Debug)]
pub struct Counter {
    path: &'static str,
    mode: ffi::IoCntMode,
    trigger: ffi::IoCntTrigger,
    dir: ffi::IoCntDirection,
    preload: i32,
    input: Box<dyn DigitalInput>,
    direction_pin: Option<Box<dyn DigitalInput>>,
}

impl Counter {
    pub fn new(
        path: &'static str,
        input: Box<dyn DigitalInput>,
        direction_pin: Option<Box<dyn DigitalInput>>,
    ) -> Counter {
        Counter {
            path,
            mode: ffi::IoCntMode::Counter,
            trigger: ffi::IoCntTrigger::RisingEdge,
            dir: ffi::IoCntDirection::Up,
            preload: 0,
            input,
            direction_pin,
        }
    }
}

impl IoChannel for Counter {
    fn init(&mut self, _chan_number: usize) -> Result<()> {
        if !path::Path::new(self.path).exists() {
            return Err(Error::generic_access_error());
        }

        self.input.init(0)?;
        if let Some(ref mut direction_pin) = self.direction_pin {
            direction_pin.init(0)?;
        }

        Ok(())
    }

    fn shutdown(&mut self) -> Result<()> {
        self.input.shutdown()?;
        if let Some(ref mut direction_pin) = self.direction_pin {
            direction_pin.shutdown()?;
        }

        Ok(())
    }
}

impl CounterInput for Counter {
    fn enable(&mut self, state: bool) -> Result<()> {
        let path_enable = format!("{}/enable", self.path);

        if state {
            fs::write(&path_enable, b"0")?;

            use ffi::IoCntMode::*;
            let attr_mode = match self.mode {
                Counter => "cnt",
                ABEncoder => "quad",
            };

            use ffi::IoCntTrigger::*;
            let attr_trigger = match self.trigger {
                RisingEdge => "rise",
                FallingEdge | AnyEdge => "fall",
            };

            use ffi::IoCntDirection::*;
            let direction = match self.dir {
                Up => b"0",
                Down => b"1",
            };

            let path_mode = format!("{}/mode", self.path);
            let path_trigger = format!("{}/trigger", self.path);
            let path_direction = format!("{}/direction", self.path);

            fs::write(path_mode, attr_mode)?;
            fs::write(path_trigger, attr_trigger)?;
            fs::write(path_direction, direction)?;

            fs::write(&path_enable, b"1")?;
        } else {
            fs::write(&path_enable, b"0")?;
        }

        Ok(())
    }

    fn setup(
        &mut self,
        mode: ffi::IoCntMode,
        trigger: ffi::IoCntTrigger,
        dir: ffi::IoCntDirection,
    ) -> Result<()> {
        self.mode = mode;
        self.trigger = trigger;
        self.dir = dir;
        Ok(())
    }

    fn set_preload(&mut self, preload: i32) -> Result<()> {
        self.preload = preload;
        Ok(())
    }

    fn get(&mut self) -> Result<i32> {
        let path = format!("{}/value", self.path);
        let value = fs::read_to_string(path)?;
        let mut value = value
            .trim()
            .parse::<i32>()
            .map_err(|_| Error::generic_access_error())?;

        use ffi::IoCntTrigger::*;
        match self.trigger {
            RisingEdge => {}
            FallingEdge => {}
            AnyEdge => {
                value *= 2;

                let input = self.input.get().unwrap_or(false);

                let direction = match &mut self.direction_pin {
                    None => false,
                    Some(direction_pin) => direction_pin.get().unwrap_or(false),
                };

                use ffi::IoCntDirection::*;
                let inc = match self.dir {
                    Up => 1,
                    Down => -1,
                };

                if input {
                    if !direction {
                        value += inc;
                    } else {
                        value -= inc;
                    }
                }
            }
        }

        value += self.preload;

        Ok(value)
    }
}
