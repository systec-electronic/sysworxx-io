// SPDX-License-Identifier: LGPL-3.0-or-later
//
// (c) SYSTEC electronic AG, D-08468 Heinsdorfergrund, Am Windrad 2
//     www.systec-electronic.com

use std::fmt;
use std::fs::File;
use std::io::prelude::*;
use std::path::PathBuf;

use crate::error::*;
use crate::{DigitalOutput, IoChannel};

pub struct Led {
    name: &'static str,
    file: Option<File>,
}

impl fmt::Debug for Led {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "led/Led {}", self.name)
    }
}

impl Led {
    pub fn new(name: &'static str) -> Led {
        Led { name, file: None }
    }
}

impl IoChannel for Led {
    fn init(&mut self, _chan_number: usize) -> Result<()> {
        Ok(())
    }

    fn shutdown(&mut self) -> Result<()> {
        Ok(())
    }
}

impl DigitalOutput for Led {
    fn set(&mut self, val: bool) -> Result<()> {
        // open file on set and leave it open once used for better concurrent usage
        if self.file.is_none() {
            let mut path_buf = PathBuf::new();
            path_buf.push("/sys/class/leds");
            path_buf.push(self.name);
            path_buf.push("brightness");

            self.file = Some(File::create(path_buf)?);
        }

        let v = if val { b"1" } else { b"0" };
        match &mut self.file {
            Some(f) => {
                f.write_all(v)?;
                Ok(())
            }
            None => Err(Error::GenericError),
        }
    }
}
