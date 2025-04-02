// SPDX-License-Identifier: LGPL-3.0-or-later
//
// (c) SYSTEC electronic AG, D-08468 Heinsdorfergrund, Am Windrad 2
//     www.systec-electronic.com

#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

// This provides the Foreign Function Interface (FFI) for the C API of
// libctr700drv.so.
// Initially, it has been created by Rust bindgen from ctr700drv.h.

use crate::sysworxx_io::*;
use std::sync::{Arc, Mutex};

const SPECIAL_INPUT_RUN_SWITCH: u8 = 0x80;
const RUN_SWITCH_CHANNEL: u8 = 0x26;

#[doc = "< Function call succeeded"]
pub const kCtr700DrvResult_Success: tCtr700DrvResult = 0;
#[doc = "< Generic error occurred"]
pub const kCtr700DrvResult_Error: tCtr700DrvResult = 255;
#[doc = "< The functionality is not implemented by the library"]
pub const kCtr700DrvResult_NotImplemented: tCtr700DrvResult = 254;
#[doc = "< One of the given parameters is invalid (e.g. NULL pointer or parameter is out of range)"]
pub const kCtr700DrvResult_InvalidParameter: tCtr700DrvResult = 253;
#[doc = "< The provided channel number is invalid"]
pub const kCtr700DrvResult_InvalidChannel: tCtr700DrvResult = 252;
#[doc = "< The provided mode is invalid"]
pub const kCtr700DrvResult_InvalidMode: tCtr700DrvResult = 251;
#[doc = "< The provided timebase is invalid"]
pub const kCtr700DrvResult_InvalidTimebase: tCtr700DrvResult = 250;
#[doc = "< The provided delta parameter is invalid"]
pub const kCtr700DrvResult_InvalidDelta: tCtr700DrvResult = 249;
#[doc = "< The PTO table is completely filled"]
pub const kCtr700DrvResult_PtoParamTabFull: tCtr700DrvResult = 248;
#[doc = "< Access to the device or peripheral has failed"]
pub const kCtr700DrvResult_DevAccessFailed: tCtr700DrvResult = 247;
#[doc = "< Reserved error code; currently unused."]
pub const kCtr700DrvResult_InvalidProcImgCfg: tCtr700DrvResult = 246;
#[doc = "< Reserved error code; currently unused."]
pub const kCtr700DrvResult_ProcImgCfgUnknown: tCtr700DrvResult = 245;
#[doc = "< Reserved error code; currently unused."]
pub const kCtr700DrvResult_ShpImgError: tCtr700DrvResult = 244;
#[doc = "< Reserved error code; currently unused."]
pub const kCtr700DrvResult_AddressOutOfRange: tCtr700DrvResult = 243;
#[doc = "< The watchdog did timeout"]
pub const kCtr700DrvResult_WatchdogTimeout: tCtr700DrvResult = 242;
#[doc = " @brief Common error codes for all API functions"]
pub type tCtr700DrvResult = ::std::os::raw::c_uint;
pub const tCtr700Drv_Bool_kCtr700Drv_False: tCtr700Drv_Bool = 0;
pub const tCtr700Drv_Bool_kCtr700Drv_True: tCtr700Drv_Bool = 1;
#[doc = " @brief Simple boolean type for some functions of the API."]
pub type tCtr700Drv_Bool = u8;
pub const kCtr700DrvAnalogIn_Channel0: tCtr700DrvAnalogIn = 0;
pub const kCtr700DrvAnalogIn_Channel1: tCtr700DrvAnalogIn = 1;
pub const kCtr700DrvAnalogIn_Channel2: tCtr700DrvAnalogIn = 2;
pub const kCtr700DrvAnalogIn_Channel3: tCtr700DrvAnalogIn = 3;
#[doc = " @brief Analog input channel type"]
pub type tCtr700DrvAnalogIn = ::std::os::raw::c_uint;
pub const kCtr700DrvAnalogMode_Voltage: tCtr700DrvAnalogMode = 0;
pub const kCtr700DrvAnalogMode_Current: tCtr700DrvAnalogMode = 1;
#[doc = " @brief Analog channel mode type"]
pub type tCtr700DrvAnalogMode = ::std::os::raw::c_uint;
pub const kCtr700DrvCounter_Channel0: tCtr700DrvCounter = 0;
#[doc = " @brief Counter channel type"]
pub type tCtr700DrvCounter = ::std::os::raw::c_uint;
#[doc = " The counter will count edges on digital input 14. The direction of"]
#[doc = " counting is determined by the value of digital input 15."]
pub const kCtr700DrvCounterMode_Counter: tCtr700DrvCounterMode = 0;
#[doc = " The counter will count in A/B decoder mode. Digital input 14 is used"]
#[doc = " for the 'A' input and digital input 15 is used for 'B'. Switching the"]
#[doc = " inputs will result in inverse counting."]
pub const kCtr700DrvCounterMode_AB_Decoder: tCtr700DrvCounterMode = 1;
#[doc = " @brief Counter mode type"]
pub type tCtr700DrvCounterMode = ::std::os::raw::c_uint;
pub const kCtr700DrvCounterTrigger_RisingEdge: tCtr700DrvCounterTrigger = 0;
pub const kCtr700DrvCounterTrigger_FallingEdge: tCtr700DrvCounterTrigger = 1;
pub const kCtr700DrvCounterTrigger_AnyEdge: tCtr700DrvCounterTrigger = 2;
#[doc = " @brief Counter trigger type (only applies to #kCtr700DrvCounterMode_Counter"]
pub type tCtr700DrvCounterTrigger = ::std::os::raw::c_uint;
pub const kCtr700DrvCounterDirection_Up: tCtr700DrvCounterDirection = 0;
pub const kCtr700DrvCounterDirection_Down: tCtr700DrvCounterDirection = 1;
#[doc = " @brief Counter direction type can be used to invert the direction of"]
#[doc = "        counting."]
pub type tCtr700DrvCounterDirection = ::std::os::raw::c_uint;
pub const kCtr700DrvPwm_Channel0: tCtr700DrvPwm = 0;
pub const kCtr700DrvPwm_Channel1: tCtr700DrvPwm = 1;
#[doc = " @brief PWM channel type"]
pub type tCtr700DrvPwm = ::std::os::raw::c_uint;
pub const kCtr700DrvPwmTimebase_800NS: tCtr700DrvPwmTimebase = 1;
pub const kCtr700DrvPwmTimebase_1MS: tCtr700DrvPwmTimebase = 2;
#[doc = " @brief PWM timebase type"]
pub type tCtr700DrvPwmTimebase = ::std::os::raw::c_uint;
#[doc = "< Internal temperature sensor of the i.MX7"]
pub const kCtr700DrvTmp_Channel0: tCtr700DrvTmp = 0;
#[doc = "< Temperature sensor on the system PCB of sysWORXX CTR-700"]
pub const kCtr700DrvTmp_Channel1: tCtr700DrvTmp = 1;
#[doc = " @brief Temperature sensor type"]
pub type tCtr700DrvTmp = ::std::os::raw::c_uint;
#[doc = " @brief Hardware information structure"]
#[doc = ""]
#[doc = " This structure will be filled by #Ctr700DrvGetHardwareInfo. It contains the"]
#[doc = " revision information as well as the channel counts for the different"]
#[doc = " peripherals."]
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct tCtr700DrvHwInfo {
    #[doc = "< The PCB revision number"]
    pub m_uPcbRevision: u16,
    #[doc = "< Number of digital inputs"]
    pub m_uDiChannels: u16,
    #[doc = "< Number of digital outputs"]
    pub m_uDoChannels: u16,
    #[doc = "< Number of relay outputs"]
    pub m_uRelayChannels: u16,
    #[doc = "< Number of analog inputs"]
    pub m_uAiChannels: u16,
    #[doc = "< Number of analog outputs"]
    pub m_uAoChannels: u16,
    #[doc = "< Number of counter channels"]
    pub m_uCntChannels: u16,
    #[doc = "< Number of A/B decoder channels"]
    pub m_uEncChannels: u16,
    #[doc = "< Number of PWM channels"]
    pub m_uPwmChannels: u16,
    #[doc = "< Number of temperature channels"]
    pub m_uTmpChannels: u16,
}
#[test]
fn bindgen_test_layout_tCtr700DrvHwInfo() {
    assert_eq!(
        ::std::mem::size_of::<tCtr700DrvHwInfo>(),
        20usize,
        concat!("Size of: ", stringify!(tCtr700DrvHwInfo))
    );
    assert_eq!(
        ::std::mem::align_of::<tCtr700DrvHwInfo>(),
        2usize,
        concat!("Alignment of ", stringify!(tCtr700DrvHwInfo))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tCtr700DrvHwInfo>())).m_uPcbRevision as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(tCtr700DrvHwInfo),
            "::",
            stringify!(m_uPcbRevision)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tCtr700DrvHwInfo>())).m_uDiChannels as *const _ as usize },
        2usize,
        concat!(
            "Offset of field: ",
            stringify!(tCtr700DrvHwInfo),
            "::",
            stringify!(m_uDiChannels)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tCtr700DrvHwInfo>())).m_uDoChannels as *const _ as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(tCtr700DrvHwInfo),
            "::",
            stringify!(m_uDoChannels)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<tCtr700DrvHwInfo>())).m_uRelayChannels as *const _ as usize
        },
        6usize,
        concat!(
            "Offset of field: ",
            stringify!(tCtr700DrvHwInfo),
            "::",
            stringify!(m_uRelayChannels)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tCtr700DrvHwInfo>())).m_uAiChannels as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(tCtr700DrvHwInfo),
            "::",
            stringify!(m_uAiChannels)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tCtr700DrvHwInfo>())).m_uAoChannels as *const _ as usize },
        10usize,
        concat!(
            "Offset of field: ",
            stringify!(tCtr700DrvHwInfo),
            "::",
            stringify!(m_uAoChannels)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tCtr700DrvHwInfo>())).m_uCntChannels as *const _ as usize },
        12usize,
        concat!(
            "Offset of field: ",
            stringify!(tCtr700DrvHwInfo),
            "::",
            stringify!(m_uCntChannels)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tCtr700DrvHwInfo>())).m_uEncChannels as *const _ as usize },
        14usize,
        concat!(
            "Offset of field: ",
            stringify!(tCtr700DrvHwInfo),
            "::",
            stringify!(m_uEncChannels)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tCtr700DrvHwInfo>())).m_uPwmChannels as *const _ as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(tCtr700DrvHwInfo),
            "::",
            stringify!(m_uPwmChannels)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tCtr700DrvHwInfo>())).m_uTmpChannels as *const _ as usize },
        18usize,
        concat!(
            "Offset of field: ",
            stringify!(tCtr700DrvHwInfo),
            "::",
            stringify!(m_uTmpChannels)
        )
    );
}
#[doc = " @brief Diagnose information structure"]
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct tCtr700DrvDiagInfo {
    #[doc = " @brief Signals powerfail errors of the driver for digital outputs (**active high**)"]
    #[doc = ""]
    #[doc = " This signal will be active, when the power supply for digital outputs"]
    #[doc = " is not properly connected."]
    pub m_fDigiOutPowerFail: u8,
    #[doc = " @brief Signals an error for digital outputs (**active low**)"]
    #[doc = ""]
    #[doc = " This error signal will be active in one of two cases:"]
    #[doc = " - overtemperature error of the driver IC"]
    #[doc = " - internal communication error of the driver IC"]
    pub m_fDigiOutDiag: u8,
    #[doc = " @brief Signals an error for digital inputs (**active low**)"]
    #[doc = ""]
    #[doc = " This error signal will be active in one of two cases:"]
    #[doc = " - power supply is not connected to the driver IC"]
    #[doc = " - internal communication error of the driver IC"]
    pub m_fDigiInError: u8,
    #[doc = " @brief Signals an over-current error on USB interface (**active low**)"]
    pub m_fUsbOverCurrent: u8,
}
#[test]
fn bindgen_test_layout_tCtr700DrvDiagInfo() {
    assert_eq!(
        ::std::mem::size_of::<tCtr700DrvDiagInfo>(),
        4usize,
        concat!("Size of: ", stringify!(tCtr700DrvDiagInfo))
    );
    assert_eq!(
        ::std::mem::align_of::<tCtr700DrvDiagInfo>(),
        1usize,
        concat!("Alignment of ", stringify!(tCtr700DrvDiagInfo))
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<tCtr700DrvDiagInfo>())).m_fDigiOutPowerFail as *const _ as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(tCtr700DrvDiagInfo),
            "::",
            stringify!(m_fDigiOutPowerFail)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<tCtr700DrvDiagInfo>())).m_fDigiOutDiag as *const _ as usize
        },
        1usize,
        concat!(
            "Offset of field: ",
            stringify!(tCtr700DrvDiagInfo),
            "::",
            stringify!(m_fDigiOutDiag)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<tCtr700DrvDiagInfo>())).m_fDigiInError as *const _ as usize
        },
        2usize,
        concat!(
            "Offset of field: ",
            stringify!(tCtr700DrvDiagInfo),
            "::",
            stringify!(m_fDigiInError)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<tCtr700DrvDiagInfo>())).m_fUsbOverCurrent as *const _ as usize
        },
        3usize,
        concat!(
            "Offset of field: ",
            stringify!(tCtr700DrvDiagInfo),
            "::",
            stringify!(m_fUsbOverCurrent)
        )
    );
}
#[doc = " @brief Callback function type for asynchronous handling of digital inputs"]
pub type tCtr700DrvInterruptCallback =
    ::std::option::Option<unsafe extern "C" fn(arg1: u8, arg2: u8)>;
#[doc = "< Disable interrupt handling for the channel"]
pub const kCtr700DrvInterruptNone: tCtr700DrvInterruptTrigger = 0;
#[doc = "< Enable interrupt handling if the input value changes from low to high"]
pub const kCtr700DrvInterruptRisingEdge: tCtr700DrvInterruptTrigger = 1;
#[doc = "< Enable interrupt handling if the input value changes from high to low"]
pub const kCtr700DrvInterruptFallingEdge: tCtr700DrvInterruptTrigger = 2;
#[doc = "< Enable interrupt handling if the input value changes any way"]
pub const kCtr700DrvInterruptBothEdge: tCtr700DrvInterruptTrigger = 3;
#[doc = " @brief Trigger type for asynchronous digital input handling"]
pub type tCtr700DrvInterruptTrigger = ::std::os::raw::c_uint;

lazy_static! {
    static ref HWINFO: Arc<Mutex<IoHwInfo>> = Arc::new(Mutex::new(IoHwInfo::default()));
}

#[doc = " @brief Initializes the I/O driver."]
#[doc = " @note This function has to be called before any of the other API"]
#[doc = "       functions can be used."]
#[doc = ""]
#[doc = " @return int32_t Driver result code of type tCtr700DrvResult"]
#[no_mangle]
pub extern "C" fn Ctr700DrvInitialize() -> i32 {
    let ret = unsafe { IoInit() };
    match ret {
        IoResult::Success => {}
        _ => {
            return ret as i32;
        }
    }
    let mut hwinfo = HWINFO.lock().unwrap();
    let ret = unsafe { IoGetHardwareInfo(&mut *hwinfo) };
    ret as i32
}

#[doc = " @brief De-initialization of the I/O driver"]
#[doc = ""]
#[doc = " @return int32_t Driver result code of type tCtr700DrvResult"]
#[no_mangle]
pub extern "C" fn Ctr700DrvShutDown() -> i32 {
    let ret = unsafe { IoShutdown() };
    ret as i32
}

#[doc = " @brief Get the version of the I/O driver"]
#[doc = ""]
#[doc = " @param puMajor Pointer to the resulting major part of the version number"]
#[doc = " @param puMinor Pointer to the resulting minor part of the version number"]
#[doc = " @return int32_t Driver result code of type tCtr700DrvResult"]
#[no_mangle]
pub extern "C" fn Ctr700DrvGetVersion(puMajor: *mut u8, puMinor: *mut u8) -> i32 {
    (catch_unwind! {{
        check_ptr!(puMajor, IoResult::InvalidParameter);
        check_ptr!(puMinor, IoResult::InvalidParameter);

        unsafe {
            *puMajor = env!("CARGO_PKG_VERSION_MAJOR").parse::<u8>().unwrap();
            *puMinor = env!("CARGO_PKG_VERSION_MINOR").parse::<u8>().unwrap();
        }

        IoResult::Success
    }}) as i32
}

#[doc = " @brief Get the tickcount of the system in milliseconds"]
#[doc = ""]
#[doc = " This is a increasing time value starting at an unknown point in"]
#[doc = " time."]
#[doc = ""]
#[doc = " @param puTickCount_p Pointer to the resulting timestamp value"]
#[doc = " @return int32_t Driver result code of type tCtr700DrvResult"]
#[no_mangle]
#[allow(clippy::not_unsafe_ptr_arg_deref)] // for some reason the unsafe block is ignored by clippy
pub extern "C" fn Ctr700DrvGetTickCount(puTickCount_p: *mut u32) -> i32 {
    unsafe { IoGetTickCount(puTickCount_p) as i32 }
}

#[doc = " @brief Enable the systems watchdog"]
#[doc = ""]
#[doc = " @param fMonitorOnly_p Enable monitoring only mode. If the watchdog was not"]
#[doc = "        serviced in time, an error will be reported by the return value of"]
#[doc = "        Ctr700DrvServiceWatchdog()."]
#[doc = ""]
#[doc = " The watchdog intervall has a fixed timeout setting of:"]
#[doc = " - 1000 ms for non-monitoring mode"]
#[doc = " - 900 ms for monitoring mode"]
#[doc = ""]
#[doc = " @return int32_t Driver result code of type tCtr700DrvResult"]
#[no_mangle]
pub extern "C" fn Ctr700DrvEnableWatchdog(fMonitorOnly_p: u8) -> i32 {
    unsafe { IoEnableWatchdog(IoBool::from(fMonitorOnly_p)) as i32 }
}

#[doc = " @brief Service the system watchdog"]
#[doc = ""]
#[doc = " @return int32_t Driver result code of type tCtr700DrvResult"]
#[no_mangle]
pub extern "C" fn Ctr700DrvServiceWatchdog() -> i32 {
    unsafe { IoServiceWatchdog() as i32 }
}

#[doc = " @brief Get information about device revision and available I/O channels"]
#[doc = ""]
#[doc = " @param pHwInfo_p Destination structure with the resulting information"]
#[doc = " @return int32_t Driver result code of type tCtr700DrvResult"]
#[no_mangle]
#[allow(clippy::not_unsafe_ptr_arg_deref)] // for some reason the unsafe block is ignored by clippy
pub extern "C" fn Ctr700DrvGetHardwareInfo(pHwInfo_p: *mut tCtr700DrvHwInfo) -> i32 {
    check_ptr!(pHwInfo_p, kCtr700DrvResult_InvalidParameter as i32);

    let srcinfo = HWINFO.lock().unwrap();
    unsafe {
        let dstinfo = pHwInfo_p.as_mut().unwrap();
        dstinfo.m_uPcbRevision = srcinfo.m_uPcbRevision as u16;
        dstinfo.m_uDiChannels = srcinfo.m_uLegacyDiChannels as u16;
        dstinfo.m_uDoChannels = srcinfo.m_uLegacyDoChannels as u16;
        dstinfo.m_uRelayChannels = srcinfo.m_uLegacyRelayChannels as u16;
        dstinfo.m_uAiChannels = srcinfo.m_uAiChannels as u16;
        dstinfo.m_uAoChannels = srcinfo.m_uAoChannels as u16;
        dstinfo.m_uAiChannels = srcinfo.m_uAiChannels as u16;
        dstinfo.m_uCntChannels = srcinfo.m_uCntChannels as u16;
        dstinfo.m_uEncChannels = srcinfo.m_uEncChannels as u16;
        dstinfo.m_uPwmChannels = srcinfo.m_uPwmChannels as u16;
        dstinfo.m_uTmpChannels = srcinfo.m_uTmpChannels as u16;
    }

    kCtr700DrvResult_Success as i32
}

#[doc = " @brief Set the RUN LED"]
#[doc = ""]
#[doc = " @param fState_p The state to set"]
#[doc = " @return int32_t Driver result code of type tCtr700DrvResult"]
#[no_mangle]
pub extern "C" fn Ctr700DrvSetRunLed(fState_p: u8) -> i32 {
    unsafe { IoSetRunLed(IoBool::from(fState_p)) as i32 }
}

#[doc = " @brief Set the ERROR LED"]
#[doc = ""]
#[doc = " @param fState_p The state to set"]
#[doc = " @return int32_t Driver result code of type tCtr700DrvResult"]
#[no_mangle]
pub extern "C" fn Ctr700DrvSetErrLed(fState_p: u8) -> i32 {
    unsafe { IoSetErrLed(IoBool::from(fState_p)) as i32 }
}

#[doc = " @brief Get value of the RUN switch"]
#[doc = ""]
#[doc = " @param pfRunSwitch_p Pointer to the value destination"]
#[doc = " @return int32_t Driver result code of type tCtr700DrvResult"]
#[no_mangle]
#[allow(clippy::not_unsafe_ptr_arg_deref)] // for some reason the unsafe block is ignored by clippy
pub extern "C" fn Ctr700DrvGetRunSwitch(pfRunSwitch_p: *mut u8) -> i32 {
    check_ptr!(pfRunSwitch_p, kCtr700DrvResult_InvalidParameter as i32);

    unsafe {
        let mut state: IoBool = IoBool::False;
        let ret = IoGetRunSwitch(&mut state) as i32;
        match state {
            IoBool::True => *pfRunSwitch_p = tCtr700Drv_Bool_kCtr700Drv_True,
            IoBool::False => *pfRunSwitch_p = tCtr700Drv_Bool_kCtr700Drv_False,
        }
        ret
    }
}

#[doc = " @brief Get value of the config switch (DIP 4)"]
#[doc = ""]
#[doc = " @param pfConfig_p Pointer to the value destination"]
#[doc = " @return int32_t Driver result code of type tCtr700DrvResult"]
#[no_mangle]
#[allow(clippy::not_unsafe_ptr_arg_deref)] // for some reason the unsafe block is ignored by clippy
pub extern "C" fn Ctr700DrvGetConfigEnabled(pfConfig_p: *mut u8) -> i32 {
    check_ptr!(pfConfig_p, kCtr700DrvResult_InvalidParameter as i32);

    unsafe {
        let mut state: IoBool = IoBool::False;
        let ret = IoGetConfigEnabled(&mut state) as i32;
        match state {
            IoBool::True => *pfConfig_p = tCtr700Drv_Bool_kCtr700Drv_True,
            IoBool::False => *pfConfig_p = tCtr700Drv_Bool_kCtr700Drv_False,
        }
        ret
    }
}

#[doc = " @brief Get the state of power fail signal"]
#[doc = " @note Channel 32/0x20 is used for EXT_RESET"]
#[doc = ""]
#[doc = " @param pfFail_p Pointer to the value destination"]
#[doc = " @return int32_t Driver result code of type tCtr700DrvResult"]
#[no_mangle]
pub extern "C" fn Ctr700DrvGetPowerFail(pfFail_p: *mut u8) -> i32 {
    check_ptr!(pfFail_p, kCtr700DrvResult_InvalidParameter as i32);

    unsafe {
        let mut state: IoBool = IoBool::False;
        let ret = IoGetInput(32, &mut state) as i32;
        match state {
            IoBool::True => *pfFail_p = tCtr700Drv_Bool_kCtr700Drv_True,
            IoBool::False => *pfFail_p = tCtr700Drv_Bool_kCtr700Drv_False,
        }
        ret
    }
}

#[doc = " @brief Get current state of diagnostic signals"]
#[doc = ""]
#[doc = " @param pDiagInfo_p Pointer to the value destination"]
#[doc = " @return int32_t Driver result code of type tCtr700DrvResult"]
#[no_mangle]
pub extern "C" fn Ctr700DrvGetDiagInfo(pDiagInfo_p: *mut tCtr700DrvDiagInfo) -> i32 {
    check_ptr!(pDiagInfo_p, kCtr700DrvResult_InvalidParameter as i32);

    fn GetDiagInfo(uChannel: u8) -> u8 {
        unsafe {
            let mut state: IoBool = IoBool::False;
            IoGetInput(uChannel, &mut state);
            match state {
                IoBool::True => tCtr700Drv_Bool_kCtr700Drv_True,
                IoBool::False => tCtr700Drv_Bool_kCtr700Drv_False,
            }
        }
    }

    unsafe {
        let pDiagInfo_p: &mut tCtr700DrvDiagInfo = pDiagInfo_p.as_mut().unwrap();
        pDiagInfo_p.m_fDigiOutPowerFail = GetDiagInfo(35);
        pDiagInfo_p.m_fDigiOutDiag = GetDiagInfo(36);
        pDiagInfo_p.m_fDigiInError = GetDiagInfo(33);
        pDiagInfo_p.m_fUsbOverCurrent = GetDiagInfo(34);

        let ret = IoResult::Success as i32;
        ret
    }
}

#[doc = " @brief Get value of the EXT_FAIL signal on the backplane bus"]
#[doc = " @note Channel 37/0x25 is used for EXT_RESET"]
#[doc = ""]
#[doc = " @param pfFail_p Pointer to the value destination"]
#[doc = " @return int32_t Driver result code of type tCtr700DrvResult"]
#[no_mangle]
pub extern "C" fn Ctr700DrvGetExtFail(pfFail_p: *mut u8) -> i32 {
    check_ptr!(pfFail_p, kCtr700DrvResult_InvalidParameter as i32);

    unsafe {
        let mut state: IoBool = IoBool::False;
        let ret = IoGetInput(37, &mut state) as i32;
        match state {
            IoBool::True => *pfFail_p = tCtr700Drv_Bool_kCtr700Drv_True,
            IoBool::False => *pfFail_p = tCtr700Drv_Bool_kCtr700Drv_False,
        }
        ret
    }
}

#[doc = " @brief Set the value of the EXT_RESET signal on the backplane bus"]
#[doc = " @note Channel 32/0x20 is used for EXT_RESET"]
#[doc = ""]
#[doc = " @param fEnable_p The value to set"]
#[doc = " @return int32_t Driver result code of type tCtr700DrvResult"]
#[no_mangle]
pub extern "C" fn Ctr700DrvSetExtReset(fEnable_p: u8) -> i32 {
    unsafe { IoSetOutput(32, IoBool::from(fEnable_p)) as i32 }
}

#[doc = " @brief Get the value of a digital input"]
#[doc = ""]
#[doc = " @param uChannel_p The channel of the digital input"]
#[doc = " @param pfState_p Pointer to the state destination"]
#[doc = " @return int32_t Driver result code of type tCtr700DrvResult"]
#[no_mangle]
#[allow(clippy::not_unsafe_ptr_arg_deref)] // for some reason the unsafe block is ignored by clippy
pub extern "C" fn Ctr700DrvGetDigiIn(uChannel_p: u8, pfState_p: *mut u8) -> i32 {
    check_ptr!(pfState_p, kCtr700DrvResult_InvalidParameter as i32);

    unsafe {
        let mut state: IoBool = IoBool::False;
        let ret = IoGetInput(uChannel_p, &mut state) as i32;
        match state {
            IoBool::True => *pfState_p = tCtr700Drv_Bool_kCtr700Drv_True,
            IoBool::False => *pfState_p = tCtr700Drv_Bool_kCtr700Drv_False,
        }
        ret
    }
}

#[doc = " @brief Set the value of a digital output"]
#[doc = ""]
#[doc = " @param uChannel_p The channel of the digital output"]
#[doc = " @param fEnable_p The value to set"]
#[doc = " @return int32_t Driver result code of type tCtr700DrvResult"]
#[no_mangle]
pub extern "C" fn Ctr700DrvSetDigiOut(uChannel_p: u8, fEnable_p: u8) -> i32 {
    unsafe { IoSetOutput(uChannel_p, IoBool::from(fEnable_p)) as i32 }
}

#[doc = " @brief Set the value of a relay output"]
#[doc = ""]
#[doc = " @param uChannel_p The channel of the relay"]
#[doc = " @param fEnable_p The value to set"]
#[doc = " @return int32_t Driver result code of type tCtr700DrvResult"]
#[no_mangle]
pub extern "C" fn Ctr700DrvSetRelay(uChannel_p: u8, fEnable_p: u8) -> i32 {
    let srcinfo = HWINFO.lock().unwrap();
    unsafe {
        IoSetOutput(
            uChannel_p + srcinfo.m_uLegacyRelayOffset,
            IoBool::from(fEnable_p),
        );
        IoResult::Success as i32
    }
}

#[doc = " @brief Enable/disable a counter channel"]
#[doc = ""]
#[doc = " @param uChannel_p The channel to control"]
#[doc = " @param fEnable_p @see kCtr700Drv_True to enable,"]
#[doc = "                  @see kCtr700Drv_False to disable"]
#[doc = " @return int32_t Driver result code of type tCtr700DrvResult"]
#[no_mangle]
pub extern "C" fn Ctr700DrvCntEnable(uChannel_p: u8, fEnable_p: u8) -> i32 {
    unsafe { IoCntEnable(uChannel_p, IoBool::from(fEnable_p)) as i32 }
}

#[doc = " @brief Setup the counters mode"]
#[doc = ""]
#[doc = " @param uChannel_p The channel to setup"]
#[doc = " @param uMode_p The mode of the counter, see #tCtr700DrvCounterMode"]
#[doc = " @param uTrigger_p The trigger of the counter, see #tCtr700DrvCounterTrigger"]
#[doc = " @param uDir_p The direction of counting, see #tCtr700DrvCounterDirection"]
#[doc = " @return int32_t Driver result code of type tCtr700DrvResult"]
#[no_mangle]
pub extern "C" fn Ctr700DrvCntSetMode(
    uChannel_p: u8,
    uMode_p: u8,
    uTrigger_p: u8,
    uDir_p: u8,
) -> i32 {
    unsafe {
        IoCntSetup(
            uChannel_p,
            IoCntMode::from(uMode_p),
            IoCntTrigger::from(uTrigger_p),
            IoCntDirection::from(uDir_p),
        ) as i32
    }
}

#[doc = " @brief Set the initial value of the counter"]
#[doc = ""]
#[doc = " @param uChannel_p The channel to setup"]
#[doc = " @param iPreload_p The initial value to set"]
#[doc = " @return int32_t Driver result code of type tCtr700DrvResult"]
#[no_mangle]
pub extern "C" fn Ctr700DrvCntSetPreload(uChannel_p: u8, iPreload_p: i32) -> i32 {
    unsafe { IoCntSetPreload(uChannel_p, iPreload_p) as i32 }
}

#[doc = " @brief Get the value of a counter channel"]
#[doc = ""]
#[doc = " @param uChannel_p The channel to get the value for"]
#[doc = " @param piValue_p Pointer to the value destination"]
#[doc = " @return int32_t Driver result code of type tCtr700DrvResult"]
#[no_mangle]
pub extern "C" fn Ctr700DrvCntGetValue(uChannel_p: u8, piValue_p: *mut i32) -> i32 {
    unsafe { IoCntGetValue(uChannel_p, piValue_p) as i32 }
}

#[doc = " @brief Set the timebase for PWM output"]
#[doc = ""]
#[doc = " @param uChannel_p The channel to get the value for"]
#[doc = " @param uTimeBase_p Timebase enum"]
#[doc = " @return int32_t Driver result code of type tCtr700DrvResult"]
#[no_mangle]
pub extern "C" fn Ctr700DrvPwmSetTimeBase(uChannel_p: u8, uTimeBase_p: u8) -> i32 {
    unsafe { IoPwmSetTimebase(uChannel_p, IoPwmTimebase::from(uTimeBase_p)) as i32 }
}

#[doc = " @brief Setup an PWM channel"]
#[doc = ""]
#[doc = " @param uChannel_p The channel to setup"]
#[doc = " @param uPeriod_p The period length in units set by #Ctr700DrvPwmSetTimeBase"]
#[doc = " @param uPulseLen_p The pulse length of the signal ('on' time / duty cycle)"]
#[doc = " @return int32_t Driver result code of type tCtr700DrvResult"]
#[no_mangle]
pub extern "C" fn Ctr700DrvPwmSetParam(uChannel_p: u8, uPeriod_p: u16, uPulseLen_p: u16) -> i32 {
    unsafe { IoPwmSetup(uChannel_p, uPeriod_p, uPulseLen_p) as i32 }
}

#[doc = " @brief Enable/Disable PWM output"]
#[doc = ""]
#[doc = " @param uChannel_p The channel to enable/disable"]
#[doc = " @param fRun_p @see kCtr700Drv_True to enable,"]
#[doc = "               @see kCtr700Drv_False to disable"]
#[doc = " @return int32_t Driver result code of type tCtr700DrvResult"]
#[no_mangle]
pub extern "C" fn Ctr700DrvPwmEnable(uChannel_p: u8, fRun_p: u8) -> i32 {
    unsafe { IoPwmEnable(uChannel_p, fRun_p != 0) as i32 }
}

#[doc = " @brief *Not implemented*"]
#[doc = ""]
#[doc = " @param uChannel_p -"]
#[doc = " @param uPeriod_p -"]
#[doc = " @param iDelta_p -"]
#[doc = " @param uPulseCnt_p -"]
#[doc = " @return int32_t Driver result code of type tCtr700DrvResult"]
#[no_mangle]
pub extern "C" fn Ctr700DrvPtoSetParam(
    _uChannel_p: u8,
    _uPeriod_p: u16,
    _iDelta_p: i16,
    _uPulseCnt_p: u32,
) -> i32 {
    kCtr700DrvResult_NotImplemented as i32
}

#[doc = " @brief *Not implemented*"]
#[doc = ""]
#[doc = " @param uChannel_p -"]
#[doc = " @param fRun_p -"]
#[doc = " @return int32_t Driver result code of type tCtr700DrvResult"]
#[no_mangle]
pub extern "C" fn Ctr700DrvPtoEnable(_uChannel_p: u8, _fRun_p: u8) -> i32 {
    kCtr700DrvResult_NotImplemented as i32
}

#[doc = " @brief *Not implemented*"]
#[doc = ""]
#[doc = " @param uChannel_p -"]
#[doc = " @param pfRun_p -"]
#[doc = " @return int32_t Driver result code of type tCtr700DrvResult"]
#[no_mangle]
pub extern "C" fn Ctr700DrvPtoGetState(_uChannel_p: u8, _pfRun_p: *mut u8) -> i32 {
    kCtr700DrvResult_NotImplemented as i32
}

#[doc = " @brief Get the value of an ADC channel"]
#[doc = ""]
#[doc = " @param uChannel_p The channel to get"]
#[doc = " @param puAdcValue_p Pointer to the value destination"]
#[doc = " @return int32_t Driver result code of type tCtr700DrvResult"]
#[no_mangle]
pub extern "C" fn Ctr700DrvAdcGetValue(uChannel_p: u8, puAdcValue_p: *mut u16) -> i32 {
    unsafe { IoAdcGetValue(uChannel_p, puAdcValue_p) as i32 }
}

#[doc = " @brief Setup an ADC channel for voltage or current measurement"]
#[doc = " @note The ADC channel has a default configuration determined by the operating"]
#[doc = "       system configuration. See file /etc/systec/adc_modes."]
#[doc = ""]
#[doc = " @param uChannel_p The channel to setup"]
#[doc = " @param uMode_p The mode of type #tCtr700DrvAnalogMode"]
#[doc = " @return int32_t Driver result code of type tCtr700DrvResult"]
#[no_mangle]
pub extern "C" fn Ctr700DrvAdcSetMode(uChannel_p: u8, uMode_p: u8) -> i32 {
    unsafe { IoAdcSetMode(uChannel_p, IoAnalogMode::from(uMode_p)) as i32 }
}

#[doc = " @brief Get the value of a temperature sensor"]
#[doc = ""]
#[doc = " @param uSensor_p The temperature sensor channel"]
#[doc = " @param piValue_p Pointer to the value destination"]
#[doc = " @return int32_t Driver result code of type tCtr700DrvResult"]
#[no_mangle]
pub extern "C" fn Ctr700DrvTmpGetValue(uSensor_p: u8, piValue_p: *mut i32) -> i32 {
    unsafe { IoTmpGetValue(uSensor_p, piValue_p) as i32 }
}

#[doc = " @brief Register a callback to signal changes on an digital input"]
#[doc = " @note Channels 0..15 are used for digital inputs,"]
#[doc = "       Channel 128/0x80 is used for the RUN switch"]
#[doc = ""]
#[doc = " @param uChannel_p The channel of the digital input"]
#[doc = " @param pfnCallback_p The callback function to register of type"]
#[doc = "                      #tCtr700DrvInterruptCallback"]
#[doc = " @param uInterruptTrigger_p Set the kind of trigger for the input"]
#[doc = "                            #tCtr700DrvInterruptTrigger"]
#[doc = " @return int32_t Driver result code of type tCtr700DrvResult"]
#[no_mangle]
pub extern "C" fn Ctr700DrvRegisterInterruptCallback(
    uChannel_p: u8,
    pfnCallback_p: tCtr700DrvInterruptCallback,
    uInterruptTrigger_p: u32,
) -> i32 {
    let mut channel: u8 = uChannel_p;
    match channel {
        SPECIAL_INPUT_RUN_SWITCH => channel = RUN_SWITCH_CHANNEL,
        _ => (),
    };

    unsafe {
        IoRegisterInputCallback(
            channel,
            pfnCallback_p,
            IoInputTrigger::from(uInterruptTrigger_p),
        ) as i32
    }
}

#[doc = " @brief Un-register / disable interrupt handling for a digital input"]
#[doc = ""]
#[doc = " @param uChannel_p Analogous to #Ctr700DrvRegisterInterruptCallback"]
#[doc = " @return int32_t Driver result code of type tCtr700DrvResult"]
#[no_mangle]
pub extern "C" fn Ctr700DrvUnregisterInterruptCallback(uChannel_p: u8) -> i32 {
    let mut channel: u8 = uChannel_p;
    match channel {
        SPECIAL_INPUT_RUN_SWITCH => channel = RUN_SWITCH_CHANNEL,
        _ => (),
    };
    unsafe { IoUnregisterInputCallback(channel) as i32 }
}
