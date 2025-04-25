// SPDX-License-Identifier: LGPL-3.0-or-later
// SPDX-FileCopyrightText: 2025 SYS TEC electronic AG <https://www.systec-electronic.com/>

use std::fs;

use crate::error::*;

const PATH_FW_COMPATIBLE: &str = "/sys/firmware/devicetree/base/compatible";

pub fn get_device_name() -> Result<String> {
    let compatibles = fs::read_to_string(PATH_FW_COMPATIBLE)?;
    let devices = ["ctr", "pi"];
    for device in devices {
        if let Some(device) = decode_compatibles(&compatibles, device) {
            return Ok(device);
        }
    }
    Err(Error::GenericError)
}

fn decode_compatibles(compatibles: &str, prefix: &str) -> Option<String> {
    let first_compatible = compatibles.split_terminator('\0').next().unwrap_or("");
    let split_compatible: Vec<&str> = first_compatible.split(',').collect();
    let device_name = split_compatible.iter().find(|&s| s.contains(prefix));
    device_name.map(|device_name| device_name.to_string())
}

pub fn get_hardware_revision() -> Result<u8> {
    let compatibles = fs::read_to_string(PATH_FW_COMPATIBLE)?;
    decode_hw_revision(compatibles)
}

fn decode_hw_revision(str: String) -> Result<u8> {
    let first_compatible = str.split_terminator('\0').next().unwrap_or("");
    first_compatible
        .rsplit_terminator(",rev")
        .next()
        .unwrap_or("")
        .parse::<u8>()
        .map_err(|_| Error::GenericError)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn decode_test() {
        let rev = decode_hw_revision("systec,ctr700,rev0".to_string());
        assert_eq!(rev, Ok(0));

        let rev = decode_hw_revision("systec,ctr700,rev1\0systec,ctr700".to_string());
        assert_eq!(rev, Ok(1));

        let rev = decode_hw_revision("systec,ctr7002\0systec,ctr700".to_string());
        assert_eq!(rev, Err(Error::GenericError));
    }
}
