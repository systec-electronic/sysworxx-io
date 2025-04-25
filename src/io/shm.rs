// SPDX-License-Identifier: LGPL-3.0-or-later
// SPDX-FileCopyrightText: 2025 SYS TEC electronic AG <https://www.systec-electronic.com/>

use std::fmt;
use std::sync::{Arc, Mutex};

use crate::error::{Error, Result};
use crate::ffi;
use crate::shm;
use crate::{AnalogInput, IoChannel, TempSensor};

pub struct Sampler {
    inner: Arc<Mutex<SamplerInner>>,
}

impl Default for Sampler {
    fn default() -> Self {
        Self::new()
    }
}

impl Sampler {
    pub fn new() -> Sampler {
        Sampler {
            inner: Arc::new(Mutex::new(SamplerInner::new())),
        }
    }
}

pub struct SamplerInner {
    shm_client: Arc<Mutex<shm::ShmClient>>,
}

impl SamplerInner {
    fn new() -> SamplerInner {
        SamplerInner {
            shm_client: Arc::new(Mutex::new(shm::ShmClient::new().expect("shm client"))),
        }
    }

    fn ain_get_value(&mut self, index: usize) -> Result<i64> {
        let mut shm_client = self.shm_client.lock().map_err(|_| Error::GenericError)?;
        let mut shm = shm_client.lock();
        Ok(shm.analog_value_get(index))
    }

    fn ain_set_mode(&mut self, index: usize, mode: ffi::IoAnalogMode) -> Result<()> {
        let mut shm_client = self.shm_client.lock().map_err(|_| Error::GenericError)?;
        {
            let mut shm = shm_client.lock();
            shm.analog_cfg_set(index, mode);
        }
        shm_client
            .emit_client_event()
            .map_err(|_| Error::GenericError)
    }

    fn temp_get_value(&mut self, index: usize) -> Result<f64> {
        let mut shm_client = self.shm_client.lock().map_err(|_| Error::GenericError)?;
        let mut shm = shm_client.lock();
        Ok(shm.temperature_value_get(index))
    }

    fn temp_set_mode(
        &mut self,
        index: usize,
        mode: ffi::IoTmpMode,
        sensor_type: ffi::IoTmpSensorType,
    ) -> Result<()> {
        let mut shm_client = self.shm_client.lock().map_err(|_| Error::GenericError)?;
        {
            let mut shm = shm_client.lock();
            shm.temperature_cfg_set(index, mode, sensor_type);
        }
        shm_client
            .emit_client_event()
            .map_err(|_| Error::GenericError)
    }
}

pub struct Ai {
    index: usize,
    sampler: Arc<Mutex<SamplerInner>>,
}

impl Ai {
    pub fn new(sampler: &Sampler, index: usize) -> Ai {
        Ai {
            index,
            sampler: sampler.inner.clone(),
        }
    }
}

impl fmt::Debug for Ai {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "shm/Ai {}", self.index)
    }
}

impl IoChannel for Ai {
    fn init(&mut self, chan_number: usize) -> Result<()> {
        if chan_number >= shm::NUM_CHANNELS_PER_TYPE {
            Err(Error::InvalidChannel)
        } else {
            Ok(())
        }
    }

    fn shutdown(&mut self) -> Result<()> {
        Ok(())
    }
}

impl AnalogInput for Ai {
    fn get(&mut self) -> Result<i64> {
        let mut sampler = self.sampler.lock().map_err(|_| Error::GenericError)?;
        sampler.ain_get_value(self.index)
    }

    fn set_mode(&mut self, mode: ffi::IoAnalogMode) -> Result<()> {
        let mut sampler = self.sampler.lock().map_err(|_| Error::GenericError)?;
        sampler.ain_set_mode(self.index, mode)
    }
}

pub struct Temp {
    index: usize,
    sampler: Arc<Mutex<SamplerInner>>,
}

impl fmt::Debug for Temp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "shm/Tc {}", self.index)
    }
}

impl Temp {
    pub fn new(sampler: &Sampler, index: usize) -> Temp {
        Temp {
            index,
            sampler: sampler.inner.clone(),
        }
    }
}

impl IoChannel for Temp {
    fn init(&mut self, chan_number: usize) -> Result<()> {
        if chan_number >= shm::NUM_CHANNELS_PER_TYPE {
            Err(Error::InvalidChannel)
        } else {
            Ok(())
        }
    }

    fn shutdown(&mut self) -> Result<()> {
        Ok(())
    }
}

impl TempSensor<f64> for Temp {
    fn get(&mut self) -> Result<f64> {
        let mut sampler = self.sampler.lock().map_err(|_| Error::GenericError)?;
        sampler.temp_get_value(self.index)
    }

    fn set_mode(&mut self, mode: ffi::IoTmpMode, sensor_type: ffi::IoTmpSensorType) -> Result<()> {
        let mut sampler = self.sampler.lock().map_err(|_| Error::GenericError)?;
        sampler.temp_set_mode(self.index, mode, sensor_type)
    }
}
