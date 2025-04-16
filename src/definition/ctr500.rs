// SPDX-License-Identifier: LGPL-3.0-or-later
// SPDX-FileCopyrightText: 2025 SYS TEC electronic AG <https://www.systec-electronic.com/>

use std::env;
use std::time::Duration;

use crate::io::{evdev, imx, null, sensors, sysfs};
use crate::labeled::Labeled;
use crate::Io;

const CNT0_PATH: &str = "/sys/devices/soc0/soc/30400000.aips-bus/30650000.flextimer/";

pub fn definition() -> Io {
    let disable_lmsensors = env::var("SYSWORXX_IO_DISABLE_LMSENSORS").is_ok();

    let tmp0: Box<dyn crate::TempSensor<f64>>;
    let tmp1: Box<dyn crate::TempSensor<f64>>;
    if disable_lmsensors {
        tmp0 = Box::new(null::Temp::new());
        tmp1 = Box::new(null::Temp::new());
    } else {
        tmp0 = Box::new(Labeled::new(
            "CPU",
            sensors::LmSensor::new("imx_thermal_zone-virtual-0", Duration::from_millis(2000)),
        ));
        tmp1 = Box::new(Labeled::new(
            "Baseboard",
            sensors::LmSensor::new("lm75-i2c-1-48", Duration::from_millis(2000)),
        ));
    };

    let mut digi_inputs = evdev::EvdevCollector::from_name("user_input").unwrap();

    Io {
        watchdog: Box::new(Labeled::new("Watchdog", null::Wdg::new())),
        run_led: Box::new(Labeled::new("Run_LED", sysfs::Do::new(73))),
        err_led: Box::new(Labeled::new("Error_LED", sysfs::Do::new(83))),
        run_switch: Box::new(Labeled::new(
            "Run_Switch",
            evdev::Di::active_low(&mut digi_inputs, evdev::KeyCode::KEY_1),
        )),
        config_switch: Box::new(Labeled::new("Config_Switch", sysfs::Di::active_low(176))),
        outputs: vec![
            Box::new(Labeled::new("DO0", sysfs::Do::new(75))),
            Box::new(Labeled::new("DO1", sysfs::Do::new(84))),
            // 2
            Box::new(Labeled::new("DO2", sysfs::Pwm::new(0, 0))),
            Box::new(Labeled::new("DO3", sysfs::Pwm::new(1, 0))),
            // 4
            Box::new(null::Output::not_implemented()),
            Box::new(null::Output::not_implemented()),
            Box::new(null::Output::not_implemented()),
            Box::new(null::Output::not_implemented()),
            Box::new(null::Output::not_implemented()),
            Box::new(null::Output::not_implemented()),
            Box::new(null::Output::not_implemented()),
            Box::new(null::Output::not_implemented()),
            Box::new(null::Output::not_implemented()),
            Box::new(null::Output::not_implemented()),
            Box::new(null::Output::not_implemented()),
            Box::new(null::Output::not_implemented()),
            // 16
            Box::new(null::Output::not_implemented()),
            Box::new(null::Output::not_implemented()),
            Box::new(null::Output::not_implemented()),
            Box::new(null::Output::not_implemented()),
            // 20
            Box::new(null::Output::not_implemented()),
            Box::new(null::Output::not_implemented()),
            Box::new(null::Output::not_implemented()),
            Box::new(null::Output::not_implemented()),
            Box::new(null::Output::not_implemented()),
            Box::new(null::Output::not_implemented()),
            Box::new(null::Output::not_implemented()),
            Box::new(null::Output::not_implemented()),
            Box::new(null::Output::not_implemented()),
            Box::new(null::Output::not_implemented()),
            // 30
            Box::new(null::Output::not_implemented()),
            Box::new(null::Output::not_implemented()),
            // 32
            Box::new(Labeled::new("Ext_Reset", sysfs::Do::new(80))), // /EXT_RESET
        ],
        inputs: vec![
            // 0
            Box::new(Labeled::new(
                "DI0",
                evdev::Di::new(&mut digi_inputs, evdev::KeyCode::KEY_F1),
            )),
            Box::new(Labeled::new(
                "DI1",
                evdev::Di::new(&mut digi_inputs, evdev::KeyCode::KEY_F2),
            )),
            Box::new(Labeled::new(
                "DI2",
                evdev::Di::new(&mut digi_inputs, evdev::KeyCode::KEY_F3),
            )),
            Box::new(Labeled::new(
                "DI3",
                evdev::Di::new(&mut digi_inputs, evdev::KeyCode::KEY_F4),
            )),
            Box::new(null::Input::not_implemented()),
            Box::new(null::Input::not_implemented()),
            Box::new(null::Input::not_implemented()),
            Box::new(null::Input::not_implemented()),
            Box::new(null::Input::not_implemented()),
            Box::new(null::Input::not_implemented()),
            Box::new(null::Input::not_implemented()),
            Box::new(null::Input::not_implemented()),
            Box::new(null::Input::not_implemented()),
            Box::new(null::Input::not_implemented()),
            Box::new(null::Input::not_implemented()),
            Box::new(null::Input::not_implemented()),
            // 16
            Box::new(null::Input::not_implemented()),
            Box::new(null::Input::not_implemented()),
            Box::new(null::Input::not_implemented()),
            Box::new(null::Input::not_implemented()),
            // 20
            Box::new(null::Input::not_implemented()),
            Box::new(null::Input::not_implemented()),
            Box::new(null::Input::not_implemented()),
            Box::new(null::Input::not_implemented()),
            Box::new(null::Input::not_implemented()),
            Box::new(null::Input::not_implemented()),
            Box::new(null::Input::not_implemented()),
            Box::new(null::Input::not_implemented()),
            Box::new(null::Input::not_implemented()),
            Box::new(null::Input::not_implemented()),
            // 30
            Box::new(null::Input::not_implemented()),
            Box::new(null::Input::not_implemented()),
            // 32
            Box::new(Labeled::new("PF", sysfs::Di::new(47))), // /PF
            Box::new(Labeled::new("DI_ERR", sysfs::Di::new(498))), // /DI_ERR
            Box::new(Labeled::new("USB_OC", sysfs::Di::new(499))), // USB_/OC
            Box::new(Labeled::new("DO_PF", sysfs::Di::new(496))), // DO_PF (actually not existing)
            Box::new(Labeled::new("DO_DIAG", sysfs::Di::new(497))), // DO_DIAG
            Box::new(Labeled::new("EXT_FAIL", sysfs::Di::new(43))), // /EXT_FAIL
            Box::new(Labeled::new(
                "RUN",
                evdev::Di::active_low(&mut digi_inputs, evdev::KeyCode::KEY_1),
            )), // RUN
        ],
        analog_inputs: vec![],
        analog_outputs: vec![],
        temp_sensors: vec![tmp0, tmp1],
        counter_input: vec![Box::new(imx::Counter::new(
            CNT0_PATH,
            Box::new(evdev::Di::new(&mut digi_inputs, evdev::KeyCode::KEY_F3)),
            None,
        ))],
        relay_offset: Some(16),
        pwm_outputs: vec![
            Box::new(sysfs::Pwm::new(0, 0)),
            Box::new(sysfs::Pwm::new(1, 0)),
        ],
    }
}
