// SPDX-License-Identifier: LGPL-3.0-or-later
// SPDX-FileCopyrightText: 2025 SYS TEC electronic AG <https://www.systec-electronic.com/>

use crate::error::*;
use crate::{DigitalInput, DigitalOutput, IoChannel, TempSensor, Watchdog};

#[derive(Debug)]
enum Behaviour {
    NotImplemented,
    AlwaysActive,
}

#[derive(Debug)]
pub struct Output {}

impl Output {
    pub fn not_implemented() -> Output {
        Output {}
    }
}

impl IoChannel for Output {
    fn init(&mut self, _chan_number: usize) -> Result<()> {
        Ok(())
    }
    fn shutdown(&mut self) -> Result<()> {
        Ok(())
    }
    fn is_dummy(&self) -> bool {
        true
    }
}

impl DigitalOutput for Output {
    fn set(&mut self, _val: bool) -> Result<()> {
        Err(Error::NotImplemented)
    }
}

#[derive(Debug)]
pub struct Input {
    behaviour: Behaviour,
}

impl Input {
    pub fn not_implemented() -> Input {
        Input {
            behaviour: Behaviour::NotImplemented,
        }
    }

    pub fn always_active() -> Input {
        Input {
            behaviour: Behaviour::AlwaysActive,
        }
    }
}

impl IoChannel for Input {
    fn init(&mut self, _chan_number: usize) -> Result<()> {
        Ok(())
    }
    fn shutdown(&mut self) -> Result<()> {
        Ok(())
    }
    fn is_dummy(&self) -> bool {
        true
    }
}

impl DigitalInput for Input {
    fn get(&mut self) -> Result<bool> {
        match self.behaviour {
            Behaviour::NotImplemented => Err(Error::NotImplemented),
            Behaviour::AlwaysActive => Ok(true),
        }
    }
}

#[derive(Debug, Default)]
pub struct Temp {}

impl Temp {
    pub fn new() -> Temp {
        Temp::default()
    }
}

impl IoChannel for Temp {
    fn init(&mut self, _chan_number: usize) -> Result<()> {
        Ok(())
    }

    fn shutdown(&mut self) -> Result<()> {
        Ok(())
    }
    fn is_dummy(&self) -> bool {
        true
    }
}

impl TempSensor<f64> for Temp {
    fn get(&mut self) -> Result<f64> {
        Err(Error::NotImplemented)
    }
}

#[derive(Debug, Default)]
pub struct Wdg {}

impl Wdg {
    pub fn new() -> Wdg {
        Wdg::default()
    }
}

impl Watchdog for Wdg {
    fn enable(&mut self, _monitor: bool) -> Result<()> {
        Ok(())
    }

    fn service(&mut self) -> Result<()> {
        Ok(())
    }
}
