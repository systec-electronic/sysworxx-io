// SPDX-License-Identifier: LGPL-3.0-or-later
// SPDX-FileCopyrightText: 2025 SYS TEC electronic AG <https://www.systec-electronic.com/>

use std::fs;
use std::path;

use crate::error::{Error, Result};
use crate::ffi;
use crate::{CounterInput, DigitalInput, IoChannel};

// Differences to IMX counter:
// By using function setting "pulse-direction" (meaning: counter)
// the direction pin dictates, if the value is count up or down.
// LOW = down; HIGH = up;
// The direction pin does this automatically via kernel driver.
//
// Additionally, there are function settings to completely ignore
// the direction pin. These are called "increase" and "decrease".
//
// Regardless which function is used, there seems to be no way to change the
// trigger edge for the counter - it changes on BOTH edges at all times.

#[derive(Debug)]
pub struct Counter {
    path: &'static str,
    function: ffi::IoCntMode,
    trigger: ffi::IoCntTrigger,
    dir: ffi::IoCntDirection,
    preload: i32,
    input: Box<dyn DigitalInput>,
}

impl Counter {
    pub fn new(path: &'static str, input: Box<dyn DigitalInput>) -> Counter {
        Counter {
            path,
            function: ffi::IoCntMode::Counter,
            trigger: ffi::IoCntTrigger::RisingEdge,
            dir: ffi::IoCntDirection::Up,
            preload: 0,
            input,
        }
    }
}

impl IoChannel for Counter {
    fn init(&mut self, _chan_number: usize) -> Result<()> {
        if !path::Path::new(self.path).exists() {
            return Err(Error::generic_access_error());
        }

        self.input.init(0)?;

        Ok(())
    }

    fn shutdown(&mut self) -> Result<()> {
        self.input.shutdown()?;

        Ok(())
    }
}

impl CounterInput for Counter {
    fn enable(&mut self, state: bool) -> Result<()> {
        let path_enable = format!("{}/enable", self.path);

        if state {
            fs::write(&path_enable, b"0")?;

            use ffi::IoCntMode::*;
            let attr_function = match self.function {
                // If in counter mode, set to increase or decrease
                // depending on the given direction. This "frees" the
                // direction pin and lets us switch in SW only.
                Counter => {
                    use ffi::IoCntDirection::*;
                    match self.dir {
                        Up => "increase",
                        Down => "decrease",
                    }
                }
                ABEncoder => "quadrature x4",
            };

            let path_function = format!("{}/function", self.path);
            fs::write(path_function, attr_function)?;

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
        // not supported by the peripheral / the Kernel driver
        if matches!(
            trigger,
            ffi::IoCntTrigger::FallingEdge | ffi::IoCntTrigger::RisingEdge
        ) {
            return Err(Error::NotImplemented);
        }

        self.function = mode;
        self.trigger = trigger;
        self.dir = dir;
        Ok(())
    }

    fn set_preload(&mut self, preload: i32) -> Result<()> {
        self.preload = preload;
        Ok(())
    }

    fn get(&mut self) -> Result<i32> {
        let path = format!("{}/count", self.path);
        let value = fs::read_to_string(path)?;
        let mut value = value
            .trim()
            .parse::<i32>()
            .map_err(|_| Error::generic_access_error())?;

        value += self.preload;

        Ok(value)
    }
}
