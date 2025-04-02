// SPDX-License-Identifier: LGPL-3.0-or-later
//
// (c) SYSTEC electronic AG, D-08468 Heinsdorfergrund, Am Windrad 2
//     www.systec-electronic.com

use std::mem;

use raw_sync::events::{EventImpl, EventInit, EventState};
use raw_sync::locks::{LockGuard, LockImpl, LockInit, Mutex};
use raw_sync::Timeout;
use shared_memory_extended::{Shmem, ShmemConf};

use crate::ffi;

#[derive(Debug)]
pub enum Event {
    Update,
}

#[derive(Debug)]
pub enum Channels {
    AnalogInput(Vec<usize>),
    TempInput(Vec<usize>),
}

pub struct Group {
    pub notifier: crossbeam_channel::Receiver<Event>,
    pub channels: Channels,
}

pub struct Mappings {
    pub groups: Vec<Group>,
}

const FLINK_PATH: &str = "/run/iomapping.link";
const SHM_SIZE: usize = 4096;

pub const GLOBAL_LOCK_ID: usize = 0;
pub const DAEMON_EVT_ID: usize = 0;
pub const CLIENT_EVT_ID: usize = 1;
pub const NUM_CHANNELS_PER_TYPE: usize = 32;

struct ShmImage {
    _mem: Shmem,
    // Server -> Client
    analog_values_offset: usize,
    temperature_values_offset: usize,

    // Client -> Server
    analog_config_offset: usize,
    temperature_config_offset: usize,

    // lock for all data inside the image
    mutex: Box<dyn LockImpl>,
    server_event: Box<dyn EventImpl>,
    client_event: Box<dyn EventImpl>,
}

struct ShmImageGuard<'g> {
    guard: LockGuard<'g>,
    analog_values_offset: usize,
    temperature_values_offset: usize,
    analog_config_offset: usize,
    temperature_config_offset: usize,
}

impl ShmImage {
    fn config() -> ShmemConf {
        ShmemConf::new()
            .size(SHM_SIZE)
            .flink(FLINK_PATH)
            .writable(true)
    }

    fn create() -> Result<Self, Box<dyn std::error::Error>> {
        let _ = std::fs::remove_file(FLINK_PATH);
        let mem = Self::config().create()?;

        let analog_values_end = mem::size_of::<AnalogInputValues>();
        let temperature_values_end = analog_values_end + mem::size_of::<TemperatureValues>();
        let analog_config_end = temperature_values_end + mem::size_of::<AnalogInputConfigs>();
        let temp_config_end = analog_config_end + mem::size_of::<TemperatureConfigs>();

        let ptr_image = mem.as_ptr();
        let mut ptr = ptr_image.wrapping_add(temp_config_end);

        let (mutex, mutex_len) = unsafe { Mutex::new(ptr, ptr_image) }?;

        ptr = ptr.wrapping_add(mutex_len);

        let (server_event, server_event_len) = unsafe { raw_sync::events::Event::new(ptr, true) }?;
        server_event.set(EventState::Clear)?;

        ptr = ptr.wrapping_add(server_event_len);

        let (client_event, _client_event_len) = unsafe { raw_sync::events::Event::new(ptr, true) }?;
        server_event.set(EventState::Clear)?;

        Ok(Self {
            _mem: mem,
            analog_values_offset: analog_values_end,
            temperature_values_offset: temperature_values_end,
            analog_config_offset: analog_config_end,
            temperature_config_offset: temp_config_end,
            mutex,
            server_event,
            client_event,
        })
    }

    fn open() -> Result<Self, Box<dyn std::error::Error>> {
        let mem = Self::config().open()?;

        let analog_values_end = mem::size_of::<AnalogInputValues>();
        let temperature_values_end = analog_values_end + mem::size_of::<TemperatureValues>();
        let analog_config_end = temperature_values_end + mem::size_of::<AnalogInputConfigs>();
        let temp_config_end = analog_config_end + mem::size_of::<TemperatureConfigs>();

        let ptr_image = mem.as_ptr();
        let mut ptr = ptr_image.wrapping_add(temp_config_end);

        let (mutex, mutex_len) = unsafe { Mutex::from_existing(ptr, ptr_image) }?;

        ptr = ptr.wrapping_add(mutex_len);

        let (server_event, server_event_len) =
            unsafe { raw_sync::events::Event::from_existing(ptr) }?;
        server_event.set(EventState::Clear)?;

        ptr = ptr.wrapping_add(server_event_len);

        let (client_event, _client_event_len) =
            unsafe { raw_sync::events::Event::from_existing(ptr) }?;
        server_event.set(EventState::Clear)?;

        Ok(Self {
            _mem: mem,
            analog_values_offset: analog_values_end,
            temperature_values_offset: temperature_values_end,
            analog_config_offset: analog_config_end,
            temperature_config_offset: temp_config_end,
            server_event,
            client_event,
            mutex,
        })
    }

    fn lock(&mut self) -> ShmImageGuard {
        ShmImageGuard {
            guard: self.mutex.lock().expect("access shared memory locked"),
            analog_values_offset: self.analog_values_offset,
            temperature_values_offset: self.temperature_values_offset,
            analog_config_offset: self.analog_config_offset,
            temperature_config_offset: self.temperature_config_offset,
        }
    }
}

impl<'g> ShmImageGuard<'g> {
    fn analog_value(&mut self, index: usize) -> &mut i64 {
        let values =
            (*self.guard).wrapping_add(self.analog_values_offset) as *mut AnalogInputValues;
        unsafe { (*values).get_mut(index) }.expect("invalid ADC channel")
    }

    fn analog_cfg(&mut self, index: usize) -> &mut Config<ffi::IoAnalogMode> {
        let values =
            (*self.guard).wrapping_add(self.analog_config_offset) as *mut AnalogInputConfigs;
        unsafe { (*values).get_mut(index) }.expect("invalid ADC channel")
    }

    fn temperature_value(&mut self, index: usize) -> &mut f64 {
        let values =
            (*self.guard).wrapping_add(self.temperature_values_offset) as *mut TemperatureValues;
        unsafe { (*values).get_mut(index) }.expect("invalid TMP channel")
    }

    fn temperature_cfg(&mut self, index: usize) -> &mut Config<TmpConfig> {
        let values =
            (*self.guard).wrapping_add(self.temperature_config_offset) as *mut TemperatureConfigs;
        unsafe { (*values).get_mut(index) }.expect("invalid TMP channel")
    }
}

pub struct ShmServer {
    image: ShmImage,
}

unsafe impl Send for ShmServer {}

pub struct ShmServerGuard<'g>(ShmImageGuard<'g>);

impl ShmServer {
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        Ok(Self {
            image: ShmImage::create()?,
        })
    }

    pub fn lock(&mut self) -> ShmServerGuard {
        ShmServerGuard(self.image.lock())
    }

    pub fn emit_server_event(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        self.image.server_event.set(EventState::Signaled)
    }

    pub fn await_client_event(
        &mut self,
        timeout: Timeout,
    ) -> Result<(), Box<dyn std::error::Error>> {
        self.image.client_event.wait(timeout)
    }
}

impl<'g> ShmServerGuard<'g> {
    pub fn analog_value_set(&mut self, index: usize, value: i64) {
        *self.0.analog_value(index) = value
    }

    pub fn analog_cfg_get(&mut self, index: usize) -> Config<ffi::IoAnalogMode> {
        *self.0.analog_cfg(index)
    }

    pub fn analog_cfg_set_confirm(&mut self, index: usize) {
        *self.0.analog_cfg(index) = Config::Keep
    }

    pub fn temperature_value_set(&mut self, index: usize, value: f64) {
        *self.0.temperature_value(index) = value
    }

    pub fn temperature_cfg_get(&mut self, index: usize) -> Config<TmpConfig> {
        *self.0.temperature_cfg(index)
    }

    pub fn temperature_cfg_set_confirm(&mut self, index: usize) {
        *self.0.temperature_cfg(index) = Config::Keep
    }
}

pub struct ShmClient {
    image: ShmImage,
}

unsafe impl Send for ShmClient {}

pub struct ShmClientGuard<'g>(ShmImageGuard<'g>);

impl ShmClient {
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        Ok(Self {
            image: ShmImage::open()?,
        })
    }

    pub fn lock(&mut self) -> ShmClientGuard {
        ShmClientGuard(self.image.lock())
    }

    pub fn emit_client_event(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        self.image.client_event.set(EventState::Signaled)
    }

    pub fn await_server_event(
        &mut self,
        timeout: Timeout,
    ) -> Result<(), Box<dyn std::error::Error>> {
        self.image.client_event.wait(timeout)
    }
}

impl<'g> ShmClientGuard<'g> {
    pub fn analog_value_get(&mut self, index: usize) -> i64 {
        *self.0.analog_value(index)
    }

    pub fn analog_cfg_set(&mut self, index: usize, cfg: ffi::IoAnalogMode) {
        *self.0.analog_cfg(index) = Config::Change(cfg)
    }

    pub fn temperature_value_get(&mut self, index: usize) -> f64 {
        *self.0.temperature_value(index)
    }

    pub fn temperature_cfg_set(
        &mut self,
        index: usize,
        mode: ffi::IoTmpMode,
        sensor_type: ffi::IoTmpSensorType,
    ) {
        *self.0.temperature_cfg(index) = Config::Change(TmpConfig(mode, sensor_type))
    }
}

#[repr(u8)]
#[derive(Debug, Copy, Clone)]
pub enum Config<T> {
    Keep,
    Change(T),
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct TmpConfig(pub ffi::IoTmpMode, pub ffi::IoTmpSensorType);

type AnalogInputValues = [i64; NUM_CHANNELS_PER_TYPE];
type TemperatureValues = [f64; NUM_CHANNELS_PER_TYPE];
type AnalogInputConfigs = [Config<ffi::IoAnalogMode>; NUM_CHANNELS_PER_TYPE];
type TemperatureConfigs = [Config<TmpConfig>; NUM_CHANNELS_PER_TYPE];
