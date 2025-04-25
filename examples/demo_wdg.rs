// SPDX-License-Identifier: LGPL-3.0-or-later
// SPDX-FileCopyrightText: 2025 SYS TEC electronic AG <https://www.systec-electronic.com/>

use std::thread::sleep;
use std::time::Duration;

use sysworxx_io::ffi::*;

fn chkerr(code: IoResult) {
    match code {
        IoResult::Success => {}
        _ => println!("Error: {:?}", code),
    }
}

fn main() {
    env_logger::init();

    chkerr(IoInit());

    // chkerr(IoEnableWatchdog(true));
    chkerr(IoEnableWatchdog(false.into()));
    chkerr(IoServiceWatchdog());

    let mut ticks = 0;

    loop {
        chkerr(unsafe { IoGetTickCount(&mut ticks) });
        print!("{:06}: ", ticks);

        sleep(Duration::from_millis(800));

        let mut value = false.into();
        chkerr(unsafe { IoGetInput(0, &mut value) });

        if *value {
            print!("Delay!");
            sleep(Duration::from_millis(201));
        }

        println!("");
        chkerr(IoServiceWatchdog());
    }
}
