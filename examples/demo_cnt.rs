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
    chkerr(IoInit());

    let mut hw: IoHwInfo = IoHwInfo::default();
    chkerr(unsafe { IoGetHardwareInfo(&mut hw) });
    println!("{:#?}", hw);

    chkerr(IoCntSetup(
        0,
        IoCntMode::Counter,
        IoCntTrigger::AnyEdge,
        IoCntDirection::Down,
    ));
    chkerr(IoCntSetPreload(0, 0));
    chkerr(IoCntEnable(0, true.into()));

    chkerr(IoCntSetup(
        1,
        IoCntMode::Counter,
        IoCntTrigger::AnyEdge,
        IoCntDirection::Down,
    ));
    chkerr(IoCntSetPreload(1, 10));
    chkerr(IoCntEnable(1, true.into()));

    let mut value = 0;

    loop {
        sleep(Duration::from_millis(200));

        chkerr(unsafe { IoCntGetValue(0, &mut value) });
        print!("{} ", value);
        chkerr(unsafe { IoCntGetValue(1, &mut value) });
        println!("{}", value);
    }

    // chkerr(IoShutdown());
}
