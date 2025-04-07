// SPDX-License-Identifier: LGPL-3.0-or-later
//
// (c) SYSTEC electronic AG, D-08468 Heinsdorfergrund, Am Windrad 2
//     www.systec-electronic.com

// This is a copy of ffi.rs from sysworxx-io without any code.
// It is the declaration of the exported functions of libsysworxx-io.so.

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

/// @brief Hardware information structure
///
/// This structure will be filled by IoGetHardwareInfo. It contains the
/// revision information as well as the channel counts for the different
/// peripherals.
#[repr(C)]
#[allow(non_snake_case)]
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

impl From<u8> for IoBool {
    fn from(value: u8) -> IoBool {
        if value != 0 {
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

impl From<u32> for IoInputTrigger {
    fn from(value: u32) -> IoInputTrigger {
        match value {
            0 => IoInputTrigger::None,
            1 => IoInputTrigger::RisingEdge,
            2 => IoInputTrigger::FallingEdge,
            3 => IoInputTrigger::BothEdge,
            _ => IoInputTrigger::None,
        }
    }
}

/// Callback function for changes on digital inputs
pub type IoInputCallback = Option<unsafe extern "C" fn(u8, u8)>;

/// @brief Analog channel mode type
#[repr(u8)]
#[derive(Debug, Copy, Clone)]
pub enum IoAnalogMode {
    Voltage = 0,
    Current = 1,
}

impl From<u8> for IoAnalogMode {
    fn from(value: u8) -> IoAnalogMode {
        match value {
            0 => IoAnalogMode::Voltage,
            1 => IoAnalogMode::Current,
            _ => IoAnalogMode::Voltage,
        }
    }
}

/// @brief Temperature channel modes
#[repr(u8)]
#[derive(Debug, Copy, Clone)]
pub enum IoTmpMode {
    RtdTwoWire = 0,
    RtdThreeWire = 1,
    RtdFourWire = 2,
}

impl From<u8> for IoTmpMode {
    fn from(value: u8) -> IoTmpMode {
        match value {
            0 => IoTmpMode::RtdTwoWire,
            1 => IoTmpMode::RtdThreeWire,
            2 => IoTmpMode::RtdFourWire,
            _ => IoTmpMode::RtdTwoWire,
        }
    }
}

/// @brief Temperature channel types
#[repr(u8)]
#[derive(Debug, Copy, Clone)]
pub enum IoTmpSensorType {
    PT100 = 0,
    PT1000 = 1,
}

impl From<u8> for IoTmpSensorType {
    fn from(value: u8) -> IoTmpSensorType {
        match value {
            0 => IoTmpSensorType::PT100,
            1 => IoTmpSensorType::PT1000,
            _ => IoTmpSensorType::PT100,
        }
    }
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

impl From<u8> for IoCntMode {
    fn from(value: u8) -> IoCntMode {
        match value {
            0 => IoCntMode::Counter,
            1 => IoCntMode::ABEncoder,
            _ => IoCntMode::Counter,
        }
    }
}

/// @brief Counter trigger type (only applies when in mode "counter")
#[repr(u8)]
#[derive(Debug, Copy, Clone)]
pub enum IoCntTrigger {
    RisingEdge = 0,
    FallingEdge = 1,
    AnyEdge = 2,
}

impl From<u8> for IoCntTrigger {
    fn from(value: u8) -> IoCntTrigger {
        match value {
            0 => IoCntTrigger::RisingEdge,
            1 => IoCntTrigger::FallingEdge,
            2 => IoCntTrigger::AnyEdge,
            _ => IoCntTrigger::RisingEdge,
        }
    }
}

/// @brief Counter direction type can be used to invert the direction of counting.
#[repr(u8)]
#[derive(Debug, Copy, Clone)]
pub enum IoCntDirection {
    Up = 0,
    Down = 1,
}

impl From<u8> for IoCntDirection {
    fn from(value: u8) -> IoCntDirection {
        match value {
            0 => IoCntDirection::Up,
            1 => IoCntDirection::Down,
            _ => IoCntDirection::Up,
        }
    }
}

/// @brief PWM timebase for period and duty cycle setting.
#[repr(u8)]
#[derive(Debug, Copy, Clone)]
pub enum IoPwmTimebase {
    Ns800 = 1,
    Ms1 = 2,
}

impl From<u8> for IoPwmTimebase {
    fn from(value: u8) -> IoPwmTimebase {
        match value {
            1 => IoPwmTimebase::Ns800,
            2 => IoPwmTimebase::Ms1,
            _ => IoPwmTimebase::Ns800,
        }
    }
}

extern "C" {
    /// @brief Initializes the I/O driver.
    /// @note This function has to be called before any of the other API
    ///       functions can be used.
    ///
    /// @return IoResult Driver result code of type IoResult
    pub fn IoInit() -> IoResult;
}

extern "C" {
    /// @brief De-initialization of the I/O driver
    ///
    /// @return IoResult Driver result code of type IoResult
    pub fn IoShutdown() -> IoResult;
}

#[allow(clippy::not_unsafe_ptr_arg_deref)] // for some reason the unsafe block is ignored by clippy
extern "C" {
    /// @brief Get the version of the I/O driver
    ///
    /// @param puMajor_p Pointer to the resulting major part of the version number
    /// @param puMinor_p Pointer to the resulting minor part of the version number
    /// @return IoResult Driver result code of type IoResult
    pub fn IoGetVersion(puMajor_p: *mut u8, puMinor_p: *mut u8) -> IoResult;
}

extern "C" {
    /// @brief Get the tickcount of the system in milliseconds
    ///
    /// This is a increasing time value starting at an unknown point in
    /// time.
    ///
    /// @param puTickCount_p Pointer to the resulting timestamp value
    /// @return IoResult Driver result code of type IoResult
    pub fn IoGetTickCount(puTickCount_p: *mut u32) -> IoResult;
}

extern "C" {
    /// @brief Enable the systems watchdog
    ///
    /// @param fMonitorOnly_p Enable monitoring only mode. If the watchdog was not
    ///        serviced in time, an error will be reported by the return value of
    ///        IoServiceWatchdog().
    ///
    /// @return IoResult Driver result code of type IoResult
    pub fn IoEnableWatchdog(fMonitorOnly_p: IoBool) -> IoResult;
}

extern "C" {
    /// @brief Service the system watchdog
    ///
    /// @return IoResult Driver result code of type IoResult
    pub fn IoServiceWatchdog() -> IoResult;
}

#[allow(clippy::not_unsafe_ptr_arg_deref)] // for some reason the unsafe block is ignored by clippy
extern "C" {
    /// @brief Get information about device revision and available I/O channels
    ///
    /// @param pHwInfo_p Destination structure with the resulting information
    /// @return IoResult Driver result code of type IoResult
    pub fn IoGetHardwareInfo(pHwInfo_p: *mut IoHwInfo) -> IoResult;
}

extern "C" {
    /// @brief Set the RUN LED
    ///
    /// @param fState_p The state to set
    /// @return IoResult Driver result code of type IoResult
    pub fn IoSetRunLed(fState_p: IoBool) -> IoResult;
}

extern "C" {
    /// @brief Set the ERROR LED
    ///
    /// @param fState_p The state to set
    /// @return IoResult Driver result code of type IoResult
    pub fn IoSetErrLed(fState_p: IoBool) -> IoResult;
}

extern "C" {
    /// @brief Get device interface information
    ///
    /// @param sPath_p Path to file
    /// @return IoResult Driver result code of type IoResult
    pub fn IoGetJson(sPath_p: *const std::os::raw::c_char) -> IoResult;
}

extern "C" {
    /// @brief Get value of the RUN switch
    ///
    /// @param pfRunSwitch_p Pointer to the value destination
    /// @return IoResult Driver result code of type IoResult
    pub fn IoGetRunSwitch(pfRunSwitch_p: *mut IoBool) -> IoResult;
}

extern "C" {
    /// @brief Get value of the config switch
    ///
    /// @param pfConfig_p Pointer to the value destination
    /// @return IoResult Driver result code of type IoResult
    pub fn IoGetConfigEnabled(pfConfig_p: *mut IoBool) -> IoResult;
}

extern "C" {
    /// @brief Set the value of a digital output
    ///
    /// @param uChannel_p The channel of the digital output
    /// @param fEnable_p The value to set
    /// @return IoResult Driver result code of type IoResult
    pub fn IoSetOutput(uChannel_p: u8, fEnable_p: IoBool) -> IoResult;
}

extern "C" {
    /// @brief Get the value of a digital input
    ///
    /// @param uChannel_p The channel of the digital input
    /// @param pfState_p Pointer to the state destination
    /// @return IoResult Driver result code of type IoResult
    pub fn IoGetInput(uChannel_p: u8, pfState_p: *mut IoBool) -> IoResult;
}

extern "C" {
    /// @brief Register a callback to signal changes on an digital input
    ///
    /// @param uChannel_p The channel of the digital input
    /// @param pfnCallback_p The callback function to register of type #IoInputCallback
    /// @param uInterruptTrigger_p Set the kind of trigger for the input #IoInputTrigger
    /// @return IoResult Driver result code of type IoResult
    pub fn IoRegisterInputCallback(
        uChannel_p: u8,
        pfnCallback_p: IoInputCallback,
        uInterruptTrigger_p: IoInputTrigger,
    ) -> IoResult;
}

extern "C" {
    /// @brief Un-register / disable interrupt handling for a digital input
    ///
    /// @param uChannel_p Analogous to #IoRegisterInputCallback
    /// @return IoResult Driver result code of type IoResult
    pub fn IoUnregisterInputCallback(uChannel_p: u8) -> IoResult;
}

extern "C" {
    /// @brief Get the value of an ADC channel
    ///
    /// @param uChannel_p The channel to get
    /// @param puAdcValue_p Pointer to the value destination
    /// @return IoResult Driver result code of type IoResult
    pub fn IoAdcGetValue(uChannel_p: u8, puAdcValue_p: *mut u16) -> IoResult;
}

extern "C" {
    /// @brief Setup an ADC channel for voltage or current measurement
    ///
    /// If a ADC does not support a specific mode or sensor type the error IoResult_NotImplemented
    /// will be returned.
    ///
    /// @param uChannel_p The channel to setup
    /// @param uMode_p The mode of type #IoAnalogMode
    /// @return IoResult Driver result code of type IoResult
    pub fn IoAdcSetMode(uChannel_p: u8, uMode_p: IoAnalogMode) -> IoResult;
}

extern "C" {
    /// @brief Set DAC output value
    ///
    /// @param uChannel_p The channel to set
    /// @param uValue_p The value to set
    /// @return IoResult Driver result code of type IoResult
    pub fn IoDacSetValue(uChannel_p: u8, uValue_p: u16) -> IoResult;
}

extern "C" {
    /// @brief Set mode of a given temperature sensor
    ///
    /// If a sensor does not support a specific mode or sensor type the error IoResult_NotImplemented
    /// will be returned.
    ///
    /// @param uChannel_p The temperature sensor channel
    /// @param uMode_p The mode of type temperature sensor
    /// @return IoResult Driver result code of type IoResult
    pub fn IoTmpSetMode(uChannel_p: u8, uMode_p: IoTmpMode, uType_p: IoTmpSensorType) -> IoResult;
}

extern "C" {
    /// @brief Get the value of a temperature sensor
    ///
    /// @param uChannel_p The temperature sensor channel
    /// @param piValue_p Pointer to the value destination in 1/10000 °C
    /// @return IoResult Driver result code of type IoResult
    pub fn IoTmpGetValue(uChannel_p: u8, piValue_p: *mut i32) -> IoResult;
}

extern "C" {
    /// @brief Enable/disable a counter channel
    ///
    /// @param uChannel_p The channel to control
    /// @param fEnable_p Enable with true value, false will disable it
    /// @return IoResult Driver result code of type IoResult
    pub fn IoCntEnable(uChannel_p: u8, fEnable_p: IoBool) -> IoResult;
}

extern "C" {
    /// @brief Setup the counters mode
    ///
    /// @param uChannel_p The channel to setup
    /// @param uMode_p The mode of the counter, see IoCntMode
    /// @param uTrigger_p The trigger of the counter, see IoCntTrigger
    /// @param uDir_p The direction of counting, see IoCntDirection
    /// @return IoResult Driver result code of type IoResult
    pub fn IoCntSetup(
        uChannel_p: u8,
        uMode_p: IoCntMode,
        uTrigger_p: IoCntTrigger,
        uDir_p: IoCntDirection,
    ) -> IoResult;
}

extern "C" {
    /// @brief Set the initial value of the counter
    ///
    /// @param uChannel_p The channel to setup
    /// @param iPreload_p The initial value to set
    /// @return IoResult Driver result code of type IoResult
    pub fn IoCntSetPreload(uChannel_p: u8, iPreload_p: i32) -> IoResult;
}

extern "C" {
    /// @brief Get the value of a counter channel
    ///
    /// @param uChannel_p The channel to get the value for
    /// @param piValue_p Pointer to the value destination
    /// @return IoResult Driver result code of type IoResult
    pub fn IoCntGetValue(uChannel_p: u8, piValue_p: *mut i32) -> IoResult;
}

extern "C" {
    /// @brief Set the timebase for PWM output
    ///
    /// @param uChannel_p The channel to set the value for
    /// @param timebase Timebase enum
    /// @return IoResult Driver result code of type IoResult
    pub fn IoPwmSetTimebase(uChannel_p: u8, timebase: IoPwmTimebase) -> IoResult;
}

extern "C" {
    /// @brief Setup an PWM channel
    ///
    /// @param uChannel_p The channel to setup
    /// @param period The periode of the PWM
    /// @param duty_cycle The duty cycle of the PWM
    /// @return IoResult Driver result code of type IoResult
    pub fn IoPwmSetup(uChannel_p: u8, period: u16, duty_cycle: u16) -> IoResult;
}

extern "C" {
    /// @brief Enable/Disable PWM output
    ///
    /// @param uChannel_p The channel of the digital output
    /// @param fRun_p The value to set
    /// @return IoResult Driver result code of type IoResult
    pub fn IoPwmEnable(uChannel_p: u8, fRun_p: bool) -> IoResult;
}
