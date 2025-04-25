// SPDX-License-Identifier: LGPL-3.0-or-later
// SPDX-FileCopyrightText: 2025 SYS TEC electronic AG <https://www.systec-electronic.com/>

use std::time::Instant;

use raw_sync::Timeout;
use sysworxx_io::definition::load_device_definition_shm;
use sysworxx_io::{ffi, hw_rev, shm};

pub fn main() {
    let mut shm_client = shm::ShmClient::new().unwrap();
    let (_, mappings) = match load_device_definition_shm(
        &hw_rev::get_device_name().unwrap_or("fallback".to_string()),
    ) {
        Some(e) => e,
        None => {
            eprintln!("Device does not support iodaemon!");
            std::process::exit(1);
        }
    };

    let count_adc: usize = mappings
        .groups
        .iter()
        .filter_map(|group| match &group.channels {
            shm::Channels::AnalogInput(channels) => Some(channels.len()),
            _ => None,
        })
        .sum();

    let count_tmp: usize = mappings
        .groups
        .iter()
        .filter_map(|group| match &group.channels {
            shm::Channels::TempInput(channels) => Some(channels.len()),
            _ => None,
        })
        .sum();

    dbg!(count_adc, count_tmp);

    {
        let mut shm = shm_client.lock();
        shm.analog_cfg_set(0, ffi::IoAnalogMode::Voltage);
        shm.analog_cfg_set(1, ffi::IoAnalogMode::Voltage);
        shm.analog_cfg_set(4, ffi::IoAnalogMode::Current);
        shm.analog_cfg_set(5, ffi::IoAnalogMode::Current);
        shm.temperature_cfg_set(0, ffi::IoTmpMode::RtdTwoWire, ffi::IoTmpSensorType::PT100);
        shm.temperature_cfg_set(1, ffi::IoTmpMode::RtdTwoWire, ffi::IoTmpSensorType::PT1000);
        shm.temperature_cfg_set(2, ffi::IoTmpMode::RtdThreeWire, ffi::IoTmpSensorType::PT100);
        shm.temperature_cfg_set(3, ffi::IoTmpMode::RtdFourWire, ffi::IoTmpSensorType::PT100);
        // this should produce an error on the server side
        shm.temperature_cfg_set(6, ffi::IoTmpMode::RtdFourWire, ffi::IoTmpSensorType::PT100);
    }
    shm_client.emit_client_event().expect("emit client event");

    let mut start = Instant::now();
    loop {
        shm_client
            .await_server_event(Timeout::Infinite)
            .expect("await server event");

        let now = Instant::now();
        println!("{:?}", (now - start).as_millis());
        start = now;

        {
            let mut locked = shm_client.lock();
            for i in 0..count_adc {
                println!("A{}: {}", i, locked.analog_value_get(i));
            }
            for i in 0..count_tmp {
                println!("T{}: {}", i, locked.temperature_value_get(i));
            }
        }
    }
}
