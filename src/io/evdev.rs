// SPDX-License-Identifier: LGPL-3.0-or-later
//
// (c) SYSTEC electronic AG, D-08468 Heinsdorfergrund, Am Windrad 2
//     www.systec-electronic.com

use std::cell::RefCell;
use std::fmt;
use std::sync::{Arc, Mutex};
use std::thread;

pub use evdev::KeyCode;

use parking_lot::ReentrantMutex;

use crate::error::{Error, Result};
use crate::ffi;
use crate::io::util::PairMap;
use crate::{DigitalInput, IoChannel};

pub struct EvdevCollector {
    gpios: Arc<Mutex<PairMap<u16, Vec<DiInnerSync>>>>,
    dev: Arc<Mutex<evdev::Device>>,
}

impl EvdevCollector {
    fn get_dev(devname: &str) -> Result<evdev::Device> {
        evdev::enumerate()
            .map(|(_path, dev)| dev)
            .find(|dev| dev.name() == Some(devname))
            .ok_or(Error::GenericError)
    }

    pub fn supports_key(&self, key: KeyCode) -> bool {
        self.dev
            .lock()
            .unwrap()
            .supported_keys()
            .map_or(false, |keys| keys.contains(key))
    }

    pub fn from_name(devname: &str) -> Result<EvdevCollector> {
        let devname = devname.to_string();
        let gpios = Arc::new(Mutex::new(PairMap::default()));
        let gpios_cloned = gpios.clone();
        let dev = Arc::new(Mutex::new(EvdevCollector::get_dev(&devname)?));

        thread::Builder::new()
            .name(devname.to_owned())
            .spawn(move || {
                // We need a second Device instance in the thread to be sure to not deadlock.
                // This could happen, if the device is waiting for events and on the other side
                // someone tries to register a new input.
                // We can unwrap here safely, since the first device creating happened above and it
                // returned successfully.
                let mut dev_thread = EvdevCollector::get_dev(&devname).unwrap();

                loop {
                    for event in dev_thread.fetch_events().unwrap() {
                        let new_value = event.value() != 0;

                        let mut gpios = gpios_cloned.lock().unwrap();

                        let gpios: Option<&mut Vec<DiInnerSync>> = gpios.get_mut(event.code());
                        if let Some(gpios) = gpios {
                            for gpio in gpios.iter() {
                                let lock = gpio.lock();
                                let mut gpio = lock.borrow_mut();

                                let pol = match gpio.polarity {
                                    Polarity::ActiveLow => true,
                                    Polarity::ActiveHigh => false,
                                };
                                gpio.value = new_value ^ pol;

                                // ensure we to not borrow the gpio twice, since this would panic -> drop it!
                                // (see documentation on RefCell)
                                // this solves problems when calling function on inputs while the callback
                                // is executed
                                // the ReentrantLock still ensures that no other thread will call into the
                                // module
                                let callback = gpio.callback;
                                let trigger = gpio.trigger;
                                let number = gpio.number;
                                let new_value = gpio.value;
                                drop(gpio);

                                if let Some(Some(callback)) = callback {
                                    let should_call = match trigger {
                                        ffi::IoInputTrigger::None => false,
                                        ffi::IoInputTrigger::RisingEdge => new_value,
                                        ffi::IoInputTrigger::FallingEdge => !new_value,
                                        ffi::IoInputTrigger::BothEdge => true,
                                    };

                                    if should_call {
                                        callback(number as u8, new_value.into());
                                    }
                                }
                            }
                        }
                    }
                }
            })
            .unwrap();

        Ok(EvdevCollector { gpios, dev })
    }

    fn register(&mut self, key: KeyCode, gpio: DiInnerSync) -> Result<()> {
        fn get_current_input_state(dev: &evdev::Device, code: KeyCode) -> Result<bool> {
            let val = dev.get_key_state()?.contains(code);
            Ok(val)
        }

        {
            let lock = gpio.lock();
            let mut gpio = lock.borrow_mut();
            let dev = self.dev.lock().unwrap();
            let pol = match gpio.polarity {
                Polarity::ActiveLow => true,
                Polarity::ActiveHigh => false,
            };
            gpio.value = get_current_input_state(&dev, key)? ^ pol;
        }

        let mut gpios = self.gpios.lock().map_err(|_| Error::GenericError)?;
        if gpios.contains(key.0) {
            let gpios_vec = gpios.get_mut(key.0).unwrap();
            gpios_vec.push(gpio);
        } else {
            gpios.set(key.0, vec![gpio]);
        }

        Ok(())
    }
}

#[derive(Debug)]
enum Polarity {
    ActiveHigh,
    ActiveLow,
}

struct DiInner {
    value: bool,
    number: usize,
    callback: Option<ffi::IoInputCallback>,
    trigger: ffi::IoInputTrigger,
    polarity: Polarity,
}

type DiInnerSync = Arc<ReentrantMutex<RefCell<DiInner>>>;

pub struct Di {
    inner: DiInnerSync,
}

impl fmt::Debug for Di {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "evdev/Di")
    }
}

impl Di {
    fn new_priv(collector: &mut EvdevCollector, key: KeyCode, polarity: Polarity) -> Di {
        let gpio = Arc::new(ReentrantMutex::new(RefCell::new(DiInner {
            value: false,
            number: usize::default(),
            callback: None,
            trigger: ffi::IoInputTrigger::None,
            polarity,
        })));
        collector.register(key, gpio.clone()).unwrap();
        Di { inner: gpio }
    }

    pub fn new(collector: &mut EvdevCollector, key: KeyCode) -> Di {
        Di::new_priv(collector, key, Polarity::ActiveHigh)
    }

    pub fn active_low(collector: &mut EvdevCollector, key: KeyCode) -> Di {
        Di::new_priv(collector, key, Polarity::ActiveLow)
    }
}

impl IoChannel for Di {
    fn init(&mut self, chan_number: usize) -> Result<()> {
        let lock = self.inner.lock();
        let mut _self = lock.borrow_mut();
        _self.number = chan_number;
        Ok(())
    }

    fn shutdown(&mut self) -> Result<()> {
        Ok(())
    }
}

impl DigitalInput for Di {
    fn get(&mut self) -> Result<bool> {
        let lock = self.inner.lock();
        let mut _self = lock.borrow_mut();
        Ok(_self.value)
    }

    fn register_callback(
        &mut self,
        callback: ffi::IoInputCallback,
        trigger: ffi::IoInputTrigger,
    ) -> Result<()> {
        let lock = self.inner.lock();
        let mut _self = lock.borrow_mut();
        _self.callback = Some(callback);
        _self.trigger = trigger;
        Ok(())
    }

    fn unregister_callback(&mut self) -> Result<()> {
        let lock = self.inner.lock();
        let mut _self = lock.borrow_mut();

        _self.callback = None;
        _self.trigger = ffi::IoInputTrigger::None;
        Ok(())
    }
}
