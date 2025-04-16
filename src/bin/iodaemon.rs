// SPDX-License-Identifier: LGPL-3.0-or-later
// SPDX-FileCopyrightText: 2025 SYS TEC electronic AG <https://www.systec-electronic.com/>

#[macro_use]
extern crate log;

extern crate crossbeam_channel;
extern crate libc;
extern crate signal_hook;

use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

use raw_sync::Timeout;

use sysworxx_io::hw_rev;
use sysworxx_io::shm;
use sysworxx_io::signal;

#[derive(Debug)]
enum ValueChanged {
    Ain(usize, i64),
    Temp(usize, f64),
    Flush,
}

pub fn main() {
    use env_logger::Env;
    let env = Env::new().filter("IO_LOG").write_style("IO_LOG_STYLE");
    env_logger::init_from_env(env);

    println!("=======================================================");
    println!("=                                                     =");
    println!("=     SYS TEC electronic AG                           =");
    println!("=     D-08468 Heinsdorfergrund, Am Windrad 2          =");
    println!("=     www.systec-electronic.com                       =");
    println!("=                                                     =");
    println!(
        "=     sysWORXX I/O daemon, Version {}.{}.{}              =",
        env!("CARGO_PKG_VERSION_MAJOR").parse::<u8>().unwrap(),
        env!("CARGO_PKG_VERSION_MINOR").parse::<u8>().unwrap(),
        env!("CARGO_PKG_VERSION_PATCH").parse::<u8>().unwrap()
    );
    println!("=                                                     =");
    println!("=     (c) 2025 SYS TEC electronic AG                  =");
    println!("=                                                     =");
    println!("=======================================================");

    let (mut io, mut mappings) = match sysworxx_io::definition::load_device_definition_shm(
        &hw_rev::get_device_name().unwrap_or("fallback".to_string()),
    ) {
        Some(e) => e,
        None => {
            eprintln!("Device does not need/support iodaemon!");
            std::process::exit(0);
        }
    };

    match io.init() {
        Ok(()) => {}
        Err(e) => {
            eprintln!("Failed to initialize: {}", e);
            std::process::exit(1);
        }
    }

    let io = Arc::new(Mutex::new(io));

    let mut shm_server = match shm::ShmServer::new() {
        Ok(shared) => shared,
        Err(e) => {
            eprintln!("Failed to create/open shared memory: {}", e);
            std::process::exit(1);
        }
    };

    let (tx, rx) = crossbeam_channel::unbounded();

    let count_adc: usize = mappings
        .groups
        .iter()
        .filter_map(|group| match &group.channels {
            shm::Channels::AnalogInput(channels) => Some(channels.len()),
            _ => None,
        })
        .sum();

    let count_temp: usize = mappings
        .groups
        .iter()
        .filter_map(|group| match &group.channels {
            shm::Channels::TempInput(channels) => Some(channels.len()),
            _ => None,
        })
        .sum();

    let mut index = 0;
    while !mappings.groups.is_empty() {
        let mapping = mappings.groups.swap_remove(0);

        let io = io.clone();
        let tx = tx.clone();

        thread::Builder::new()
            .name(format!("worker{}", index))
            .spawn(move || loop {
                let evt = mapping.notifier.recv();

                {
                    let mut io = io.lock().unwrap();

                    match evt.unwrap() {
                        shm::Event::Update => match &mapping.channels {
                            shm::Channels::AnalogInput(cs) => {
                                for index in cs {
                                    io.analog_input_get(*index)
                                        .map(|val| tx.send(ValueChanged::Ain(*index, val)).unwrap())
                                        .ok();
                                }

                                tx.send(ValueChanged::Flush).unwrap();
                            }
                            shm::Channels::TempInput(cs) => {
                                for index in cs {
                                    io.tmp_input_get(*index)
                                        .map(|val| {
                                            tx.send(ValueChanged::Temp(*index, val)).unwrap()
                                        })
                                        .ok();
                                }

                                tx.send(ValueChanged::Flush).unwrap();
                            }
                        },
                    }
                }
            })
            .unwrap();

        index += 1;
    }

    let signal_notifier = match signal::notify(&[signal::SIGINT, signal::SIGTERM]) {
        Ok(notifier) => notifier,
        Err(e) => {
            eprintln!("Failed to create signal notifier: {}", e);
            std::process::exit(1);
        }
    };

    let config_check = crossbeam_channel::tick(std::time::Duration::from_millis(100));

    loop {
        crossbeam_channel::select! {
            recv(rx) -> change => {
                match change.unwrap() {
                    ValueChanged::Ain(channel, value) => {
                        shm_server.lock().analog_value_set(channel, value)
                    },
                    ValueChanged::Temp(channel, value) => {
                        shm_server.lock().temperature_value_set(channel, value)
                    },
                    ValueChanged::Flush => {
                        shm_server.emit_server_event().expect("emit values updated");
                    }
                }
            }

            recv(config_check) -> _ => {
                match shm_server.await_client_event(Timeout::Val(Duration::from_millis(0))) {
                    Ok(_) => {
                        debug!("--- Apply new configuration ---");

                        for i in 0..count_adc {
                            let mut shm = shm_server.lock();
                            match shm.analog_cfg_get(i) {
                                shm::Config::Keep => { /* nothing to change */ }
                                shm::Config::Change(mode) => {
                                    let mut io = io.lock().unwrap();

                                    debug!("Set mode AIN{}: {:?}", i, mode);

                                    match io.analog_mode_set(i, mode) {
                                        Ok(()) => {}
                                        Err(e) => {
                                            eprintln!("Failed to change configuration:");
                                            eprintln!("    AIN{} to {:?}", i, mode);
                                            eprintln!("    error: {}", e);
                                        }
                                    }
                                }
                            }

                            shm.analog_cfg_set_confirm(i);
                        }

                        for i in 0..count_temp {
                            let mut shm = shm_server.lock();
                            match shm.temperature_cfg_get(i) {
                                shm::Config::Keep => { /* nothing to change */ }
                                shm::Config::Change(ref cfg) => {
                                    let mut io = io.lock().unwrap();

                                    debug!("Set mode TMP{}: {:?} / {:?}", i, cfg.0, cfg.1);

                                    match io.tmp_set_mode(i, cfg.0, cfg.1) {
                                        Ok(()) => {}
                                        Err(e) => {
                                            eprintln!("Failed to change configuration:");
                                            eprintln!("    TMP{} to {:?}", i, cfg);
                                            eprintln!("    error: {}", e);
                                        }
                                    }
                                }
                            }

                            shm.temperature_cfg_set_confirm(i);
                        }
                    }
                    Err(_) => {
                        // FIXME: there is no way the differentiate between actual errors and a
                        //        timeout currently
                    }
                }
            }

            recv(signal_notifier) -> signal => match signal {
                Ok(signal) => {
                    info!("Exit due to signal: {}", signal);
                    drop(shm_server);
                    break;
                }
                Err(_) => {
                    info!("Exit due to error");
                    drop(shm_server);
                    break;
                }
            },
        }
    }
}
