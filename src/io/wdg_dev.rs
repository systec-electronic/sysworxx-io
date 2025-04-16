// SPDX-License-Identifier: LGPL-3.0-or-later
// SPDX-FileCopyrightText: 2025 SYS TEC electronic AG <https://www.systec-electronic.com/>

// use std::fs::File;
// use std::io::prelude::*;
// use std::os::unix::io::AsRawFd;
// use std::time::Instant;

use crate::error::{Error, Result};
use crate::Watchdog;

// TODO: This implementation works except setting the watchdog's timeout.

// const WATCHDOG_IOCTL_BASE: u8 = b'W'; // Defined in linux/spi/spidev.h
// const WDIOC_SETTIMEOUT: u8 = 6;
// ioctl_write_int!(wdg_set_timeout, WATCHDOG_IOCTL_BASE, WDIOC_SETTIMEOUT);
//
// const WDG_INTERVAL: nix::sys::ioctl::ioctl_param_type = 30;

#[derive(Debug)]
pub struct Wdg {
    //     path: &'static str,
    //     file: Option<File>,
    //     last_ticks: Instant,
}

impl Wdg {
    pub fn new(_path: &'static str) -> Wdg {
        Wdg {
            // path,
            // file: None,
            // last_ticks: Instant::now(),
        }
    }
}

impl Watchdog for Wdg {
    fn enable(&mut self, _monitor: bool) -> Result<()> {
        Err(Error::NotImplemented)
        // let mut f = File::create(self.path)?;

        // if monitor {
        //     write!(f, "V")?;
        //     self.file = None;
        //     drop(f);
        // } else {
        //     unsafe {
        //         dbg!(f.as_raw_fd());
        //         let ret = wdg_set_timeout(f.as_raw_fd(), WDG_INTERVAL);
        //         dbg!(ret);
        //     }

        //     self.file = Some(f);
        // }

        // Ok(())
    }

    fn service(&mut self) -> Result<()> {
        Err(Error::NotImplemented)
        // match &mut self.file {
        //     Some(f) => {
        //         write!(f, "a")?;
        //     },
        //     _ => { /* monitoring mode */ }
        // }

        // let now = Instant::now();
        // let diff = now.checked_duration_since(self.last_ticks);
        // self.last_ticks = now;

        // match diff {
        //     None => {
        //         Err(Error::WatchdogTimeout)
        //     },
        //     Some(diff) => {
        //         if diff.as_millis() > (WDG_INTERVAL * 1000) as u128 {
        //             Err(Error::WatchdogTimeout)
        //         } else {
        //             Ok(())
        //         }
        //     },
        // }
    }
}

// impl Drop for Wdg {
//     fn drop(&mut self) {
//         match &mut self.file {
//             Some(f) => {
//                 write!(f, "V").unwrap();
//                 drop(f);
//             },
//             _ => { }
//         }
//
//         self.file = None;
//     }
// }
