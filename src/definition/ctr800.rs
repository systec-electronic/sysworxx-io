// SPDX-License-Identifier: LGPL-3.0-or-later
// SPDX-FileCopyrightText: 2025 SYS TEC electronic AG <https://www.systec-electronic.com/>

use std::time::Duration;

use ini::Ini;

use crate::io::lookup::{LogicLevel, Lookup};
use crate::io::{am62x, shm as shmio};
use crate::io::{evdev, iio, null, sensors, sysfs, util};
use crate::labeled::Labeled;
use crate::shm;
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

    let shm_sampler = shmio::Sampler::new();

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
            lookup.gpio_do("DO0", "4201000.gpio", 2),
            lookup.gpio_do("DO1", "4201000.gpio", 3),
            lookup.gpio_do("DO2", "600000.gpio", 51),
            lookup.gpio_do("DO3", "600000.gpio", 52),
            lookup.gpio_do("DO4", "600000.gpio", 57),
            lookup.gpio_do("DO5", "600000.gpio", 58),
            lookup.gpio_do("DO6", "600000.gpio", 59),
            lookup.gpio_do("DO7", "600000.gpio", 60),
            lookup.gpio_do("DO8", "600000.gpio", 61),
            lookup.gpio_do("DO9", "600000.gpio", 17),
            lookup.gpio_do("DO10", "600000.gpio", 1),
            lookup.gpio_do("DO11", "600000.gpio", 26),
            lookup.gpio_do("DO12", "600000.gpio", 19),
            lookup.gpio_do("DO13", "600000.gpio", 20),
            // 14
            Box::new(Labeled::new("DO14", sysfs::Pwm::new(0, 0))), // chip 0, channel 0
            Box::new(Labeled::new("DO15", sysfs::Pwm::new(2, 0))), // chip 2, channel 0
            // 16
            lookup.gpio_do("Relay0", "600000.gpio", 3),
            lookup.gpio_do("Relay1", "600000.gpio", 4),
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
            Box::new(Labeled::new(
                "DI4",
                evdev::Di::new(&mut digi_inputs, evdev::KeyCode::KEY_F5),
            )),
            Box::new(Labeled::new(
                "DI5",
                evdev::Di::new(&mut digi_inputs, evdev::KeyCode::KEY_F6),
            )),
            Box::new(Labeled::new(
                "DI6",
                evdev::Di::new(&mut digi_inputs, evdev::KeyCode::KEY_F7),
            )),
            Box::new(Labeled::new(
                "DI7",
                evdev::Di::new(&mut digi_inputs, evdev::KeyCode::KEY_F8),
            )),
            Box::new(Labeled::new(
                "DI8",
                evdev::Di::new(&mut digi_inputs, evdev::KeyCode::KEY_F9),
            )),
            Box::new(Labeled::new(
                "DI9",
                evdev::Di::new(&mut digi_inputs, evdev::KeyCode::KEY_F10),
            )),
            Box::new(Labeled::new(
                "DI10",
                evdev::Di::new(&mut digi_inputs, evdev::KeyCode::KEY_F11),
            )),
            Box::new(Labeled::new(
                "DI11",
                evdev::Di::new(&mut digi_inputs, evdev::KeyCode::KEY_F12),
            )),
            Box::new(Labeled::new(
                "DI12",
                evdev::Di::new(&mut digi_inputs, evdev::KeyCode::KEY_F13),
            )),
            Box::new(Labeled::new(
                "DI13",
                evdev::Di::new(&mut digi_inputs, evdev::KeyCode::KEY_F14),
            )),
            Box::new(Labeled::new(
                "DI14",
                evdev::Di::new(&mut digi_inputs, evdev::KeyCode::KEY_F15),
            )),
            Box::new(Labeled::new(
                "DI15",
                evdev::Di::new(&mut digi_inputs, evdev::KeyCode::KEY_F16),
            )),
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
        analog_inputs: vec![
            Box::new(Labeled::new("AIN0", shmio::Ai::new(&shm_sampler, 0))),
            Box::new(Labeled::new("AIN1", shmio::Ai::new(&shm_sampler, 1))),
            Box::new(Labeled::new("AIN2", shmio::Ai::new(&shm_sampler, 2))),
            Box::new(Labeled::new("AIN3", shmio::Ai::new(&shm_sampler, 3))),
        ],
        analog_outputs: vec![],
        temp_sensors: vec![tmp0, tmp1],
        counter_input: vec![Box::new(am62x::Counter::new(
            CNT0_PATH,
            Box::new(evdev::Di::new(&mut digi_inputs, evdev::KeyCode::KEY_F15)),
        ))],
        relay_offset: Some(16),
        // maximum PWM period is 469754879ns (~469ms)
        pwm_outputs: vec![
            Box::new(sysfs::Pwm::new(0, 0)), // chip, channel
            Box::new(sysfs::Pwm::new(2, 0)), // chip, channel
        ],
    }
}

pub fn definition_shm() -> (Io, shm::Mappings) {
    let adc_calib = Ini::load_from_file("/boot/vendor/adc_calib").unwrap_or_default();
    let shifter = util::Shifter::new(util::Shift::Up(3));
    let adc_sampler: iio::Sampler<i64> =
        iio::Sampler::new("iio:device0", Duration::from_millis(100));
    let notifier_adc = adc_sampler.get_notifier();
    let lookup: Lookup = Lookup::new();

    let adc_channel = |index, gpio_offset, gpio_chip, calib_section| {
        let (do_v, do_i) = lookup.gpio_pair_adc(gpio_chip, gpio_offset);
        util::AiIniCalib::new_shift(
            &adc_calib,
            calib_section,
            util::AiSwitch::new(
                iio::Ai::new(&adc_sampler, index),
                sysfs::Do::new(do_v),
                sysfs::Do::new(do_i),
            ),
            shifter,
        )
    };

    let io = Io {
        watchdog: Box::new(null::Wdg::new()),
        run_led: Box::new(null::Output::not_implemented()),
        err_led: Box::new(null::Output::not_implemented()),
        run_switch: Box::new(null::Input::not_implemented()),
        config_switch: Box::new(null::Input::not_implemented()),
        outputs: vec![],
        inputs: vec![],
        analog_inputs: vec![
            Box::new(adc_channel(1, 0, "1-0038", "AIN0")),
            Box::new(adc_channel(4, 1, "1-0038", "AIN1")),
            Box::new(adc_channel(6, 2, "1-0038", "AIN2")),
            Box::new(adc_channel(8, 3, "1-0038", "AIN3")),
        ],
        analog_outputs: vec![],
        temp_sensors: vec![],
        counter_input: vec![],
        relay_offset: None,
        pwm_outputs: vec![],
    };

    let shm_mapping = shm::Mappings {
        groups: vec![shm::Group {
            notifier: notifier_adc,
            channels: shm::Channels::AnalogInput(vec![0, 1, 2, 3]),
        }],
    };

    (io, shm_mapping)
}
