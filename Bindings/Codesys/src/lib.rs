// SPDX-License-Identifier: LGPL-3.0-or-later
//
// (c) SYSTEC electronic AG, D-08468 Heinsdorfergrund, Am Windrad 2
//     www.systec-electronic.com

use anyhow::{bail, Result};
use askama::Template;
use log::error;
use packed_struct;
use packed_struct::prelude::*;
use std::collections::HashMap;
use std::str;

#[derive(Template)]
#[template(path = "../templates/device_template_codesys.xml", escape = "none")]
pub struct DeviceXmlTemplate<'a> {
    pub model_name: &'a str,
    pub device_description: &'a str,
    pub vendor_name: &'a str,
    pub type_name: &'a str,
    pub type_description: &'a str,
    pub device_type: usize,
    pub device_id: &'a str,
    pub device_version: &'a str,
    pub order_number: &'a str,
    pub io_interfaces: &'a str,
}

pub enum ChannelType {
    DigitalInputs,
    ConfigSwitch,
    RunSwitch,
    AnalogInputs,
    DigitalOutputs,
    RunLed,
    ErrorLed,
    AnalogInputsMode,
    DigitalOutputsMask,
    RunLedMask,
    ErrorLedMask,
    AnalogInputsModeMask,
}

impl ChannelType {
    // CODESYS index definitions
    pub fn get_index_base(&self) -> usize {
        match self {
            // All inputs start at index 1000
            ChannelType::DigitalInputs => 1000 as usize,
            ChannelType::ConfigSwitch => 1100 as usize,
            ChannelType::RunSwitch => 1101 as usize,
            ChannelType::AnalogInputs => 1200 as usize,
            // All outputs start at index 2000
            ChannelType::DigitalOutputs => 2000 as usize,
            ChannelType::RunLed => 2100 as usize,
            ChannelType::ErrorLed => 2101 as usize,
            ChannelType::AnalogInputsMode => 2200 as usize,
            // DO masks
            ChannelType::DigitalOutputsMask => 2500 as usize,
            ChannelType::RunLedMask => 2600 as usize,
            ChannelType::ErrorLedMask => 2601 as usize,
            ChannelType::AnalogInputsModeMask => 2700 as usize,
        }
    }

    pub fn get_data_type(&self) -> String {
        match self {
            ChannelType::AnalogInputs => "UINT".to_owned(),
            _ => "BOOL".to_owned(),
        }
    }

    pub fn get_channel_type(index: usize) -> Result<Self> {
        match index {
            // Inputs
            (1000..=1099) => Ok(ChannelType::DigitalInputs),
            1100 => Ok(ChannelType::ConfigSwitch),
            1101 => Ok(ChannelType::RunSwitch),
            (1200..=1299) => Ok(ChannelType::AnalogInputs),
            // Outputs
            (2000..=2099) => Ok(ChannelType::DigitalOutputs),
            2100 => Ok(ChannelType::RunLed),
            2101 => Ok(ChannelType::ErrorLed),
            (2200..=2299) => Ok(ChannelType::AnalogInputsMode),
            // Outputs mask
            (2500..=2599) => Ok(ChannelType::DigitalOutputsMask),
            2600 => Ok(ChannelType::RunLedMask),
            2601 => Ok(ChannelType::ErrorLedMask),
            (2700..=2799) => Ok(ChannelType::AnalogInputsModeMask),
            _ => bail!("Cannot match index {index} to a channel type!"),
        }
    }
}

#[derive(PackedStruct, Debug)]
#[packed_struct(size_bytes = "12", bit_numbering = "msb0", endian = "lsb")]
pub struct ReceiveMessageHeader {
    #[packed_field(bytes = "0..=3")]
    pub msg_id: u32,
    #[packed_field(bytes = "4..=7")]
    pub msg_type: i32,
    #[packed_field(bytes = "8..=11")]
    pub data_size: u32,
}

#[derive(PackedStruct, Debug)]
#[packed_struct(size_bytes = "16", bit_numbering = "msb0", endian = "lsb")]
pub struct ResponseMessageHeader {
    #[packed_field(bytes = "0..=3")]
    pub msg_id: u32,
    #[packed_field(bytes = "4..=7")]
    pub msg_type: i32,
    #[packed_field(bytes = "8..=11")]
    pub data_size: u32,
    #[packed_field(bytes = "12..=15")]
    pub error: u32,
}

#[derive(Default, Debug)]
pub struct ProcessImage {
    pub data: HashMap<usize, PlcValue>,
}

// Incomming data types are always USINT.
// Accoring to the documentation, outgoing values
// must always be of type USINT, UINT, UDINT or ULINT.
// Other values should be "casted" to match these types.
#[derive(Debug)]
pub enum PlcValue {
    USINT(u8),
    UINT(u16),
    UDINT(u32),
    ULINT(u64),
}

impl PlcValue {
    pub fn from(data_type: &str, data_value: &str) -> Result<PlcValue> {
        match data_type {
            "USINT" => Ok(PlcValue::USINT(data_value.parse::<u8>()?)),
            "UINT" => Ok(PlcValue::UINT(data_value.parse::<u16>()?)),
            "UDINT" => Ok(PlcValue::UDINT(data_value.parse::<u32>()?)),
            "ULINT" => Ok(PlcValue::ULINT(data_value.parse::<u64>()?)),
            _ => bail!("Unsuported data type!"),
        }
    }

    pub fn to_command_string(&self, index: &usize) -> String {
        match *self {
            PlcValue::USINT(v) => format!("{index}:=USINT#{v}\0"),
            PlcValue::UINT(v) => format!("{index}:=UINT#{v}\0"),
            PlcValue::UDINT(v) => format!("{index}:=UDINT#{v}\0"),
            PlcValue::ULINT(v) => format!("{index}:=ULINT#{v}\0"),
        }
    }
}

impl ProcessImage {
    pub fn decode(raw_data: &[u8]) -> Result<Self> {
        let mut data: HashMap<usize, PlcValue> = HashMap::new();
        let decoded_data: Vec<&str> = str::from_utf8(&raw_data)?
            .split("\0")
            .filter(|&d| !d.is_empty())
            .collect();

        for entry in decoded_data {
            // Data is structured as index:=type#value
            let mut slice = entry.split(":=").flat_map(|s| s.split("#"));
            let data_index = slice.next();
            let data_type = slice.next();
            let data_value = slice.next();

            if let (Some(index), Some(ty), Some(value)) = (data_index, data_type, data_value) {
                match index.parse::<usize>() {
                    Ok(index) => match PlcValue::from(ty, value) {
                        Ok(value) => data.insert(index, value),
                        Err(_) => {
                            error!("Error - invalid type/value: {ty}/{value}");
                            continue;
                        }
                    },
                    Err(_) => {
                        error!("Error - cannot decode data index: {index}!");
                        continue;
                    }
                };
            } else {
                error!("Error - cannot decode command: {entry}");
            }
        }

        Ok(Self { data })
    }

    pub fn encode(&self) -> Result<String> {
        let mut encoded_data: String = String::new();
        for (index, plc_io_data) in &self.data {
            let string_line = plc_io_data.to_command_string(index);
            encoded_data.push_str(&string_line);
        }
        Ok(encoded_data)
    }
}
