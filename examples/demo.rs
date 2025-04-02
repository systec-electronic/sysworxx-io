// SPDX-License-Identifier: LGPL-3.0-or-later
//
// (c) SYSTEC electronic AG, D-08468 Heinsdorfergrund, Am Windrad 2
//     www.systec-electronic.com

use std::thread::sleep;
use std::time::Duration;

use sysworxx_io::ffi::*;

fn chkerr(code: IoResult) {
    match code {
        IoResult::Success => {}
        _ => println!("Error: {:?}", code),
    }
}

extern "C" fn input_callback(channel: u8, value: IoBool) -> () {
    println!("value changed, channel: {}, value: {}", channel, *value);

    let mut mask = 0;
    for i in 0..11 {
        let mut val = false.into();
        chkerr(unsafe { IoGetInput(i, &mut val) });
        if *val {
            mask |= 1 << i
        };
    }

    chkerr(IoUnregisterInputCallback(channel));

    println!("DI: {:04x}", mask);
}

fn main() {
    let delay = Duration::from_millis(200);

    chkerr(IoInit());

    let mut major = 0;
    let mut minor = 0;
    unsafe { IoGetVersion(&mut major, &mut minor) };
    println!("sysworxx-io, version {}.{}", major, minor);

    let mut hw: IoHwInfo = IoHwInfo::default();
    chkerr(unsafe { IoGetHardwareInfo(&mut hw) });
    println!("{:#?}", hw);

    chkerr(IoSetOutput(33, true.into()));

    chkerr(IoSetOutput(0, true.into()));
    sleep(delay);
    chkerr(IoSetOutput(0, false.into()));

    sleep(delay);

    chkerr(IoSetOutput(1, true.into()));
    sleep(delay);
    chkerr(IoSetOutput(1, false.into()));

    for i in 0..11 {
        chkerr(IoRegisterInputCallback(
            i,
            Some(input_callback),
            IoInputTrigger::RisingEdge,
        ));
    }

    for channel in 0..8 {
        IoAdcSetMode(channel, IoAnalogMode::Voltage);
    }

    IoAdcSetMode(2, IoAnalogMode::Current);
    IoAdcSetMode(3, IoAnalogMode::Current);

    IoTmpSetMode(2, IoTmpMode::RtdTwoWire, IoTmpSensorType::PT100);
    IoTmpSetMode(3, IoTmpMode::RtdTwoWire, IoTmpSensorType::PT1000);
    IoTmpSetMode(4, IoTmpMode::RtdThreeWire, IoTmpSensorType::PT100);
    IoTmpSetMode(5, IoTmpMode::RtdThreeWire, IoTmpSensorType::PT1000);
    IoTmpSetMode(6, IoTmpMode::RtdFourWire, IoTmpSensorType::PT100);
    IoTmpSetMode(7, IoTmpMode::RtdFourWire, IoTmpSensorType::PT1000);

    let mut dac_val = 0;
    let mut ticks = 0;
    let mut led = true.into();

    loop {
        chkerr(IoSetRunLed(led));
        chkerr(IoSetErrLed(!led));
        led = !led;

        chkerr(unsafe { IoGetTickCount(&mut ticks) });
        print!("{:06}: ", ticks);

        sleep(Duration::from_millis(1000));

        for channel in 0..hw.m_uAiChannels {
            let mut value = 0;
            unsafe { IoAdcGetValue(channel, &mut value) };
            print!("{}: {} | ", channel, value);
        }

        print!("  ");

        for channel in 0..hw.m_uTmpChannels {
            let mut value = 0;
            unsafe { IoTmpGetValue(channel, &mut value) };

            print!("{}: ", channel);

            if value != std::i32::MAX {
                print!("{} °C | ", value);
            } else {
                print!("NC | ");
            }
        }

        dac_val = if dac_val < 30000 { dac_val + 5000 } else { 0 };

        print!(" AO 0-{}: {}", hw.m_uAoChannels - 1, dac_val);
        for c in 0..hw.m_uAoChannels {
            chkerr(IoDacSetValue(c, dac_val));
        }

        println!("");
    }

    // chkerr(IoShutdown());
}
