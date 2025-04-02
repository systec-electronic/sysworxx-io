// SPDX-License-Identifier: LGPL-3.0-or-later
//
// (c) SYSTEC electronic AG, D-08468 Heinsdorfergrund, Am Windrad 2
//     www.systec-electronic.com

use std::time::Duration;

use crate::io::am62x;
use crate::io::lookup::{LogicLevel, Lookup};
use crate::io::{evdev, null, sensors, sysfs};
use crate::labeled::Labeled;
use crate::Io;

const CNT0_PATH: &str = "/sys/bus/counter/devices/counter0/count0/";

pub fn definition() -> Io {
    let tmp0 = Box::new(Labeled::new(
        "CPU",
        sensors::LmSensor::new("main1_thermal-virtual-0", Duration::from_millis(2000)),
    ));

    let tmp1 = Box::new(Labeled::new(
        "Baseboard",
        sensors::LmSensor::new("lm75-i2c-1-48", Duration::from_millis(2000)),
    ));

    let lookup: Lookup = Lookup::new();

    let mut digi_inputs = evdev::EvdevCollector::from_name("gpio_input").unwrap();

    Io {
        watchdog: Box::new(null::Wdg::new()),
        run_led: lookup.gpio_do("Run_LED", "600000.gpio", 5),
        err_led: lookup.gpio_do("Error_LED", "600000.gpio", 6),
        run_switch: Box::new(Labeled::new(
            "Run_Switch",
            evdev::Di::active_low(&mut digi_inputs, evdev::KeyCode::KEY_1),
        )),
        config_switch: lookup.gpio_di("Config_Switch", "600000.gpio", 62, LogicLevel::ActiveLow),
        outputs: vec![
            lookup.gpio_do("DO0", "600000.gpio", 19),
            lookup.gpio_do("DO1", "600000.gpio", 20),
            Box::new(Labeled::new("DO2", sysfs::Pwm::new(0, 0))), // chip 0, channel 0
            Box::new(Labeled::new("DO3", sysfs::Pwm::new(2, 0))), // chip 2, channel 0
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
            // 14
            Box::new(null::Output::not_implemented()),
            Box::new(null::Output::not_implemented()),
            // 16
            Box::new(null::Output::not_implemented()),
            Box::new(null::Output::not_implemented()),
            // 18
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
            Box::new(null::Output::not_implemented()),
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
            lookup.gpio_di("PF", "600000.gpio", 43, LogicLevel::ActiveHigh),
            lookup.gpio_di("DI_ERR", "1-003a", 2, LogicLevel::ActiveHigh),
            lookup.gpio_di("USB_OC", "1-003a", 3, LogicLevel::ActiveHigh),
            lookup.gpio_di("DO_PF", "1-003a", 0, LogicLevel::ActiveHigh),
            lookup.gpio_di("DO_DIAG", "1-003a", 1, LogicLevel::ActiveHigh),
            Box::new(null::Input::not_implemented()), // EXT_FAIL?
            Box::new(Labeled::new(
                "RUN",
                evdev::Di::active_low(&mut digi_inputs, evdev::KeyCode::KEY_1),
            )),
        ],
        analog_inputs: vec![],
        analog_outputs: vec![],
        temp_sensors: vec![tmp0, tmp1],
        counter_input: vec![Box::new(am62x::Counter::new(
            CNT0_PATH,
            Box::new(evdev::Di::new(&mut digi_inputs, evdev::KeyCode::KEY_F15)),
        ))],
        relay_offset: Some(16),
        pwm_outputs: vec![
            Box::new(sysfs::Pwm::new(0, 0)), // chip, channel
            Box::new(sysfs::Pwm::new(2, 0)), // chip, channel
        ],
    }
}
