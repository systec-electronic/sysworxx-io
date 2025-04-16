// SPDX-License-Identifier: LGPL-3.0-or-later
// SPDX-FileCopyrightText: 2025 SYS TEC electronic AG <https://www.systec-electronic.com/>

mod ctr500;
mod ctr600;
mod ctr700;
mod ctr750;
mod ctr800;
mod fallback;
mod pi;

use crate::shm;
use crate::Io;

use std::collections::HashMap;

pub fn load_device_definition(device: &str) -> Io {
    let mut definitions_map = HashMap::from([
        ("ctr500", ctr500::definition as fn() -> Io),
        ("ctr600", ctr600::definition as fn() -> Io),
        ("ctr700", ctr700::definition as fn() -> Io),
        ("ctr750", ctr750::definition as fn() -> Io),
        ("ctr800", ctr800::definition as fn() -> Io),
        ("pi", pi::definition as fn() -> Io),
    ]);

    match definitions_map.remove(device) {
        Some(entry) => entry(),
        None => fallback::definition(),
    }
}

pub fn load_device_definition_shm(device: &str) -> Option<(Io, shm::Mappings)> {
    let mut definitions_map = HashMap::from([
        (
            "ctr700",
            ctr700::definition_shm as fn() -> (Io, shm::Mappings),
        ),
        (
            "ctr750",
            ctr750::definition_shm as fn() -> (Io, shm::Mappings),
        ),
        (
            "ctr800",
            ctr800::definition_shm as fn() -> (Io, shm::Mappings),
        ),
    ]);

    match definitions_map.remove(device) {
        Some(entry) => Some(entry()),
        None => fallback::definition_shm(),
    }
}
