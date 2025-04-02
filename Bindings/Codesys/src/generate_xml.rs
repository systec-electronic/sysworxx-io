// SPDX-License-Identifier: LGPL-3.0-or-later
//
// (c) SYSTEC electronic AG, D-08468 Heinsdorfergrund, Am Windrad 2
//     www.systec-electronic.com

use anyhow::{bail, Result};
use askama::Template;
use codesys_connector::*;
use std::fs::{self, File};
use std::str;
use sysworxx_io::{definition, hw_rev, Io, IoChannel};

const PATH_FW_MODEL: &str = "/sys/firmware/devicetree/base/model";

pub fn generate_xml(path: &str) -> Result<()> {
    let device_name = &hw_rev::get_device_name().unwrap_or("fallback".to_string());
    let mut io = definition::load_device_definition(device_name);

    let io_interfaces = get_interfaces_xml_string(&mut io)
        .expect("Could not get interface information for device!");
    let io_interfaces = io_interfaces.as_str();

    let type_name = fs::read_to_string(PATH_FW_MODEL).unwrap();
    let type_name = type_name
        .split_terminator('\0')
        .next()
        .unwrap_or("Unkown Device");

    let model_name = format!("{type_name} IO Module");

    // FixMe: A few entries need to be device specific - store this new information where?
    let xml_template_information = DeviceXmlTemplate {
            model_name: model_name.as_str(),
            device_description: "Unix domain socket IO modulefor sysWORXX Devices",
            vendor_name: "SYS TEC electronic AG",
            type_name,
            type_description: "A device for CODESYS to use a unix domain socket to exchange IO data with a sysWORXX device.",
            device_type: 0,
            device_id: "1001 0001",
            device_version: "1.0.0.0",
            order_number: "0",
            io_interfaces
        };

    let path = format!("{path}/sysworxx-{device_name}.devdesc.xml");
    let mut file = File::create(path).expect("Error creating file to write information!");
    xml_template_information
        .write_into(&mut file)
        .expect("Could not write file to device!");
    Ok(())
}

fn get_interfaces_xml_string(io: &mut Io) -> Result<String> {
    fn build_xml_string_single<T: IoChannel + ?Sized>(
        channel: &T,
        index: usize,
        channel_type: &ChannelType,
        interface_direction: &str,
    ) -> Result<String> {
        if let Some(label) = channel.label() {
            let mut label = String::from(label);
            let base = channel_type.get_index_base();
            let data_type = channel_type.get_data_type();

            // Adjust written message for analog input and mode
            match channel_type {
                ChannelType::DigitalOutputsMask => label = format!("{label}_ENABLE"),
                ChannelType::RunLedMask => label = format!("{label}_ENABLE"),
                ChannelType::ErrorLedMask => label = format!("{label}_ENABLE"),
                ChannelType::AnalogInputsMode => label = format!("AIN{index}_CURRENT_MODE"),
                ChannelType::AnalogInputsModeMask => {
                    label = format!("AIN{index}_CURRENT_MODE_ENABLE")
                }
                _ => (),
            }

            Ok(format!(
                "
                        <Parameter ParameterId=\"{}\" type=\"std:{}\">
                            <Attributes channel=\"{}\" />
                            <Default />
                            <Name name=\"local:{}\">{}</Name>
                        </Parameter>",
                (base + index),
                data_type,
                interface_direction,
                label,
                label,
            ))
        } else {
            bail!("Could not generate entry from label!")
        }
    }

    fn build_interface_string_vector<T: IoChannel + ?Sized>(
        channel_type: ChannelType,
        interface_direction: &str,
        channels: &[Box<T>],
    ) -> String {
        let mut io_interfaces = String::new();
        for (index, channel) in channels.iter().enumerate() {
            let io_entry = match build_xml_string_single(
                channel.as_ref(),
                index,
                &channel_type,
                interface_direction,
            ) {
                Ok(entry) => entry,
                Err(_) => continue,
            };

            io_interfaces.push_str(&io_entry);
        }
        io_interfaces
    }

    let io_channel_info = io.get_channel_info();

    // Generate all the IO's into one XML string
    let mut io_interfaces = String::new();
    io_interfaces.push_str(&build_interface_string_vector(
        ChannelType::DigitalInputs,
        "input",
        io_channel_info.inputs,
    ));
    io_interfaces.push_str(&build_interface_string_vector(
        ChannelType::AnalogInputs,
        "input",
        io_channel_info.analog_inputs,
    ));

    if let Ok(entry) = build_xml_string_single(
        io_channel_info.config_switch,
        0,
        &ChannelType::ConfigSwitch,
        "input",
    ) {
        io_interfaces.push_str(&entry);
    }

    // Run-Switch is already added in digital input vector
    io_interfaces.push_str(&build_interface_string_vector(
        ChannelType::DigitalOutputs,
        "output",
        io_channel_info.outputs,
    ));

    if let Ok(entry) =
        build_xml_string_single(io_channel_info.run_led, 0, &ChannelType::RunLed, "output")
    {
        io_interfaces.push_str(&entry);
    };

    if let Ok(entry) =
        build_xml_string_single(io_channel_info.err_led, 0, &ChannelType::ErrorLed, "output")
    {
        io_interfaces.push_str(&entry);
    };

    io_interfaces.push_str(&build_interface_string_vector(
        ChannelType::AnalogInputsMode,
        "output",
        io_channel_info.analog_inputs,
    ));

    // DO masks insert
    io_interfaces.push_str(&build_interface_string_vector(
        ChannelType::DigitalOutputsMask,
        "output",
        io_channel_info.outputs,
    ));

    if let Ok(entry) = build_xml_string_single(
        io_channel_info.run_led,
        0,
        &ChannelType::RunLedMask,
        "output",
    ) {
        io_interfaces.push_str(&entry);
    };

    if let Ok(entry) = build_xml_string_single(
        io_channel_info.err_led,
        0,
        &ChannelType::ErrorLedMask,
        "output",
    ) {
        io_interfaces.push_str(&entry);
    };

    io_interfaces.push_str(&build_interface_string_vector(
        ChannelType::AnalogInputsModeMask,
        "output",
        io_channel_info.analog_inputs,
    ));

    Ok(io_interfaces)
}
