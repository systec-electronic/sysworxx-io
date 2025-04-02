// SPDX-License-Identifier: LGPL-3.0-or-later
//
// (c) SYSTEC electronic AG, D-08468 Heinsdorfergrund, Am Windrad 2
//     www.systec-electronic.com

use std::fmt;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

use crate::error::Result;
use crate::periodic::Periodic;
use crate::{IoChannel, TempSensor};

pub struct LmSensor {
    value: Arc<Mutex<f64>>,
}

impl fmt::Debug for LmSensor {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "sensors/LmSensor")
    }
}

impl LmSensor {
    pub fn new(name: &'static str, interval: Duration) -> LmSensor {
        let value = Arc::new(Mutex::new(0f64));
        let cloned = value.clone();

        thread::Builder::new()
            .name(name.to_owned())
            .spawn(move || {
                let sensors = sensors::Sensors::new();

                let chip = sensors
                    .into_iter()
                    .find(|chip| {
                        chip.get_name()
                            .ok()
                            .filter(|chipname| chipname == name)
                            .is_some()
                    })
                    .unwrap();

                let feature = chip
                    .into_iter()
                    .find(|feature| {
                        *feature.feature_type() == sensors::FeatureType::SENSORS_FEATURE_TEMP
                    })
                    .unwrap();

                let subfeature = feature
                    .into_iter()
                    .find(|subfeature| {
                        *subfeature.subfeature_type()
                            == sensors::SubfeatureType::SENSORS_SUBFEATURE_TEMP_INPUT
                    })
                    .unwrap();

                let mut interval = Periodic::new(interval);
                loop {
                    interval.next();

                    match subfeature.get_value() {
                        Err(_) => warn!("Failed to get sensor value for: {}", name.to_string()),
                        Ok(value) => {
                            let mut cloned = cloned.lock().unwrap();
                            *cloned = value;
                        }
                    }
                }
            })
            .unwrap();

        LmSensor { value }
    }
}

impl IoChannel for LmSensor {
    fn init(&mut self, _chan_number: usize) -> Result<()> {
        Ok(())
    }

    fn shutdown(&mut self) -> Result<()> {
        Ok(())
    }
}

impl TempSensor<f64> for LmSensor {
    fn get(&mut self) -> Result<f64> {
        let value = self.value.lock().unwrap();
        Ok(*value)
    }
}
