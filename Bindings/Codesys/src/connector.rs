// SPDX-License-Identifier: LGPL-3.0-or-later
// SPDX-FileCopyrightText: 2025 SYS TEC electronic AG <https://www.systec-electronic.com/>

use anyhow::{bail, Result};
use codesys_connector::*;
use log::{debug, error};
use packed_struct::PackedStruct;
use std::collections::HashMap;
use std::convert::TryInto;
use std::io::{Read, Write};
use std::os::unix::net::{UnixListener, UnixStream};
use std::str;
use std::time::Duration;
use std::vec;
use sysworxx_io::ffi::IoAnalogMode;
use sysworxx_io::{hw_rev, Io, IoChannel};

const SOCK_DIR: &str = "/var/run/codesysextension/extfuncs";
const SOCK_ADDR: &str = "/var/run/codesysextension/extfuncs/UDS_IODriver_0.sock";

fn log_err<T, E: std::fmt::Display>(res: std::result::Result<T, E>) {
    if let Err(e) = res {
        error!("Error - {e}");
    }
}

pub fn open_plc_connection() -> Result<()> {
    std::fs::create_dir_all(SOCK_DIR)?;
    std::fs::remove_file(SOCK_ADDR).ok();
    let listener = UnixListener::bind(SOCK_ADDR)?;

    let mut io = sysworxx_io::definition::load_device_definition(
        &hw_rev::get_device_name().unwrap_or("fallback".to_string()),
    );

    match io.init() {
        Ok(()) => {}
        Err(e) => {
            error!("Failed to initialize: {}", e);
            std::process::exit(1);
        }
    };

    loop {
        let (stream, _) = listener.accept()?;
        debug!("Open connection");
        handle_connection(&mut io, stream)?;
    }
}

fn handle_connection(io: &mut Io, mut stream: UnixStream) -> Result<()> {
    // Data buffer of 1024 size may be too small in the future.
    // The header is always 12 bytes and each output
    // needs at least 14 bytes.
    // E.g. on CTR-800 we have 48 outputs -> needs at least
    // 672 + 12 for the header = 684 bytes.
    let mut data_buffer: Vec<u8> = vec![0; 1024];
    let mut process_image = ProcessImage {
        data: HashMap::new(),
    };

    let output_indices = setup_output_indices(io)?;

    stream.set_read_timeout(Some(Duration::new(1000, 00)))?;
    stream.set_write_timeout(Some(Duration::new(1000, 00)))?;

    let io_channel_info = io.get_channel_info();
    let vector_digi_inputs = get_indices(io_channel_info.inputs);
    let vector_analog_inputs = get_indices(io_channel_info.analog_inputs);
    let run_switch_exists = io_channel_info.run_switch.label().is_some();
    let config_switch_exists = io_channel_info.config_switch.label().is_some();

    loop {
        let read_size = match stream.read(&mut data_buffer) {
            Ok(read_size) => read_size,
            Err(e) => {
                error!("Error - {e}");
                break;
            }
        };

        const HEADER_SIZE: usize = std::mem::size_of::<ReceiveMessageHeader>();
        if read_size >= HEADER_SIZE {
            let recv_header = match ReceiveMessageHeader::unpack(
                data_buffer[..HEADER_SIZE]
                    .try_into()
                    .expect("convert header slice to array"),
            ) {
                Ok(v) => v,
                Err(e) => {
                    error!("Invalid header value received - {e}");
                    continue;
                }
            };

            let mut resp_data: String = String::new();
            // According to the example by CODESYS, the communication begins with sending data size 0 and error set to 1.
            // After the initial message, data is greater than 0 and error set to 0.
            let mut error = 1;

            if recv_header.data_size > 0 {
                error = 0;
                process_image = match ProcessImage::decode(&data_buffer[HEADER_SIZE..read_size]) {
                    Ok(process_image) => process_image,
                    Err(e) => {
                        error!("Invalid command data received - {e}");
                        continue;
                    }
                };

                debug!("received data: {:?}", process_image.data);
                apply_process_image(&process_image, &output_indices, io).ok();

                let res = get_inputs(
                    io,
                    &vector_digi_inputs,
                    &vector_analog_inputs,
                    run_switch_exists,
                    config_switch_exists,
                )
                .and_then(|v| v.encode());
                resp_data = match res {
                    Ok(resp_data) => resp_data,
                    Err(e) => {
                        error!("Could not get input values - {e}");
                        continue;
                    }
                }
            }

            let res_response = ResponseMessageHeader {
                msg_id: recv_header.msg_id,
                msg_type: recv_header.msg_type,
                data_size: resp_data.len() as u32,
                error,
            }
            .pack();

            let mut resp_msg: Vec<u8> = match res_response {
                Ok(resp_msg) => resp_msg.into(),
                Err(e) => {
                    error!("Could not generate response message - {e}");
                    continue;
                }
            };

            // According to the example by CODESYS, the message data (device input interfaces) need to be
            // appended to the message buffer - it may also vary in length.
            resp_msg.append(&mut resp_data.as_bytes().into());

            match stream.write(&resp_msg) {
                Ok(_) => (),
                Err(e) => {
                    error!("Error - {e}");
                }
            };
        } else {
            break;
        }
    }

    debug!("Close connection");
    disable_active_outputs(&process_image, &output_indices, io);
    Ok(())
}

fn apply_process_image(
    process_image: &ProcessImage,
    output_indices: &Vec<(usize, usize)>,
    io: &mut Io,
) -> Result<()> {
    // Check mask and only change states, if set to true
    for (output_index, mask_index) in output_indices {
        match process_image.data.get(&mask_index) {
            Some(PlcValue::USINT(1)) => {
                // Digital outputs are always sent as type USINT
                let state = match process_image.data.get(&output_index) {
                    Some(PlcValue::USINT(1)) => true,
                    _ => false,
                };

                set_output(*output_index, io, state);
            }
            _ => continue,
        }
    }

    Ok(())
}

fn setup_output_indices(io: &mut Io) -> Result<Vec<(usize, usize)>> {
    let mut output_indices: Vec<(usize, usize)> = vec![];
    let io_channel_info = io.get_channel_info();

    // Digital Outputs
    for (channel_offset, _) in io_channel_info.outputs.iter().enumerate() {
        let output_index = channel_offset + ChannelType::DigitalOutputs.get_index_base();
        let mask_index = channel_offset + ChannelType::DigitalOutputsMask.get_index_base();
        output_indices.push((output_index, mask_index));
    }

    // Analog mode switch outputs
    for (channel_offset, _) in io_channel_info.analog_inputs.iter().enumerate() {
        let output_index = channel_offset + ChannelType::AnalogInputsMode.get_index_base();
        let mask_index = channel_offset + ChannelType::AnalogInputsModeMask.get_index_base();
        output_indices.push((output_index, mask_index));
    }

    // Run LED
    let output_index = ChannelType::RunLed.get_index_base();
    let mask_index = ChannelType::RunLedMask.get_index_base();
    output_indices.push((output_index, mask_index));

    // Err LED
    let output_index = ChannelType::ErrorLed.get_index_base();
    let mask_index = ChannelType::ErrorLedMask.get_index_base();
    output_indices.push((output_index, mask_index));

    output_indices.sort();

    Ok(output_indices)
}

fn set_output(index: usize, io: &mut Io, state: bool) {
    match ChannelType::get_channel_type(index) {
        Ok(channel_type) => match channel_type {
            ChannelType::DigitalOutputs => {
                let channel = index - ChannelType::DigitalOutputs.get_index_base();
                log_err(io.output_set(channel, state));
            }
            ChannelType::RunLed => {
                log_err(io.set_run_led(state));
            }
            ChannelType::ErrorLed => {
                log_err(io.set_err_led(state));
            }
            ChannelType::AnalogInputsMode => {
                let channel = index - ChannelType::AnalogInputsMode.get_index_base();
                let ai_mode = match state {
                    true => IoAnalogMode::Current,
                    false => IoAnalogMode::Voltage,
                };

                log_err(io.analog_mode_set(channel, ai_mode));
            }
            _ => (),
        },
        Err(e) => {
            error!("{e}");
        }
    }
}

fn get_inputs(
    io: &mut Io,
    vector_digi_inputs: &Vec<usize>,
    vector_analog_inputs: &Vec<usize>,
    run_switch_exists: bool,
    config_switch_exists: bool,
) -> Result<ProcessImage> {
    let mut plc_response: ProcessImage = ProcessImage::default();
    // Digital Inputs
    for channel_offset in vector_digi_inputs {
        log_err(
            get_input(io, ChannelType::DigitalInputs, *channel_offset)
                .map(|(index, plc_data)| plc_response.data.insert(index, plc_data)),
        )
    }

    // Analog Inputs
    for channel_offset in vector_analog_inputs {
        log_err(
            get_input(io, ChannelType::AnalogInputs, *channel_offset)
                .map(|(index, plc_data)| plc_response.data.insert(index, plc_data)),
        )
    }

    // Run-Switch
    if run_switch_exists {
        log_err(
            get_input(io, ChannelType::RunSwitch, 0)
                .map(|(index, plc_data)| plc_response.data.insert(index, plc_data)),
        );
    }

    // Config-Switch
    if config_switch_exists {
        log_err(
            get_input(io, ChannelType::ConfigSwitch, 0)
                .map(|(index, plc_data)| plc_response.data.insert(index, plc_data)),
        );
    }

    Ok(plc_response)
}

fn get_input(
    io: &mut Io,
    channel_type: ChannelType,
    channel_offset: usize,
) -> Result<(usize, PlcValue)> {
    let base = channel_type.get_index_base();
    let current_index = base + channel_offset;

    fn eval_digi_in(val: bool) -> Result<PlcValue> {
        Ok(PlcValue::USINT(val.into()))
    }

    let data_value = match channel_type {
        ChannelType::DigitalInputs => eval_digi_in(io.input_get(channel_offset)?)?,
        ChannelType::RunSwitch => eval_digi_in(io.get_run_switch()?)?,
        ChannelType::ConfigSwitch => eval_digi_in(io.get_config_switch()?)?,
        ChannelType::AnalogInputs => {
            let v = io.analog_input_get(channel_offset)?;
            PlcValue::UINT(v as u16)
        }
        _ => bail!("Unsupported channel type!"),
    };

    // Append each input state to response
    Ok((current_index, data_value))
}

fn get_indices<T: IoChannel + ?Sized>(channels: &[Box<T>]) -> Vec<usize> {
    let mut indices: Vec<usize> = vec![];
    for (i, channel) in channels.iter().enumerate() {
        if let Some(_) = channel.label() {
            indices.push(i);
        }
    }
    indices
}

fn disable_active_outputs(
    process_image: &ProcessImage,
    output_indices: &Vec<(usize, usize)>,
    io: &mut Io,
) {
    for (do_index, mask_index) in output_indices {
        match process_image.data.get(&mask_index) {
            Some(PlcValue::USINT(1)) => {
                set_output(*do_index, io, false);
            }
            _ => continue,
        }
    }
}
