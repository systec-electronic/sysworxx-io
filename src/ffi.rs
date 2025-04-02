// SPDX-License-Identifier: LGPL-3.0-or-later
//
// (c) SYSTEC electronic AG, D-08468 Heinsdorfergrund, Am Windrad 2
//     www.systec-electronic.com

#![allow(non_snake_case)]

// This provides the Foreign Function Interface (FFI) for the C API.

use std::sync::{Arc, Mutex};

use crate::error::{Error, Result};
use crate::hw_rev;
use crate::Io;

lazy_static! {
    static ref INSTANCE: Arc<Mutex<Io>> =
        Arc::new(Mutex::new(crate::definition::load_device_definition(
            &hw_rev::get_device_name().unwrap_or("fallback".to_string())
        )));
}

#[repr(u32)]
#[derive(Debug)]
pub enum IoResult {
    /// Function call succeeded
    Success = 0x00,

    /// Generic error occurred
    Error = 0xff,
    /// The functionality is not implemented by the library
    NotImplemented = 0xfe,
    /// One of the given parameters is invalid (e.g. NULL pointer or parameter is out of range)
    InvalidParameter = 0xfd,
    /// The provided channel number is invalid
    InvalidChannel = 0xfc,
    /// The provided mode is invalid
    InvalidMode = 0xfb,
    /// The provided timebase is invalid
    InvalidTimebase = 0xfa,
    /// The provided delta parameter is invalid
    InvalidDelta = 0xf9,
    /// The PTO table is completely filled
    PtoParamTabFull = 0xf8,
    /// Access to the device or peripheral has failed
    DevAccessFailed = 0xf7,
    /// Reserved error code; currently unused.
    Reserved0 = 0xf6,
    /// Reserved error code; currently unused.
    Reserved1 = 0xf5,
    /// Reserved error code; currently unused.
    ShpImgError = 0xf4,
    /// Reserved error code; currently unused.
    AddressOutOfRange = 0xf3,
    /// The watchdog did timeout
    WatchdogTimeout = 0xf2,
}

/// Convert Result type to a plain C style error code
impl<T> From<Result<T>> for IoResult {
    fn from(result: Result<T>) -> IoResult {
        match result {
            Ok(_) => IoResult::Success,
            Err(Error::InvalidChannel) => IoResult::InvalidChannel,
            Err(Error::InvalidParameter) => IoResult::InvalidParameter,
            Err(Error::NotImplemented) => IoResult::NotImplemented,
            Err(Error::WatchdogTimeout) => IoResult::WatchdogTimeout,
            Err(Error::AccessFailed(_)) => IoResult::DevAccessFailed,
            Err(Error::ParseIntError) => IoResult::Error,
            Err(Error::GenericError) => IoResult::Error,
        }
    }
}

/// @brief Hardware information structure
///
/// This structure will be filled by IoGetHardwareInfo. It contains the
/// revision information as well as the channel counts for the different
/// peripherals.
#[repr(C)]
#[derive(Default, Debug)]
pub struct IoHwInfo {
    /// The PCB revision number
    pub m_uPcbRevision: u8,
    /// Number of digital inputs
    pub m_uDiChannels: u8,
    /// Number of digital outputs
    pub m_uDoChannels: u8,
    /// Number of analog inputs
    pub m_uAiChannels: u8,
    /// Number of analog outputs
    pub m_uAoChannels: u8,
    /// Number of temperature inputs
    pub m_uTmpChannels: u8,
    /// Number of counter channels
    pub m_uCntChannels: u8,
    /// Number of A/B decoder channels
    pub m_uEncChannels: u8,
    /// Number of PWM channels
    pub m_uPwmChannels: u8,
    /// Number of real digital inputs (legacy)
    pub m_uLegacyDiChannels: u8,
    /// Number of real digital outputs (legacy)
    pub m_uLegacyDoChannels: u8,
    /// Number of relay outputs (legacy)
    pub m_uLegacyRelayChannels: u8,
    /// Offset of relay outputs in DO channels (legacy)
    pub m_uLegacyRelayOffset: u8,
}

/// @brief Boolean type for usage of this API
#[repr(u8)]
#[derive(Debug, Copy, Clone)]
pub enum IoBool {
    False = 0,
    True = 1,
}

impl From<bool> for IoBool {
    fn from(value: bool) -> IoBool {
        if value {
            IoBool::True
        } else {
            IoBool::False
        }
    }
}

impl std::ops::Deref for IoBool {
    type Target = bool;

    fn deref(&self) -> &Self::Target {
        match self {
            IoBool::True => &true,
            IoBool::False => &false,
        }
    }
}

impl std::ops::Not for IoBool {
    type Output = IoBool;

    fn not(self) -> Self::Output {
        match self {
            IoBool::True => IoBool::False,
            IoBool::False => IoBool::True,
        }
    }
}

/// @brief Trigger type for asynchronous digital input handling
#[repr(u8)]
#[derive(Copy, Clone)]
pub enum IoInputTrigger {
    /// Disable interrupt handling for the channel
    None = 0,
    /// Enable interrupt handling if the input value changes from low to high
    RisingEdge = 1,
    /// Enable interrupt handling if the input value changes from high to low
    FallingEdge = 2,
    /// Enable interrupt handling if the input value changes any way
    BothEdge = 3,
}

/// Callback function for changes on digital inputs
pub type IoInputCallback = Option<extern "C" fn(u8, IoBool)>;

/// @brief Analog channel mode type
#[repr(u8)]
#[derive(Debug, Copy, Clone)]
pub enum IoAnalogMode {
    Voltage = 0,
    Current = 1,
}

/// @brief Temperature channel modes
#[repr(u8)]
#[derive(Debug, Copy, Clone)]
pub enum IoTmpMode {
    RtdTwoWire = 0,
    RtdThreeWire = 1,
    RtdFourWire = 2,
}

/// @brief Temperature channel types
#[repr(u8)]
#[derive(Debug, Copy, Clone)]
pub enum IoTmpSensorType {
    PT100 = 0,
    PT1000 = 1,
}

/// @brief Counter mode type
#[repr(u8)]
#[derive(Debug, Copy, Clone)]
pub enum IoCntMode {
    /// The counter will count edges on digital input 14. The direction of
    /// counting is determined by the value of digital input 15.
    Counter = 0,
    /// The counter will count in A/B decoder mode. Digital input 14 is used
    /// for the 'A' input and digital input 15 is used for 'B'. Switching the
    /// inputs will result in inverse counting.
    ABEncoder = 1,
}

/// @brief Counter trigger type (only applies when in mode "counter")
#[repr(u8)]
#[derive(Debug, Copy, Clone)]
pub enum IoCntTrigger {
    RisingEdge = 0,
    FallingEdge = 1,
    AnyEdge = 2,
}

/// @brief Counter direction type can be used to invert the direction of counting.
#[repr(u8)]
#[derive(Debug, Copy, Clone)]
pub enum IoCntDirection {
    Up = 0,
    Down = 1,
}

/// @brief PWM timebase for period and duty cycle setting.
#[repr(u8)]
#[derive(Debug, Copy, Clone)]
pub enum IoPwmTimebase {
    Ns800 = 1,
    Ms1 = 2,
}

/// @brief Initializes the I/O driver.
/// @note This function has to be called before any of the other API
///       functions can be used.
///
/// @return IoResult Driver result code of type IoResult
#[no_mangle]
pub extern "C" fn IoInit() -> IoResult {
    use env_logger::Env;
    let env = Env::new().filter("IO_LOG").write_style("IO_LOG_STYLE");
    match env_logger::try_init_from_env(env) {
        Ok(_) => {}
        Err(e) => warn!("Failed to initialize logger: {}", e),
    }

    debug!("IoInit");

    catch_unwind! {{
        io_do! {
            io,
            io.init()
        }
    }}
}

/// @brief De-initialization of the I/O driver
///
/// @return IoResult Driver result code of type IoResult
#[no_mangle]
pub extern "C" fn IoShutdown() -> IoResult {
    debug!("IoShutdown");

    catch_unwind! {{
        io_do! {
            io,
            io.shutdown()
        }
    }}
}

/// @brief Get the version of the I/O driver
///
/// @param puMajor_p Pointer to the resulting major part of the version number
/// @param puMinor_p Pointer to the resulting minor part of the version number
/// @return IoResult Driver result code of type IoResult
///
/// # Safety
///
/// `puMajor_p` must be a valid pointer
/// `puMinor_p` must be a valid pointer
#[no_mangle]
pub unsafe extern "C" fn IoGetVersion(puMajor_p: *mut u8, puMinor_p: *mut u8) -> IoResult {
    debug!("IoGetVersion({:?}, {:?})", puMajor_p, puMinor_p);

    catch_unwind! {{
        check_ptr!(puMajor_p, IoResult::InvalidParameter);
        check_ptr!(puMinor_p, IoResult::InvalidParameter);

        unsafe {
            *puMajor_p = env!("CARGO_PKG_VERSION_MAJOR").parse::<u8>().unwrap();
            *puMinor_p = env!("CARGO_PKG_VERSION_MINOR").parse::<u8>().unwrap();
        }

        IoResult::Success
    }}
}

/// @brief Get the tickcount of the system in milliseconds
///
/// This is a increasing time value starting at an unknown point in
/// time.
///
/// @param puTickCount_p Pointer to the resulting timestamp value
/// @return IoResult Driver result code of type IoResult
///
/// # Safety
///
/// `puTickCount_p` must be a valid pointer
#[no_mangle]
pub unsafe extern "C" fn IoGetTickCount(puTickCount_p: *mut u32) -> IoResult {
    debug!("IoGetTickCount({:?})", puTickCount_p);

    catch_unwind! {{
        check_ptr!(puTickCount_p, IoResult::InvalidParameter);

        io_do! {
            io,
            io.get_ticks().map(|v| unsafe { *puTickCount_p = v; })
        }
    }}
}

/// @brief Enable the systems watchdog
///
/// @param fMonitorOnly_p Enable monitoring only mode. If the watchdog was not
///        serviced in time, an error will be reported by the return value of
///        IoServiceWatchdog().
///
/// @return IoResult Driver result code of type IoResult
#[no_mangle]
pub extern "C" fn IoEnableWatchdog(fMonitorOnly_p: IoBool) -> IoResult {
    debug!("IoEnableWatchdog({})", *fMonitorOnly_p);

    catch_unwind! {{
        io_do! {
            io,
            io.watchdog_enable(*fMonitorOnly_p)
        }
    }}
}

/// @brief Service the system watchdog
///
/// @return IoResult Driver result code of type IoResult
#[no_mangle]
pub extern "C" fn IoServiceWatchdog() -> IoResult {
    debug!("IoServiceWatchdog");

    catch_unwind! {{
        io_do! {
            io,
            io.watchdog_service()
        }
    }}
}

/// @brief Get information about device revision and available I/O channels
///
/// @param pHwInfo_p Destination structure with the resulting information
/// @return IoResult Driver result code of type IoResult
///
/// # Safety
///
/// `pHwInfo_p` must point be a valid pointer
#[no_mangle]
pub unsafe extern "C" fn IoGetHardwareInfo(pHwInfo_p: *mut IoHwInfo) -> IoResult {
    debug!("IoGetHardwareInfo({:?})", pHwInfo_p);

    catch_unwind! {{
        check_ptr!(pHwInfo_p, IoResult::InvalidParameter);

        unsafe {
            libc::memset(pHwInfo_p as *mut libc::c_void, 0, std::mem::size_of::<IoHwInfo>());
        }

        io_do! {
            io,
            unsafe {
                io.get_hardware_info(pHwInfo_p.as_mut().unwrap())
            }
        }
    }}
}

/// @brief Set the RUN LED
///
/// @param fState_p The state to set
/// @return IoResult Driver result code of type IoResult
#[no_mangle]
pub extern "C" fn IoSetRunLed(fState_p: IoBool) -> IoResult {
    debug!("IoSetRunLed({})", *fState_p);

    catch_unwind! {{
        io_do! {
            io,
            io.set_run_led(*fState_p)
        }
    }}
}

/// @brief Set the ERROR LED
///
/// @param fState_p The state to set
/// @return IoResult Driver result code of type IoResult
#[no_mangle]
pub extern "C" fn IoSetErrLed(fState_p: IoBool) -> IoResult {
    debug!("IoSetErrLed({})", *fState_p);

    catch_unwind! {{
        io_do! {
            io,
            io.set_err_led(*fState_p)
        }
    }}
}

/// @brief Get device interface information
///
/// @param sPath_p Path to file
/// @return IoResult Driver result code of type IoResult
///
/// # Safety
///
/// `sPath_p` must be a valid pointer
#[no_mangle]
pub unsafe extern "C" fn IoGetJson(sPath_p: *const std::os::raw::c_char) -> IoResult {
    debug!("IoGetJson({:?})", sPath_p);

    catch_unwind! {{
        check_ptr!(sPath_p, IoResult::InvalidParameter);

        io_do! {
            io,
            io.write_json_info( unsafe {std::ffi::CStr::from_ptr(sPath_p)}.to_str().unwrap())
        }
    }}
}

/// @brief Get value of the RUN switch
///
/// @param pfRunSwitch_p Pointer to the value destination
/// @return IoResult Driver result code of type IoResult
///
/// # Safety
///
/// `pfRunSwitch_p` must be a valid pointer
#[no_mangle]
pub unsafe extern "C" fn IoGetRunSwitch(pfRunSwitch_p: *mut IoBool) -> IoResult {
    debug!("IoGetRunSwitch({:?})", pfRunSwitch_p);

    catch_unwind! {{
        check_ptr!(pfRunSwitch_p, IoResult::InvalidParameter);

        io_do! {
            io,
            io.get_run_switch().map(|v| unsafe {
                *pfRunSwitch_p = IoBool::from(v);
            })
        }
    }}
}

/// @brief Get value of the config switch
///
/// @param pfConfig_p Pointer to the value destination
/// @return IoResult Driver result code of type IoResult
///
/// # Safety
///
/// `pfConfig_p` must be a valid pointer
#[no_mangle]
pub unsafe extern "C" fn IoGetConfigEnabled(pfConfig_p: *mut IoBool) -> IoResult {
    debug!("IoGetConfigEnabled({:?})", pfConfig_p);

    catch_unwind! {{
        check_ptr!(pfConfig_p, IoResult::InvalidParameter);

        io_do! {
            io,
            io.get_config_switch().map(|v| unsafe {
                *pfConfig_p = IoBool::from(v);
            })
        }
    }}
}

/// @brief Set the value of a digital output
///
/// @param uChannel_p The channel of the digital output
/// @param fEnable_p The value to set
/// @return IoResult Driver result code of type IoResult
#[no_mangle]
pub extern "C" fn IoSetOutput(uChannel_p: u8, fEnable_p: IoBool) -> IoResult {
    debug!("IoSetOutput({}, {:?})", uChannel_p, *fEnable_p);

    catch_unwind! {{
        io_do! {
            io,
            io.output_set(uChannel_p as usize, *fEnable_p)
        }
    }}
}

/// @brief Get the value of a digital input
///
/// @param uChannel_p The channel of the digital input
/// @param pfState_p Pointer to the state destination
/// @return IoResult Driver result code of type IoResult
///
/// # Safety
///
/// `pfState_p` must be a valid pointer
#[no_mangle]
pub unsafe extern "C" fn IoGetInput(uChannel_p: u8, pfState_p: *mut IoBool) -> IoResult {
    debug!("IoGetInput({}, {:?})", uChannel_p, pfState_p);

    catch_unwind! {{
        check_ptr!(pfState_p, IoResult::InvalidParameter);

        io_do! {
            io,
            io.input_get(uChannel_p as usize).map(|v| unsafe { *pfState_p = IoBool::from(v); })
        }
    }}
}

/// @brief Register a callback to signal changes on an digital input
///
/// @param uChannel_p The channel of the digital input
/// @param pfnCallback_p The callback function to register of type #IoInputCallback
/// @param uInterruptTrigger_p Set the kind of trigger for the input #IoInputTrigger
/// @return IoResult Driver result code of type IoResult
#[no_mangle]
pub extern "C" fn IoRegisterInputCallback(
    uChannel_p: u8,
    pfnCallback_p: IoInputCallback,
    uInterruptTrigger_p: IoInputTrigger,
) -> IoResult {
    debug!(
        "IoRegisterInputCallback({}, {:?}, {:?})",
        uChannel_p, pfnCallback_p, uInterruptTrigger_p as u8
    );

    catch_unwind! {{
        match pfnCallback_p {
            None => IoResult::InvalidParameter,
            Some(_) => {
                io_do! {
                    io,
                    io.input_register_callback(uChannel_p as usize, pfnCallback_p, uInterruptTrigger_p)
                }
            }
        }
    }}
}

/// @brief Un-register / disable interrupt handling for a digital input
///
/// @param uChannel_p Analogous to #IoRegisterInputCallback
/// @return IoResult Driver result code of type IoResult
#[no_mangle]
pub extern "C" fn IoUnregisterInputCallback(uChannel_p: u8) -> IoResult {
    debug!("IoUnregisterInputCallback({})", uChannel_p);

    catch_unwind! {{
        io_do! {
            io,
            io.input_unregister_callback(uChannel_p as usize)
        }
    }}
}

/// @brief Get the value of an ADC channel
///
/// @param uChannel_p The channel to get
/// @param puAdcValue_p Pointer to the value destination
/// @return IoResult Driver result code of type IoResult
///
/// # Safety
///
/// `puAdcValue_p` must be a valid pointer
#[no_mangle]
pub unsafe extern "C" fn IoAdcGetValue(uChannel_p: u8, puAdcValue_p: *mut u16) -> IoResult {
    debug!("IoAdcGetValue({}, {:?})", uChannel_p, puAdcValue_p);

    catch_unwind! {{
        check_ptr!(puAdcValue_p, IoResult::InvalidParameter);

        io_do! {
            io,
            io.analog_input_get(uChannel_p as usize).map(|v| unsafe { *puAdcValue_p = v as u16; })
        }
    }}
}

/// @brief Setup an ADC channel for voltage or current measurement
///
/// If a ADC does not support a specific mode or sensor type the error IoResult_NotImplemented
/// will be returned.
///
/// @param uChannel_p The channel to setup
/// @param uMode_p The mode of type #IoAnalogMode
/// @return IoResult Driver result code of type IoResult
#[no_mangle]
pub extern "C" fn IoAdcSetMode(uChannel_p: u8, uMode_p: IoAnalogMode) -> IoResult {
    debug!("IoAdcSetMode({}, {})", uChannel_p, uMode_p as u8);

    catch_unwind! {{
        io_do! {
            io,
            io.analog_mode_set(uChannel_p as usize, uMode_p)
        }
    }}
}

/// @brief Set DAC output value
///
/// @param uChannel_p The channel to set
/// @param uValue_p The value to set
/// @return IoResult Driver result code of type IoResult
#[no_mangle]
pub extern "C" fn IoDacSetValue(uChannel_p: u8, uValue_p: u16) -> IoResult {
    debug!("IoDacSetValue({}, {})", uChannel_p, uValue_p);

    catch_unwind! {{
        io_do! {
            io,
            io.analog_output_set(uChannel_p as usize, uValue_p as i64)
        }
    }}
}

/// @brief Set mode of a given temperature sensor
///
/// If a sensor does not support a specific mode or sensor type the error IoResult_NotImplemented
/// will be returned.
///
/// @param uChannel_p The temperature sensor channel
/// @param uMode_p The mode of type temperature sensor
/// @return IoResult Driver result code of type IoResult
#[no_mangle]
pub extern "C" fn IoTmpSetMode(
    uChannel_p: u8,
    uMode_p: IoTmpMode,
    uType_p: IoTmpSensorType,
) -> IoResult {
    debug!(
        "IoTmpSetMode({}, {}, {})",
        uChannel_p, uMode_p as u8, uType_p as u8
    );

    catch_unwind! {{
        io_do! {
            io,
            io.tmp_set_mode(uChannel_p as usize, uMode_p, uType_p)
        }
    }}
}

/// @brief Get the value of a temperature sensor
///
/// @param uChannel_p The temperature sensor channel
/// @param piValue_p Pointer to the value destination in 1/10000 °C
/// @return IoResult Driver result code of type IoResult
///
/// # Safety
///
/// `piValue_p` must be a valid pointer
#[no_mangle]
pub unsafe extern "C" fn IoTmpGetValue(uChannel_p: u8, piValue_p: *mut i32) -> IoResult {
    debug!("IoTmpGetValue({}, {:?})", uChannel_p, piValue_p);

    catch_unwind! {{
        check_ptr!(piValue_p, IoResult::InvalidParameter);

        io_do! {
            io,
            io.tmp_input_get(uChannel_p as usize)
                .map(|v| (v * 10000f64) as i32)
                .map(|v| unsafe { *piValue_p = v; })
        }
    }}
}

/// @brief Enable/disable a counter channel
///
/// @param uChannel_p The channel to control
/// @param fEnable_p Enable with true value, false will disable it
/// @return IoResult Driver result code of type IoResult
#[no_mangle]
pub extern "C" fn IoCntEnable(uChannel_p: u8, fEnable_p: IoBool) -> IoResult {
    debug!("IoCntEnable({}, {:?})", uChannel_p, *fEnable_p);

    catch_unwind! {{
        io_do! {
            io,
            io.cnt_enable(uChannel_p as usize, *fEnable_p)
        }
    }}
}

/// @brief Setup the counters mode
///
/// @param uChannel_p The channel to setup
/// @param uMode_p The mode of the counter, see IoCntMode
/// @param uTrigger_p The trigger of the counter, see IoCntTrigger
/// @param uDir_p The direction of counting, see IoCntDirection
/// @return IoResult Driver result code of type IoResult
#[no_mangle]
pub extern "C" fn IoCntSetup(
    uChannel_p: u8,
    uMode_p: IoCntMode,
    uTrigger_p: IoCntTrigger,
    uDir_p: IoCntDirection,
) -> IoResult {
    debug!(
        "IoCntSetMode({}, {:?}, {:?}, {:?})",
        uChannel_p, uMode_p, uTrigger_p, uDir_p
    );

    catch_unwind! {{
        io_do! {
            io,
            io.cnt_setup(uChannel_p as usize, uMode_p, uTrigger_p, uDir_p)
        }
    }}
}

/// @brief Set the initial value of the counter
///
/// @param uChannel_p The channel to setup
/// @param iPreload_p The initial value to set
/// @return IoResult Driver result code of type IoResult
#[no_mangle]
pub extern "C" fn IoCntSetPreload(uChannel_p: u8, iPreload_p: i32) -> IoResult {
    debug!("IoCntSetPreload({}, {})", uChannel_p, iPreload_p);

    catch_unwind! {{
        io_do! {
            io,
            io.cnt_set_preload(uChannel_p as usize, iPreload_p)
        }
    }}
}

/// @brief Get the value of a counter channel
///
/// @param uChannel_p The channel to get the value for
/// @param piValue_p Pointer to the value destination
/// @return IoResult Driver result code of type IoResult
///
/// # Safety
///
/// `piValue_p` must be a valid pointer
#[no_mangle]
pub unsafe extern "C" fn IoCntGetValue(uChannel_p: u8, piValue_p: *mut i32) -> IoResult {
    debug!("IoCntGetValue({})", uChannel_p);

    catch_unwind! {{
        check_ptr!(piValue_p, IoResult::InvalidParameter);

        io_do! {
            io,
            io.cnt_get(uChannel_p as usize)
                .map(|v| unsafe { *piValue_p = v; })
        }
    }}
}

/// @brief Enable PWM output
///
/// @param uChannel_p The channel of the digital output
/// @param fRun_p The value to set
/// @return IoResult Driver result code of type IoResult
#[no_mangle]
pub extern "C" fn IoPwmEnable(uChannel_p: u8, fRun_p: bool) -> IoResult {
    debug!("IoPwmEnable({} {})", uChannel_p, fRun_p);

    catch_unwind! {{
        io_do! {
            io,
            io.pwm_enable(uChannel_p as usize, fRun_p)
        }
    }}
}

/// @brief Setup an PWM channel
///
/// @param uChannel_p The channel to setup
/// @param period The periode of the PWM
/// @param duty_cycle The duty cycle of the PWM
/// @return IoResult Driver result code of type IoResult
#[no_mangle]
pub extern "C" fn IoPwmSetup(uChannel_p: u8, period: u16, duty_cycle: u16) -> IoResult {
    debug!("IoPwmSetup({}, {:?}, {:?})", uChannel_p, period, duty_cycle);

    catch_unwind! {{
        io_do! {
            io,
            io.pwm_setup(uChannel_p as usize, period, duty_cycle)
        }
    }}
}

/// @brief Set the timebase for PWM output
///
/// @param uChannel_p The channel to get the value for
/// @param timebase Timebase enum
/// @return IoResult Driver result code of type IoResult
#[no_mangle]
pub extern "C" fn IoPwmSetTimebase(uChannel_p: u8, timebase: IoPwmTimebase) -> IoResult {
    debug!("IoPwmSetup({}, {:?},)", uChannel_p, timebase,);

    catch_unwind! {{
        io_do! {
            io,
            io.pwm_set_timebase(uChannel_p as usize, timebase)
        }
    }}
}
