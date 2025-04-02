// SPDX-License-Identifier: LGPL-3.0-or-later
//
// (c) SYSTEC electronic AG, D-08468 Heinsdorfergrund, Am Windrad 2
//     www.systec-electronic.com

use std::env;
use std::time::Duration;

use ini::Ini;

use crate::convert::rtd;
use crate::io::shm as shmio;
use crate::io::{evdev, iio, imx, led, null, sensors, sysfs, util};
use crate::labeled::Labeled;
use crate::shm;
use crate::Io;

const CNT0_PATH: &str = "/sys/devices/soc0/soc/30400000.aips-bus/30640000.flextimer/";
const CNT1_PATH: &str = "/sys/devices/soc0/soc/30400000.aips-bus/30650000.flextimer/";

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
            sensors::LmSensor::new("lm75-i2c-0-48", Duration::from_millis(2000)),
        ));
    };

    let dac_calib = Ini::load_from_file("/vendor/dac_calib").unwrap_or_default();

    let mut digi_inputs = evdev::EvdevCollector::from_name("inputs").unwrap();

    let writer_dac: iio::Writer<i64> = iio::Writer::from_spi(0, 3);
    let shifter = util::Shifter::new(util::Shift::Down(3));
    let clipper = util::Clip::new(0, 4095);

    let shm_sampler = shmio::Sampler::new();

    Io {
        watchdog: Box::new(null::Wdg::new()),
        run_led: Box::new(led::Led::new("RUN")),
        err_led: Box::new(led::Led::new("ERROR")),
        run_switch: Box::new(null::Input::always_active()),
        config_switch: Box::new(sysfs::Di::new(176)),
        outputs: vec![
            Box::new(Labeled::new("Relay0", sysfs::Do::new(74))), // REL0
            Box::new(Labeled::new("Relay1", sysfs::Do::new(78))), // REL1
            // 2
            Box::new(null::Output::not_implemented()),
            Box::new(null::Output::not_implemented()),
            Box::new(null::Output::not_implemented()),
            Box::new(null::Output::not_implemented()),
            Box::new(null::Output::not_implemented()),
            Box::new(null::Output::not_implemented()),
            Box::new(null::Output::not_implemented()),
            Box::new(null::Output::not_implemented()),
            // 10
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
            Box::new(Labeled::new("SER_MODE", sysfs::Do::new(466))), // SER_MODE
            Box::new(Labeled::new("DAC_EN", sysfs::Do::new(129))),   // DAC_EN
            Box::new(Labeled::new(
                "MODEM_/RST",
                sysfs::Do::active_low_init_high(13),
            )), // MODEM_/RST
            Box::new(Labeled::new(
                "WDG_EN",
                util::DoOnly::new(true, sysfs::Do::new(11)),
            )), // WDG_EN
            Box::new(Labeled::new("MODEM_EN", sysfs::Do::new(5))),   // MODEM_EN
            Box::new(Labeled::new("SER_DPLX", sysfs::Do::new(470))), // SER_DPLX
            Box::new(null::Output::not_implemented()),
            Box::new(null::Output::not_implemented()),
            // 40
            Box::new(Labeled::new(
                "server_status",
                led::Led::new("server_status"),
            )),
            Box::new(Labeled::new(
                "signal_strength_1",
                led::Led::new("signal_strength_1"),
            )),
            Box::new(Labeled::new(
                "signal_strength_2",
                led::Led::new("signal_strength_2"),
            )),
            Box::new(Labeled::new(
                "signal_strength_3",
                led::Led::new("signal_strength_3"),
            )),
            Box::new(Labeled::new(
                "rts_operational",
                led::Led::new("rts_operational"),
            )),
            Box::new(Labeled::new("serial_rx", led::Led::new("serial_rx"))),
            Box::new(Labeled::new("serial_tx", led::Led::new("serial_tx"))),
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
            // 10
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
            Box::new(Labeled::new("/Powerfail", sysfs::Di::new(47))), // /PF
            Box::new(Labeled::new("/DI_ERR", sysfs::Di::new(472))),   // /DI_ERR
            Box::new(Labeled::new("USB_/OC", sysfs::Di::new(473))),   // USB_/OC
            Box::new(Labeled::new("AOUT0_/ERR", sysfs::Di::new(474))), // AOUT0_/ERR
            Box::new(Labeled::new("AOUT1_/ERR", sysfs::Di::new(475))), // AOUT1_/ERR
            Box::new(Labeled::new("AOUT2_/ERR", sysfs::Di::new(476))), // AOUT2_/ERR
            Box::new(Labeled::new("AOUT3_/ERR", sysfs::Di::new(477))), // AOUT3_/ERR
        ],
        analog_inputs: vec![
            Box::new(Labeled::new("AI0", shmio::Ai::new(&shm_sampler, 0))),
            Box::new(Labeled::new("AI1", shmio::Ai::new(&shm_sampler, 1))),
            Box::new(Labeled::new("AI2", shmio::Ai::new(&shm_sampler, 2))),
            Box::new(Labeled::new("AI3", shmio::Ai::new(&shm_sampler, 3))),
            Box::new(Labeled::new("AI4", shmio::Ai::new(&shm_sampler, 4))),
            Box::new(Labeled::new("AI5", shmio::Ai::new(&shm_sampler, 5))),
            Box::new(Labeled::new("AI6", shmio::Ai::new(&shm_sampler, 6))),
            Box::new(Labeled::new("AI7", shmio::Ai::new(&shm_sampler, 7))),
        ],
        analog_outputs: vec![
            Box::new(Labeled::new(
                "AO0: 0 - 10 V",
                iio::Ao::new(&writer_dac, 0, shifter, clipper, &dac_calib, "AOUT0"),
            )),
            Box::new(Labeled::new(
                "AO1: 0 - 10 V",
                iio::Ao::new(&writer_dac, 1, shifter, clipper, &dac_calib, "AOUT1"),
            )),
            Box::new(Labeled::new(
                "AO2: 0 - 20 mA",
                iio::Ao::new(&writer_dac, 2, shifter, clipper, &dac_calib, "AOUT2"),
            )),
            Box::new(Labeled::new(
                "AO3: 0 - 20 mA",
                iio::Ao::new(&writer_dac, 3, shifter, clipper, &dac_calib, "AOUT3"),
            )),
        ],
        temp_sensors: vec![
            tmp0,
            tmp1,
            Box::new(Labeled::new("RTD0", shmio::Temp::new(&shm_sampler, 0))),
            Box::new(Labeled::new("RTD1", shmio::Temp::new(&shm_sampler, 1))),
            Box::new(Labeled::new("RTD2", shmio::Temp::new(&shm_sampler, 2))),
            Box::new(Labeled::new("RTD3", shmio::Temp::new(&shm_sampler, 3))),
            Box::new(Labeled::new("RTD4", shmio::Temp::new(&shm_sampler, 4))),
            Box::new(Labeled::new("RTD5", shmio::Temp::new(&shm_sampler, 5))),
            Box::new(Labeled::new("TC0", shmio::Temp::new(&shm_sampler, 6))),
            Box::new(Labeled::new("TC1", shmio::Temp::new(&shm_sampler, 7))),
            Box::new(Labeled::new("TC2", shmio::Temp::new(&shm_sampler, 8))),
            Box::new(Labeled::new("TC3", shmio::Temp::new(&shm_sampler, 9))),
        ],
        counter_input: vec![
            Box::new(Labeled::new(
                "CI0",
                imx::Counter::new(
                    CNT0_PATH,
                    Box::new(evdev::Di::new(&mut digi_inputs, evdev::KeyCode::KEY_F9)),
                    None,
                ),
            )),
            Box::new(Labeled::new(
                "CI1",
                imx::Counter::new(
                    CNT1_PATH,
                    Box::new(evdev::Di::new(&mut digi_inputs, evdev::KeyCode::KEY_F10)),
                    None,
                ),
            )),
        ],
        relay_offset: Some(0),
        pwm_outputs: vec![],
    }
}

pub fn definition_shm() -> (Io, shm::Mappings) {
    let adc_calib = Ini::load_from_file("/vendor/adc_calib").unwrap_or_default();
    let rtd_calib = Ini::load_from_file("/vendor/rtd_calib").unwrap_or_default();
    let tc_calib = Ini::load_from_file("/vendor/tc_calib").unwrap_or_default();

    let mut adc_sampler: iio::Sampler<i64> =
        iio::Sampler::from_spi(0, 0, Duration::from_millis(100));
    adc_sampler
        .attr_write(iio::DevAttr::SamplingFrequency(iio::AttrValue::F64(200.0)))
        .unwrap();
    let notifier_adc = adc_sampler.get_notifier();
    let adc_channel = |index, do_v, do_u, calib_section| {
        util::AiIniCalib::new(
            &adc_calib,
            calib_section,
            util::AiSwitch::new(
                iio::Ai::new(&adc_sampler, index),
                sysfs::Do::new(do_v),
                sysfs::Do::new(do_u),
            ),
        )
    };

    let mut sampler_rtd0: iio::Sampler<f64> =
        iio::Sampler::from_spi(0, 1, Duration::from_millis(250));
    let mut sampler_rtd1: iio::Sampler<f64> =
        iio::Sampler::from_spi(0, 2, Duration::from_millis(250));
    let notifier_rtd0 = sampler_rtd0.get_notifier();
    let notifier_rtd1 = sampler_rtd1.get_notifier();

    sampler_rtd0
        .attr_write(iio::DevAttr::SamplingFrequency(iio::AttrValue::F64(20.0)))
        .unwrap();
    sampler_rtd1
        .attr_write(iio::DevAttr::SamplingFrequency(iio::AttrValue::F64(20.0)))
        .unwrap();

    let rtd_channel = |sampler, index, calib_section| {
        rtd::RtdCalc::new(util::TmpRtdIniCalib::new(
            &rtd_calib,
            calib_section,
            iio::TempRtd::new(sampler, index),
        ))
    };

    let mut sampler_tc0: iio::Sampler<f64> =
        iio::Sampler::from_spi(1, 0, Duration::from_millis(500));
    sampler_tc0
        .attr_write(iio::DevAttr::SamplingFrequency(iio::AttrValue::I64(8)))
        .unwrap();

    // tc0: spi1.0: ambient + voltage2-voltage3
    // tc1: spi1.0: ambient + voltage0-voltage1
    let mut tc0 = iio::TempTc::new(&sampler_tc0, &tc_calib, "TC0", 0, 7);
    let mut tc1 = iio::TempTc::new(&sampler_tc0, &tc_calib, "TC1", 0, 2);
    tc0.attr_write(iio::ChanAttr::VoltageScale(iio::AttrValue::I64(5)))
        .unwrap();
    tc1.attr_write(iio::ChanAttr::VoltageScale(iio::AttrValue::I64(5)))
        .unwrap();

    let mut sampler_tc1: iio::Sampler<f64> =
        iio::Sampler::from_spi(1, 1, Duration::from_millis(500));
    sampler_tc1
        .attr_write(iio::DevAttr::SamplingFrequency(iio::AttrValue::I64(8)))
        .unwrap();

    // tc2: spi1.1: ambient + voltage2-voltage3
    // tc3: spi1.1: ambient + voltage0-voltage1
    let mut tc2 = iio::TempTc::new(&sampler_tc1, &tc_calib, "TC2", 0, 7);
    let mut tc3 = iio::TempTc::new(&sampler_tc1, &tc_calib, "TC3", 0, 2);
    tc2.attr_write(iio::ChanAttr::VoltageScale(iio::AttrValue::I64(5)))
        .unwrap();
    tc3.attr_write(iio::ChanAttr::VoltageScale(iio::AttrValue::I64(5)))
        .unwrap();

    let notifier_tc0 = sampler_tc0.get_notifier();
    let notifier_tc1 = sampler_tc1.get_notifier();

    {
        // This is a workaround to force the kernel to initialize gpio6 and keep its configuration.
        // Without this ADS1118 / SPI1.0 will not work since /CS would not be initialized as
        // output.
        let mut workaround_gpio6: Box<dyn crate::DigitalOutput> = Box::new(sysfs::Do::new(171));
        workaround_gpio6.init(0).unwrap();
    }

    let io = Io {
        watchdog: Box::new(null::Wdg::new()),
        run_led: Box::new(null::Output::not_implemented()),
        err_led: Box::new(null::Output::not_implemented()),
        run_switch: Box::new(null::Input::not_implemented()),
        config_switch: Box::new(null::Input::not_implemented()),
        outputs: vec![],
        inputs: vec![],
        analog_inputs: vec![
            Box::new(adc_channel(0, 480, 484, "AIN0")),
            Box::new(adc_channel(1, 481, 485, "AIN1")),
            Box::new(adc_channel(2, 482, 486, "AIN2")),
            Box::new(adc_channel(3, 483, 487, "AIN3")),
            Box::new(adc_channel(4, 488, 492, "AIN4")),
            Box::new(adc_channel(5, 489, 493, "AIN5")),
            Box::new(adc_channel(6, 490, 494, "AIN6")),
            Box::new(adc_channel(7, 491, 495, "AIN7")),
        ],
        analog_outputs: vec![],
        temp_sensors: vec![
            Box::new(rtd_channel(&sampler_rtd0, 0, "RTD0")),
            Box::new(rtd_channel(&sampler_rtd0, 1, "RTD1")),
            Box::new(rtd_channel(&sampler_rtd0, 2, "RTD2")),
            Box::new(rtd_channel(&sampler_rtd1, 0, "RTD3")),
            Box::new(rtd_channel(&sampler_rtd1, 1, "RTD4")),
            Box::new(rtd_channel(&sampler_rtd1, 2, "RTD5")),
            Box::new(tc0),
            Box::new(tc1),
            Box::new(tc2),
            Box::new(tc3),
        ],
        counter_input: vec![],
        relay_offset: None,
        pwm_outputs: vec![],
    };

    let shm_mapping = shm::Mappings {
        groups: vec![
            shm::Group {
                notifier: notifier_adc,
                channels: shm::Channels::AnalogInput(vec![0, 1, 2, 3, 4, 5, 6, 7]),
            },
            shm::Group {
                notifier: notifier_rtd0,
                channels: shm::Channels::TempInput(vec![0, 1, 2]),
            },
            shm::Group {
                notifier: notifier_rtd1,
                channels: shm::Channels::TempInput(vec![3, 4, 5]),
            },
            shm::Group {
                notifier: notifier_tc0,
                channels: shm::Channels::TempInput(vec![6, 7]),
            },
            shm::Group {
                notifier: notifier_tc1,
                channels: shm::Channels::TempInput(vec![8, 9]),
            },
        ],
    };

    (io, shm_mapping)
}
