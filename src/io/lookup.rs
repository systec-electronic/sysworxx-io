// SPDX-License-Identifier: LGPL-3.0-or-later
// SPDX-FileCopyrightText: 2025 SYS TEC electronic AG <https://www.systec-electronic.com/>

use crate::error::Result;
use crate::io::sysfs;
use crate::labeled::Labeled;
use std::collections::HashMap;
use std::fs;
use std::path::Path;

pub enum LogicLevel {
    ActiveLow,
    ActiveHigh,
}

pub struct Lookup {
    gpio_map: HashMap<String, usize>,
}

impl Lookup {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Lookup {
        Self {
            gpio_map: Self::lookup_bases().unwrap(),
        }
    }

    fn lookup_bases() -> Result<HashMap<String, usize>> {
        let mut gpio_map: HashMap<String, usize> = HashMap::new();

        for entry in Path::new("/sys/class/gpio/").read_dir()? {
            let entry = entry?;

            if entry
                .file_name()
                .to_str()
                .is_some_and(|p: &str| p.contains("gpiochip"))
            {
                let label = entry.path().join("label");
                let label = fs::read_to_string(&label)?.trim().to_string();

                let base = entry.path().join("base");
                let base = fs::read_to_string(&base)?.trim().parse::<usize>()?;

                gpio_map.insert(label, base);
            }
        }

        Ok(gpio_map)
    }

    pub fn gpio_do(
        &self,
        gpio_label: &'static str,
        gpio_chip: &'static str,
        gpio_pin_offset: usize,
    ) -> Box<Labeled<sysfs::Do>> {
        let gpio_pin = *self.gpio_map.get(gpio_chip).unwrap() + gpio_pin_offset;
        Box::new(Labeled::new(gpio_label, sysfs::Do::new(gpio_pin)))
    }

    pub fn gpio_di(
        &self,
        gpio_label: &'static str,
        gpio_chip: &'static str,
        gpio_pin_offset: usize,
        logic_level: LogicLevel,
    ) -> Box<Labeled<sysfs::Di>> {
        let gpio_pin = *self.gpio_map.get(gpio_chip).unwrap() + gpio_pin_offset;
        match logic_level {
            LogicLevel::ActiveLow => {
                Box::new(Labeled::new(gpio_label, sysfs::Di::active_low(gpio_pin)))
            }
            LogicLevel::ActiveHigh => Box::new(Labeled::new(gpio_label, sysfs::Di::new(gpio_pin))),
        }
    }

    pub fn gpio_pair_adc(&self, gpio_chip: &'static str, gpio_pin_offset: usize) -> (usize, usize) {
        let gpio_pin_v = *self.gpio_map.get(gpio_chip).unwrap() + gpio_pin_offset;
        let gpio_pin_i = gpio_pin_v + 4; // pin for current is 4 digits higher
        (gpio_pin_v, gpio_pin_i)
    }
}
