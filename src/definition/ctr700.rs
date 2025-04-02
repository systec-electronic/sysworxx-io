// SPDX-License-Identifier: LGPL-3.0-or-later
//
// (c) SYSTEC electronic AG, D-08468 Heinsdorfergrund, Am Windrad 2
//     www.systec-electronic.com

use std::env;
use std::time::Duration;

use ini::Ini;

use crate::hw_rev;
use crate::io::shm as shmio;
use crate::io::{evdev, iio, imx, null, sensors, sysfs, util};
use crate::labeled::Labeled;
use crate::shm;
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

    let rev = hw_rev::get_hardware_revision().unwrap_or(0xff);

    let shm_sampler = shmio::Sampler::new();

    let (do10, do11, do_ext_reset) = match rev {
        0 => (
            Box::new(Labeled::new("DO10", sysfs::Do::new(80))),
            Box::new(Labeled::new("DO11", sysfs::Do::new(81))),
            Box::new(Labeled::new("Ext_Reset", sysfs::Do::new(42))),
        ),
        _ => (
            Box::new(Labeled::new("DO10", sysfs::Do::new(42))),
            Box::new(Labeled::new("DO11", sysfs::Do::new(79))),
            Box::new(Labeled::new("Ext_Reset", sysfs::Do::new(80))),
        ),
    };

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
            Box::new(Labeled::new("DO0", sysfs::Do::new(70))),
            Box::new(Labeled::new("DO1", sysfs::Do::new(71))),
            Box::new(Labeled::new("DO2", sysfs::Do::new(88))),
            Box::new(Labeled::new("DO3", sysfs::Do::new(85))),
            Box::new(Labeled::new("DO4", sysfs::Do::new(72))),
            Box::new(Labeled::new("DO5", sysfs::Do::new(87))),
            Box::new(Labeled::new("DO6", sysfs::Do::new(86))),
            Box::new(Labeled::new("DO7", sysfs::Do::new(69))),
            Box::new(Labeled::new("DO8", sysfs::Do::new(76))),
            Box::new(Labeled::new("DO9", sysfs::Do::new(77))),
            do10,
            do11,
            Box::new(Labeled::new("DO12", sysfs::Do::new(75))),
            Box::new(Labeled::new("DO13", sysfs::Do::new(84))),
            // 14
            Box::new(Labeled::new("DO14", sysfs::Pwm::new(0, 0))),
            Box::new(Labeled::new("DO15", sysfs::Pwm::new(1, 0))),
            // 16
            Box::new(Labeled::new("Relay0", sysfs::Do::new(74))),
            Box::new(Labeled::new("Relay1", sysfs::Do::new(78))),
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
            do_ext_reset, // /EXT_RESET
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
            Box::new(Labeled::new("PF", sysfs::Di::new(47))), // /PF
            Box::new(Labeled::new("DI_ERR", sysfs::Di::new(490))), // /DI_ERR
            Box::new(Labeled::new("USB_OC", sysfs::Di::new(491))), // USB_/OC
            Box::new(Labeled::new("DO_PF", sysfs::Di::new(488))), // DO_PF
            Box::new(Labeled::new("DO_DIAG", sysfs::Di::new(489))), // DO_DIAG
            Box::new(Labeled::new("EXT_FAIL", sysfs::Di::new(43))), // /EXT_FAIL
            Box::new(Labeled::new(
                "RUN",
                evdev::Di::active_low(&mut digi_inputs, evdev::KeyCode::KEY_1),
            )), // RUN
        ],
        analog_inputs: vec![
            Box::new(Labeled::new("AI0", shmio::Ai::new(&shm_sampler, 0))),
            Box::new(Labeled::new("AI1", shmio::Ai::new(&shm_sampler, 1))),
            Box::new(Labeled::new("AI2", shmio::Ai::new(&shm_sampler, 2))),
            Box::new(Labeled::new("AI3", shmio::Ai::new(&shm_sampler, 3))),
        ],
        analog_outputs: vec![],
        temp_sensors: vec![tmp0, tmp1],
        counter_input: vec![Box::new(imx::Counter::new(
            CNT0_PATH,
            Box::new(evdev::Di::new(&mut digi_inputs, evdev::KeyCode::KEY_F15)),
            None,
        ))],
        relay_offset: Some(16),
        pwm_outputs: vec![
            Box::new(sysfs::Pwm::new(0, 0)),
            Box::new(sysfs::Pwm::new(1, 0)),
        ],
    }
}

pub fn definition_shm() -> (Io, shm::Mappings) {
    let adc_calib = Ini::load_from_file("/vendor/adc_calib").unwrap_or_default();

    let shifter = util::Shifter::new(util::Shift::Up(3));

    let adc_sampler: iio::Sampler<i64> =
        iio::Sampler::new("iio:device1", Duration::from_millis(100));
    let notifier_adc = adc_sampler.get_notifier();
    let adc_channel = |index, do_v, do_i, calib_section| {
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
            Box::new(adc_channel(0, 504, 508, "AIN0")),
            Box::new(adc_channel(1, 505, 509, "AIN1")),
            Box::new(adc_channel(2, 506, 510, "AIN2")),
            Box::new(adc_channel(3, 507, 511, "AIN3")),
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
