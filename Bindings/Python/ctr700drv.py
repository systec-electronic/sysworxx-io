#!/usr/bin/env python
# -*- coding: utf-8 -*-

# SPDX-License-Identifier: LGPL-3.0-or-later
#
# (c) SYSTEC electronic AG, D-08468 Heinsdorfergrund, Am Windrad 2
#     www.systec-electronic.com

"""
Project:      SYSTEC sysWORXX CTR-700
Description:  Python driver demo application
"""

from cffi import FFI


_ffibuilder = FFI()
_ffibuilder.cdef("""
    // Return codes
    typedef enum
    {
        kCtr700DrvResult_Success            = 0x00,

        kCtr700DrvResult_Error              = 0xff,
        kCtr700DrvResult_NotImplemented     = 0xfe,
        kCtr700DrvResult_InvalidParameter   = 0xfd,
        kCtr700DrvResult_InvalidChannel     = 0xfc,
        kCtr700DrvResult_InvalidMode        = 0xfb,
        kCtr700DrvResult_InvalidTimebase    = 0xfa,
        kCtr700DrvResult_InvalidDelta       = 0xf9,
        kCtr700DrvResult_PtoParamTabFull    = 0xf8,
        kCtr700DrvResult_DevAccessFailed    = 0xf7,
        kCtr700DrvResult_InvalidProcImgCfg  = 0xf6,
        kCtr700DrvResult_ProcImgCfgUnknown  = 0xf5,
        kCtr700DrvResult_ShpImgError        = 0xf4,
        kCtr700DrvResult_AddressOutOfRange  = 0xf3,
        kCtr700DrvResult_WatchdogTimeout    = 0xf2
    } tCtr700DrvResult;

    // uint8_tean type
    typedef enum
    {
        kCtr700Drv_False                    = 0,
        kCtr700Drv_True                     = 1
    } tCtr700Drv_Bool;

    // Analog input channel type
    typedef enum
    {
        kCtr700DrvAnalogIn_Channel0         = 0,
        kCtr700DrvAnalogIn_Channel1         = 1,
        kCtr700DrvAnalogIn_Channel2         = 2,
        kCtr700DrvAnalogIn_Channel3         = 3
    } tCtr700DrvAnalogIn;

    // Analog channel mode type
    typedef enum
    {
        kCtr700DrvAnalogMode_Voltage        = 0,
        kCtr700DrvAnalogMode_Current        = 1
    } tCtr700DrvAnalogMode;

    // Counter channel type
    typedef enum
    {
        kCtr700DrvCounter_Channel0          = 0
    } tCtr700DrvCounter;

    // Counter mode type
    typedef enum
    {
        kCtr700DrvCounterMode_Counter       = 0,
        kCtr700DrvCounterMode_AB_Decoder    = 1
    } tCtr700DrvCounterMode;

    // Counter trigger type
    typedef enum
    {
        kCtr700DrvCounterTrigger_RisingEdge = 0,
        kCtr700DrvCounterTrigger_FallingEdge= 1,
        kCtr700DrvCounterTrigger_AnyEdge    = 2
    } tCtr700DrvCounterTrigger;

    // Counter direction type
    typedef enum
    {
        kCtr700DrvCounterDirection_Up       = 0,
        kCtr700DrvCounterDirection_Down     = 1
    } tCtr700DrvCounterDirection;

    // PWM channel type
    typedef enum
    {
        kCtr700DrvPwm_Channel0              = 0,
        kCtr700DrvPwm_Channel1              = 1
    } tCtr700DrvPwm;

    // PWM timebase type
    typedef enum
    {
        kCtr700DrvPwmTimebase_800NS         = 1,
        kCtr700DrvPwmTimebase_1MS           = 2
    } tCtr700DrvPwmTimebase;

    // Temperature sensor channel
    typedef enum
    {
         kCtr700DrvTmp_Channel0             = 0,
         kCtr700DrvTmp_Channel1             = 1
    } tCtr700DrvTmp;

    // Hardware information structure
    typedef struct
    {
        uint16_t    m_uPcbRevision;
        uint16_t    m_uDiChannels;
        uint16_t    m_uDoChannels;
        uint16_t    m_uRelayChannels;
        uint16_t    m_uAiChannels;
        uint16_t    m_uAoChannels;
        uint16_t    m_uCntChannels;
        uint16_t    m_uEncChannels;
        uint16_t    m_uPwmChannels;
        uint16_t    m_uTmpChannels;
    } tCtr700DrvHwInfo;

    // Diagnose information structure
    typedef struct
    {
        uint8_t     m_fDigiOutPowerFail;
        uint8_t     m_fDigiOutDiag;
        uint8_t     m_fDigiInError;
        uint8_t     m_fUsbOverCurrent;
    } tCtr700DrvDiagInfo;

    // Callback function type
    typedef void (*tCtr700DrvInterruptCallback)(uint8_t, uint8_t);

    // Execute callback on interrupt kind
    typedef enum
    {
        kCtr700DrvInterruptNone         = 0x00,
        kCtr700DrvInterruptRisingEdge   = 0x01,
        kCtr700DrvInterruptFallingEdge  = 0x02,
        kCtr700DrvInterruptBothEdge     = 0x03
    } tCtr700DrvInterruptTrigger;


    //---------------------------------------------------------------------------
    //  Prototypes of driver functions
    //---------------------------------------------------------------------------

    // UNIT: Basic/Common Functions
    int32_t Ctr700DrvGetVersion         (uint8_t* puMajor, uint8_t* puMinor);
    int32_t Ctr700DrvInitialize         (void);
    int32_t Ctr700DrvShutDown           (void);
    int32_t Ctr700DrvGetTickCount       (uint32_t* puTickCount_p);
    int32_t Ctr700DrvEnableWatchdog     (uint8_t fMonitorOnly_p);
    int32_t Ctr700DrvServiceWatchdog    (void);
    int32_t Ctr700DrvGetHardwareInfo    (tCtr700DrvHwInfo* pHwInfo_p);

    // UNIT: Operator Controls
    int32_t Ctr700DrvSetRunLed          (uint8_t fState_p);
    int32_t Ctr700DrvSetErrLed          (uint8_t fState_p);
    int32_t Ctr700DrvGetRunSwitch       (uint8_t* pfRunSwitch_p);
    int32_t Ctr700DrvGetConfigEnabled   (uint8_t* pfConfig_p);
    int32_t Ctr700DrvGetPowerFail       (uint8_t* pfFail_p);
    int32_t Ctr700DrvGetDiagInfo        (tCtr700DrvDiagInfo* pDiagInfo_p);

    // UNIT: GPIO to backplane bus
    int32_t Ctr700DrvGetExtFail         (uint8_t* pfFail_p);
    int32_t Ctr700DrvSetExtReset        (uint8_t fEnable_p);

    // UNIT: Digital In/Out
    int32_t Ctr700DrvGetDigiIn          (uint8_t uChannel_p, uint8_t* pfState_p);
    int32_t Ctr700DrvSetDigiOut         (uint8_t uChannel_p, uint8_t fEnable_p);
    int32_t Ctr700DrvSetRelay           (uint8_t uChannel_p, uint8_t fEnable_p);

    // UNIT: Counter
    int32_t Ctr700DrvCntEnable          (uint8_t uChannel_p, uint8_t fEnable_p);
    int32_t Ctr700DrvCntSetMode         (uint8_t uChannel_p, uint8_t uMode_p, uint8_t uTrigger_p, uint8_t uDir_p);
    int32_t Ctr700DrvCntSetPreload      (uint8_t uChannel_p, int32_t iPreload_p);
    int32_t Ctr700DrvCntGetValue        (uint8_t uChannel_p, int32_t* piValue_p);

    // UNIT: PWM/PTO
    int32_t Ctr700DrvPwmSetTimeBase     (uint8_t uChannel_p, uint8_t uTimeBase_p);
    int32_t Ctr700DrvPwmSetParam        (uint8_t uChannel_p, uint16_t uPeriod_p, uint16_t uPulseLen_p);
    int32_t Ctr700DrvPwmEnable          (uint8_t uChannel_p, uint8_t fRun_p);
    int32_t Ctr700DrvPtoSetParam        (uint8_t uChannel_p, uint16_t uPeriod_p, int16_t iDelta_p, uint32_t uPulseCnt_p);
    int32_t Ctr700DrvPtoEnable          (uint8_t uChannel_p, uint8_t fRun_p);
    int32_t Ctr700DrvPtoGetState        (uint8_t uChannel_p, uint8_t* pfRun_p);

    // UNIT: Analog In/Out
    int32_t Ctr700DrvAdcGetValue        (uint8_t uChannel_p, uint16_t* puAdcValue_p);
    int32_t Ctr700DrvAdcSetMode         (uint8_t uChannel_p, uint8_t uMode_p);

    // UNIT: Temperature Sensor
    int32_t Ctr700DrvTmpGetValue        (uint8_t uSensor_p, int32_t* piValue_p);

    // UNIT: Interrupt handling
    int32_t Ctr700DrvRegisterInterruptCallback (
            uint8_t                     uChannel_p,
            tCtr700DrvInterruptCallback pfnCallback_p,
            uint32_t                    uInterruptTrigger_p);
    int32_t Ctr700DrvUnregisterInterruptCallback (
            uint8_t                     uChannel_p);
""")

_ffi = _ffibuilder.dlopen("ctr700drv")


class Ctr700Exception(Exception):
    error_messages = {
            _ffi.kCtr700DrvResult_Error:              "Generic error",
            _ffi.kCtr700DrvResult_NotImplemented:     "Not implemented",
            _ffi.kCtr700DrvResult_InvalidParameter:   "Invalid parameter",
            _ffi.kCtr700DrvResult_InvalidChannel:     "Invalid channel",
            _ffi.kCtr700DrvResult_InvalidMode:        "Invalid mode",
            _ffi.kCtr700DrvResult_InvalidTimebase:    "Invalid timebase",
            _ffi.kCtr700DrvResult_InvalidDelta:       "Invalid delta",
            _ffi.kCtr700DrvResult_PtoParamTabFull:    "PTO tab is is completely filled",
            _ffi.kCtr700DrvResult_DevAccessFailed:    "Access to device failed",
            _ffi.kCtr700DrvResult_InvalidProcImgCfg:  "Process image configuration invalid",
            _ffi.kCtr700DrvResult_ProcImgCfgUnknown:  "Process image configuration unknown",
            _ffi.kCtr700DrvResult_ShpImgError:        "Shared process image error",
            _ffi.kCtr700DrvResult_AddressOutOfRange:  "Address out of range",
            _ffi.kCtr700DrvResult_WatchdogTimeout:    "Watchdog timeout",
    }

    def __init__(self, result):
        msg = Ctr700Exception.error_messages.get(result,
                                                 "Unknown error code")
        msg = "{} (Error code: 0x{:x})".format(msg, result)
        super(Ctr700Exception, self).__init__(msg)


class CounterMode:
    COUNTER = _ffi.kCtr700DrvCounterMode_Counter
    AB_DECODER = _ffi.kCtr700DrvCounterMode_AB_Decoder


class CounterTrigger:
    RISING_EDGE = _ffi.kCtr700DrvCounterTrigger_RisingEdge
    FALLING_EDGE = _ffi.kCtr700DrvCounterTrigger_FallingEdge
    ANY_EDGE = _ffi.kCtr700DrvCounterTrigger_AnyEdge


class CounterDirection:
    UP = _ffi.kCtr700DrvCounterDirection_Up
    DOWN = _ffi.kCtr700DrvCounterDirection_Down

class DiagInfo(object):
    def __init__(self, info):
        self.DigiOutPowerFail = bool(info.m_fDigiOutPowerFail)
        self.DigiOutDiag = bool(info.m_fDigiOutDiag)
        self.DigiInError = bool(info.m_fDigiInError)
        self.UsbOverCurrent = bool(info.m_fUsbOverCurrent)

    def __repr__(self):
        return "DO_PF: {}, /DO_DIAG: {}, /DI_ERR: {}, /USB_OC: {}".format(
            self.DigiOutPowerFail, self.DigiOutDiag,
            self.DigiInError, self.UsbOverCurrent
        )

class _Singleton(type):
    _instances = {}
    def __call__(cls, *args, **kwargs):
        if cls not in cls._instances:
            cls._instances[cls] = super(_Singleton, cls).__call__(*args, **kwargs)
        return cls._instances[cls]

class Singleton(_Singleton('SingletonMeta', (object,), {})): pass

class Ctr700Drv(Singleton):
    def __init__(self):
        self._callbacks = {}
        self._instanceCount = 0

    def get_version(self):
        major = _ffibuilder.new('uint8_t*')
        minor = _ffibuilder.new('uint8_t*')

        result = _ffi.Ctr700DrvGetVersion(major, minor)
        self._checkResult(result)

        return int(major[0]), int(minor[0])

    def init(self):
        if self._instanceCount == 0:
            result = _ffi.Ctr700DrvInitialize()
            self._checkResult(result)

        self._instanceCount += 1

    def shutdown(self):
        self._instanceCount -= 1

        if self._instanceCount == 0:
            result = _ffi.Ctr700DrvShutDown()
            self._checkResult(result)

        if self._instanceCount < 0:
            raise RuntimeError("Ctr700Drv has not been initialized")

    def get_tick_count(self):
        tick_count = _ffibuilder.new('uint32_t*')

        result = _ffi.Ctr700DrvGetTickCount(tick_count)
        self._checkResult(result)

        return int(tick_count[0])

    def enable_watchdog(self, monitor_only=False):
        result = _ffi.Ctr700DrvEnableWatchdog(monitor_only)
        self._checkResult(result)

    def service_watchdog(self):
        result = _ffi.Ctr700DrvServiceWatchdog()
        self._checkResult(result)

    def get_hardware_info(self):
        hw_info = _ffibuilder.new('tCtr700DrvHwInfo*')
        result = _ffi.Ctr700DrvGetHardwareInfo(hw_info)
        self._checkResult(result)

        info = {
            'PcbRevision': hw_info.m_uPcbRevision,
            'DiChannels': hw_info.m_uDiChannels,
            'DoChannels': hw_info.m_uDoChannels,
            'RelayChannels': hw_info.m_uRelayChannels,
            'AiChannels': hw_info.m_uAiChannels,
            'AoChannels': hw_info.m_uAoChannels,
            'CntChannels': hw_info.m_uCntChannels,
            'EncChannels': hw_info.m_uEncChannels,
            'PwmChannels': hw_info.m_uPwmChannels,
            'TmpChannels': hw_info.m_uTmpChannels
        }

        return info

    def set_run_led(self, enable):
        state = 1 if enable else 0
        result = _ffi.Ctr700DrvSetRunLed(state)
        self._checkResult(result)

    def set_err_led(self, enable):
        state = 1 if enable else 0
        result = _ffi.Ctr700DrvSetErrLed(state)
        self._checkResult(result)

    def get_run_switch(self):
        state = _ffibuilder.new('uint8_t*')
        result = _ffi.Ctr700DrvGetRunSwitch(state)
        self._checkResult(result)
        return True if int(state[0]) else False

    def get_config_enabled(self):
        state = _ffibuilder.new('uint8_t*')
        result = _ffi.Ctr700DrvGetConfigEnabled(state)
        self._checkResult(result)
        return True if int(state[0]) else False

    def get_power_fail(self):
        state = _ffibuilder.new('uint8_t*')
        result = _ffi.Ctr700DrvGetPowerFail(state)
        self._checkResult(result)
        return True if int(state[0]) else False

    def get_diag_info(self):
        info = _ffibuilder.new('tCtr700DrvDiagInfo*')
        result = _ffi.Ctr700DrvGetDiagInfo(info)
        self._checkResult(result)
        return DiagInfo(info)

    def get_ext_fail(self):
        state = _ffibuilder.new('uint8_t*')
        result = _ffi.Ctr700DrvGetExtFail(state)
        self._checkResult(result)
        return True if int(state[0]) else False

    def set_ext_reset(self, enable):
        state = 1 if enable else 0
        result = _ffi.Ctr700DrvSetExtReset(state)
        self._checkResult(result)

    def get_digi_in(self, channel):
        state = _ffibuilder.new('uint8_t*')
        result = _ffi.Ctr700DrvGetDigiIn(channel, state)
        self._checkResult(result)
        return True if int(state[0]) else False

    def set_digi_out(self, channel, enable):
        state = 1 if enable else 0
        result = _ffi.Ctr700DrvSetDigiOut(channel, state)
        self._checkResult(result)

    def set_relay(self, channel, enable):
        state = 1 if enable else 0
        result = _ffi.Ctr700DrvSetRelay(channel, state)
        self._checkResult(result)

    def counter_enable(self, channel):
        result = _ffi.Ctr700DrvCntEnable(channel, 1)
        self._checkResult(result)

    def counter_disable(self, channel):
        result = _ffi.Ctr700DrvCntEnable(channel, 0)
        self._checkResult(result)

    def counter_set_mode(self, channel, mode, trigger, direction):
        result = _ffi.Ctr700DrvCntSetMode(channel, mode, trigger, direction)
        self._checkResult(result)

    def counter_set_preload(self, channel, preload):
        result = _ffi.Ctr700DrvCntSetPreload(channel, preload)
        self._checkResult(result)

    def counter_get_value(self, channel):
        value = _ffibuilder.new('int32_t*')
        result = _ffi.Ctr700DrvCntGetValue(channel, value)
        self._checkResult(result)
        return int(value[0])

    def pwm_enable(self, channel, period, pulse_len):
        result = _ffi.Ctr700DrvPwmSetTimeBase(
                channel,
                _ffi.kCtr700DrvPwmTimebase_1MS)
        self._checkResult(result)

        result = _ffi.Ctr700DrvPwmSetParam(channel, period, pulse_len)
        self._checkResult(result)

        result = _ffi.Ctr700DrvPwmEnable(channel, 1)
        self._checkResult(result)

    def pwm_disable(self, channel):
        result = _ffi.Ctr700DrvPwmEnable(channel, 0)
        self._checkResult(result)

    def adc_get_value(self, channel):
        value = _ffibuilder.new('uint16_t*')
        result = _ffi.Ctr700DrvAdcGetValue(channel, value)
        self._checkResult(result)
        return int(value[0])

    def adc_setup_voltage(self, channel):
        result = _ffi.Ctr700DrvAdcSetMode(
                channel,
                _ffi.kCtr700DrvAnalogMode_Voltage)
        self._checkResult(result)

    def adc_setup_current(self, channel):
        result = _ffi.Ctr700DrvAdcSetMode(
                channel,
                _ffi.kCtr700DrvAnalogMode_Current)
        self._checkResult(result)

    def temperature_get(self, channel):
        value = _ffibuilder.new('int32_t*')
        result = _ffi.Ctr700DrvTmpGetValue(channel, value)
        self._checkResult(result)
        return float(value[0]) / 10000.0

    def register_interrupt(self, channel, rising, falling, callback):
        cb = _ffibuilder.callback('void(uint8_t, uint8_t)')(callback)
        self._callbacks[channel] = cb

        trigger = 0
        if rising:
            trigger += 1
        if falling:
            trigger += 2

        result = _ffi.Ctr700DrvRegisterInterruptCallback(channel, cb, trigger)
        self._checkResult(result)

    def unregister_interrupt(self, channel):
        result = _ffi.Ctr700DrvUnregisterInterruptCallback(channel)
        self._callbacks[channel] = None
        self._checkResult(result)

    def _checkResult(self, result):
        if result == _ffi.kCtr700DrvResult_Success:
            return

        raise Ctr700Exception(result)
