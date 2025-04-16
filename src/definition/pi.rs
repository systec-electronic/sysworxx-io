// SPDX-License-Identifier: LGPL-3.0-or-later
// SPDX-FileCopyrightText: 2025 SYS TEC electronic AG <https://www.systec-electronic.com/>

use std::collections::HashMap;
use std::time::Duration;

use crate::io;
use crate::io::lookup::Lookup;
use crate::io::{null, sensors};
use crate::labeled::Labeled;
use crate::{DigitalInput, DigitalOutput, Io};

use evdev;

const SYSWORXX_DTS_PREFIX: &str = "sysworxx-";

type DtsKeyCode = u32;
const SUPPORTED_KEY_CODES: [evdev::KeyCode; 16] = [
    evdev::KeyCode::BTN_TRIGGER_HAPPY1,
    evdev::KeyCode::BTN_TRIGGER_HAPPY2,
    evdev::KeyCode::BTN_TRIGGER_HAPPY3,
    evdev::KeyCode::BTN_TRIGGER_HAPPY4,
    evdev::KeyCode::BTN_TRIGGER_HAPPY5,
    evdev::KeyCode::BTN_TRIGGER_HAPPY6,
    evdev::KeyCode::BTN_TRIGGER_HAPPY7,
    evdev::KeyCode::BTN_TRIGGER_HAPPY8,
    evdev::KeyCode::BTN_TRIGGER_HAPPY9,
    evdev::KeyCode::BTN_TRIGGER_HAPPY10,
    evdev::KeyCode::BTN_TRIGGER_HAPPY11,
    evdev::KeyCode::BTN_TRIGGER_HAPPY12,
    evdev::KeyCode::BTN_TRIGGER_HAPPY13,
    evdev::KeyCode::BTN_TRIGGER_HAPPY14,
    evdev::KeyCode::BTN_TRIGGER_HAPPY15,
    evdev::KeyCode::BTN_TRIGGER_HAPPY16,
];

fn build_gpio_input_labels(
    input_device_name: &str,
) -> Result<HashMap<DtsKeyCode, String>, std::io::Error> {
    let dt_path = std::path::Path::new("/sys/firmware/devicetree/base/").join(input_device_name);
    let read_dir = std::fs::read_dir(dt_path)?;

    let mut labels = HashMap::new();
    for entry in read_dir {
        let entry = entry?;
        let metadata = entry.metadata()?;

        if !metadata.is_dir() {
            continue;
        }

        let keycode = std::fs::read(entry.path().join("linux,code"))?;
        let label = std::fs::read_to_string(entry.path().join("label"))?
            .trim_end_matches('\0')
            .to_string();

        for supported_keycode in SUPPORTED_KEY_CODES {
            if (supported_keycode.code() as DtsKeyCode).to_be_bytes() == keycode.as_slice() {
                labels.insert(supported_keycode.code() as DtsKeyCode, label);
                break;
            }
        }
    }

    Ok(labels)
}

/// Build inputs for all 'gpio-keys' instances in base device tree node starting with 'sysworxx-'
/// with stable order.
fn build_sysworxx_pi_inputs() -> Vec<Box<dyn DigitalInput>> {
    let mut input_device_names: Vec<_> = evdev::enumerate()
        .filter_map(|(_, d)| d.name().map(ToString::to_string))
        .filter(|name| name.starts_with(SYSWORXX_DTS_PREFIX))
        .collect();
    input_device_names.sort();

    let build_name = |labels: &HashMap<_, _>, key: evdev::KeyCode| -> String {
        labels
            .get(&(key.code() as DtsKeyCode))
            .map(ToString::to_string)
            .unwrap_or_else(|| format!("INPUT_{:08x}", key.code() as DtsKeyCode))
    };

    let mut inputs = vec![];
    for input_device_name in input_device_names {
        let mut collector = io::evdev::EvdevCollector::from_name(&input_device_name)
            .expect("build evdev collector");
        let labels = build_gpio_input_labels(&input_device_name).unwrap_or_default();

        for key in SUPPORTED_KEY_CODES {
            if collector.supports_key(key) {
                inputs.push(Box::new(Labeled::new(
                    String::leak(build_name(&labels, key)),
                    io::evdev::Di::new(&mut collector, key),
                )) as Box<dyn DigitalInput>);
            }
        }
    }

    inputs
}

/// Build outputs for RGB-LED and all gpio-chips ('gpio-aggregator') with name 'sysworxx-' with
/// stable order.
fn build_sysworxx_pi_outputs() -> Vec<Box<dyn DigitalOutput>> {
    // TODO: simplify this when transitioning to character devices
    let lookup: Lookup = Lookup::new();
    let mut outputs = vec![
        lookup.gpio_do("LED_BL", "600000.gpio", 11) as Box<dyn DigitalOutput>,
        lookup.gpio_do("LED_RD", "600000.gpio", 12) as Box<dyn DigitalOutput>,
        lookup.gpio_do("LED_GN", "600000.gpio", 14) as Box<dyn DigitalOutput>,
    ];

    let mut chips: Vec<_> = gpio_cdev::chips()
        .expect("chip iterator")
        .filter_map(|c| c.ok())
        .filter(|c| c.label().starts_with(SYSWORXX_DTS_PREFIX))
        .collect();
    chips.sort_by_key(|c| c.label().to_string());

    let lines: Vec<(String, Vec<String>)> = chips
        .into_iter()
        .map(|c| {
            let lines = c
                .lines()
                .filter_map(|l| l.info().ok())
                .filter_map(|l_info| l_info.name().map(ToString::to_string))
                .collect::<Vec<_>>();
            (c.label().to_string(), lines)
        })
        .collect();

    for (chip_name, line_names) in lines.into_iter() {
        for (i, line_name) in line_names.into_iter().enumerate() {
            outputs.push(lookup.gpio_do(
                String::leak(line_name),
                String::leak(chip_name.clone()),
                i,
            ));
        }
    }

    outputs
}

pub fn definition() -> Io {
    let tmp0 = Box::new(Labeled::new(
        "CPU",
        sensors::LmSensor::new("main1_thermal-virtual-0", Duration::from_millis(2000)),
    ));

    Io {
        watchdog: Box::new(null::Wdg::new()),
        run_led: Box::new(null::Output::not_implemented()),
        err_led: Box::new(null::Output::not_implemented()),
        run_switch: Box::new(null::Input::not_implemented()),
        config_switch: Box::new(null::Input::not_implemented()),
        outputs: build_sysworxx_pi_outputs(),
        inputs: build_sysworxx_pi_inputs(),
        analog_inputs: vec![],
        analog_outputs: vec![],
        temp_sensors: vec![tmp0],
        counter_input: vec![],
        relay_offset: None,
        pwm_outputs: vec![],
    }
}
