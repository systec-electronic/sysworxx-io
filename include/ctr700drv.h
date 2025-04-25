// SPDX-License-Identifier: LGPL-3.0-or-later
// SPDX-FileCopyrightText: 2025 SYS TEC electronic AG <https://www.systec-electronic.com/>

/****************************************************************************

  Project:      SYSTEC sysWORXX CTR-700
  Description:  Declarations for board driver

  -------------------------------------------------------------------------

  Revision History:

  2017/09/21 -ad:   V1.00 Initial version
  2019/03/18 -ad:   V2.00 Add doxygen comments
  2019/10/14 -aw:   V2.01 Change form of company to AG

****************************************************************************/

#ifndef _CTR700DRV_H_
#define _CTR700DRV_H_

#include <stdint.h>

/**
 * @mainpage
 *
 * The I/O driver has an interface to access the different signals and
 * peripherals of the CTR-700. This section describes some general usage
 * guidelines to use it.
 *
 * 1. To initialize the driver one has to call the function
 *    Ctr700DrvInitialize(). Without this all other functions will result in an
 *    error.
 *
 * 2. When cleaning up or stopping an application the used code should call
 *    Ctr700DrvShutDown().
 *
 * 3. The functions Ctr700DrvInitialize() / Ctr700DrvShutDown() have to be called
 *    pairwise. It is not safe to call Ctr700DrvInitialize() multiple times
 *    without calling Ctr700DrvShutDown().
 *
 * 4. All driver functions use the common result type #tCtr700DrvResult.
 */

/**
 * @file ctr700drv.h
 * @addtogroup ctr700drv sysWORXX CTR-700 I/O driver
 * @brief sysWORXX CTR-700 I/O driver library
 * @{
 */

#ifdef __cplusplus
    extern "C" {
#endif

/**
 * @addtogroup types I/O driver types
 * @brief Parameter and result types of I/O driver functions.
 * @{
 */

/**
 * @brief Common error codes for all API functions
 */
typedef enum
{
    kCtr700DrvResult_Success            = 0x00, /**< Function call succeeded */

    kCtr700DrvResult_Error              = 0xff, /**< Generic error occurred */
    kCtr700DrvResult_NotImplemented     = 0xfe, /**< The functionality is not implemented by the library */
    kCtr700DrvResult_InvalidParameter   = 0xfd, /**< One of the given parameters is invalid (e.g. NULL pointer or parameter is out of range) */
    kCtr700DrvResult_InvalidChannel     = 0xfc, /**< The provided channel number is invalid */
    kCtr700DrvResult_InvalidMode        = 0xfb, /**< The provided mode is invalid */
    kCtr700DrvResult_InvalidTimebase    = 0xfa, /**< The provided timebase is invalid */
    kCtr700DrvResult_InvalidDelta       = 0xf9, /**< The provided delta parameter is invalid */
    kCtr700DrvResult_PtoParamTabFull    = 0xf8, /**< The PTO table is completely filled */
    kCtr700DrvResult_DevAccessFailed    = 0xf7, /**< Access to the device or peripheral has failed */
    kCtr700DrvResult_InvalidProcImgCfg  = 0xf6, /**< Reserved error code; currently unused. */
    kCtr700DrvResult_ProcImgCfgUnknown  = 0xf5, /**< Reserved error code; currently unused. */
    kCtr700DrvResult_ShpImgError        = 0xf4, /**< Reserved error code; currently unused. */
    kCtr700DrvResult_AddressOutOfRange  = 0xf3, /**< Reserved error code; currently unused. */
    kCtr700DrvResult_WatchdogTimeout    = 0xf2  /**< The watchdog did timeout */
} tCtr700DrvResult;

/**
 * @brief Simple boolean type for some functions of the API.
 */
typedef enum
{
    kCtr700Drv_False                    = 0,
    kCtr700Drv_True                     = !(kCtr700Drv_False)
} tCtr700Drv_Bool;

/**
 * @brief Analog input channel type
 */
typedef enum
{
    kCtr700DrvAnalogIn_Channel0         = 0,
    kCtr700DrvAnalogIn_Channel1         = 1,
    kCtr700DrvAnalogIn_Channel2         = 2,
    kCtr700DrvAnalogIn_Channel3         = 3
} tCtr700DrvAnalogIn;

/**
 * @brief Analog channel mode type
 */
typedef enum
{
    kCtr700DrvAnalogMode_Voltage        = 0,
    kCtr700DrvAnalogMode_Current        = 1
} tCtr700DrvAnalogMode;

/**
 * @brief Counter channel type
 */
typedef enum
{
    kCtr700DrvCounter_Channel0          = 0
} tCtr700DrvCounter;

/**
 * @brief Counter mode type
 */
typedef enum
{
    /**
     * The counter will count edges on digital input 14. The direction of
     * counting is determined by the value of digital input 15.
     */
    kCtr700DrvCounterMode_Counter       = 0,
    /**
     * The counter will count in A/B decoder mode. Digital input 14 is used
     * for the 'A' input and digital input 15 is used for 'B'. Switching the
     * inputs will result in inverse counting.
     */
    kCtr700DrvCounterMode_AB_Decoder    = 1
} tCtr700DrvCounterMode;

/**
 * @brief Counter trigger type (only applies to #kCtr700DrvCounterMode_Counter
 */
typedef enum
{
    kCtr700DrvCounterTrigger_RisingEdge  = 0,
    kCtr700DrvCounterTrigger_FallingEdge = 1,
    kCtr700DrvCounterTrigger_AnyEdge     = 2
} tCtr700DrvCounterTrigger;

/**
 * @brief Counter direction type can be used to invert the direction of
 *        counting.
 */
typedef enum
{
    kCtr700DrvCounterDirection_Up       = 0,
    kCtr700DrvCounterDirection_Down     = 1
} tCtr700DrvCounterDirection;

/**
 * @brief PWM channel type
 */
typedef enum
{
    kCtr700DrvPwm_Channel0              = 0,
    kCtr700DrvPwm_Channel1              = 1
} tCtr700DrvPwm;

/**
 * @brief PWM timebase type
 */
typedef enum
{
    kCtr700DrvPwmTimebase_800NS         = 1,
    kCtr700DrvPwmTimebase_1MS           = 2
} tCtr700DrvPwmTimebase;

/**
 * @brief Temperature sensor type
 */
typedef enum
{
     kCtr700DrvTmp_Channel0             = 0, /**< Internal temperature sensor of the i.MX7 */
     kCtr700DrvTmp_Channel1             = 1  /**< Temperature sensor on the system PCB of sysWORXX CTR-700 */
} tCtr700DrvTmp;

/**
 * @brief Hardware information structure
 *
 * This structure will be filled by #Ctr700DrvGetHardwareInfo. It contains the
 * revision information as well as the channel counts for the different
 * peripherals.
 */
typedef struct
{
    uint16_t    m_uPcbRevision;   /**< The PCB revision number */
    uint16_t    m_uDiChannels;    /**< Number of digital inputs */
    uint16_t    m_uDoChannels;    /**< Number of digital outputs */
    uint16_t    m_uRelayChannels; /**< Number of relay outputs */
    uint16_t    m_uAiChannels;    /**< Number of analog inputs */
    uint16_t    m_uAoChannels;    /**< Number of analog outputs */
    uint16_t    m_uCntChannels;   /**< Number of counter channels */
    uint16_t    m_uEncChannels;   /**< Number of A/B decoder channels */
    uint16_t    m_uPwmChannels;   /**< Number of PWM channels */
    uint16_t    m_uTmpChannels;   /**< Number of temperature channels */
} tCtr700DrvHwInfo;

/**
 * @brief Diagnose information structure
 */
typedef struct
{
    /**
     * @brief Signals powerfail errors of the driver for digital outputs (**active high**)
     *
     * This signal will be active, when the power supply for digital outputs
     * is not properly connected.
     */
    uint8_t     m_fDigiOutPowerFail;

    /**
     * @brief Signals an error for digital outputs (**active low**)
     *
     * This error signal will be active in one of two cases:
     * - overtemperature error of the driver IC
     * - internal communication error of the driver IC
     */
    uint8_t     m_fDigiOutDiag;

    /**
     * @brief Signals an error for digital inputs (**active low**)
     *
     * This error signal will be active in one of two cases:
     * - power supply is not connected to the driver IC
     * - internal communication error of the driver IC
     */
    uint8_t     m_fDigiInError;

    /**
     * @brief Signals an over-current error on USB interface (**active low**)
     */
    uint8_t     m_fUsbOverCurrent;
} tCtr700DrvDiagInfo;

/**
 * @brief Callback function type for asynchronous handling of digital inputs
 */
typedef void (*tCtr700DrvInterruptCallback)(uint8_t, uint8_t);

/**
 * @brief Trigger type for asynchronous digital input handling
 */
typedef enum
{
    kCtr700DrvInterruptNone         = 0x00, /**< Disable interrupt handling for the channel */
    kCtr700DrvInterruptRisingEdge   = 0x01, /**< Enable interrupt handling if the input value changes from low to high */
    kCtr700DrvInterruptFallingEdge  = 0x02, /**< Enable interrupt handling if the input value changes from high to low */
    kCtr700DrvInterruptBothEdge     = 0x03  /**< Enable interrupt handling if the input value changes any way */
} tCtr700DrvInterruptTrigger;

/** @} */

/**
 * @addtogroup basics Basic I/O driver functions
 * @brief Basic functions of I/O driver functions, which are not related to a
 *        specific kind of peripheral.
 * @{
 */

/**
 * @brief Initializes the I/O driver.
 * @note This function has to be called before any of the other API
 *       functions can be used.
 *
 * @return int32_t Driver result code of type tCtr700DrvResult
 */
int32_t Ctr700DrvInitialize         (void);

/**
 * @brief De-initialization of the I/O driver
 *
 * @return int32_t Driver result code of type tCtr700DrvResult
 */
int32_t Ctr700DrvShutDown           (void);

/**
 * @brief Get the version of the I/O driver
 *
 * @param puMajor Pointer to the resulting major part of the version number
 * @param puMinor Pointer to the resulting minor part of the version number
 * @return int32_t Driver result code of type tCtr700DrvResult
 */
int32_t Ctr700DrvGetVersion         (uint8_t* puMajor, uint8_t* puMinor);

/**
 * @brief Get the tickcount of the system in milliseconds
 *
 * This is a increasing time value starting at an unknown point in
 * time.
 *
 * @param puTickCount_p Pointer to the resulting timestamp value
 * @return int32_t Driver result code of type tCtr700DrvResult
 */
int32_t Ctr700DrvGetTickCount       (uint32_t* puTickCount_p);

/**
 * @brief Enable the systems watchdog
 *
 * @param fMonitorOnly_p Enable monitoring only mode. If the watchdog was not
 *        serviced in time, an error will be reported by the return value of
 *        Ctr700DrvServiceWatchdog().
 *
 * The watchdog interval has a fixed timeout setting of:
 * - 1000 ms for non-monitoring mode
 * - 900 ms for monitoring mode
 *
 * @return int32_t Driver result code of type tCtr700DrvResult
 */
int32_t Ctr700DrvEnableWatchdog     (uint8_t fMonitorOnly_p);

/**
 * @brief Service the system watchdog
 *
 * @return int32_t Driver result code of type tCtr700DrvResult
 */
int32_t Ctr700DrvServiceWatchdog    (void);

/**
 * @brief Get information about device revision and available I/O channels
 *
 * @param pHwInfo_p Destination structure with the resulting information
 * @return int32_t Driver result code of type tCtr700DrvResult
 */
int32_t Ctr700DrvGetHardwareInfo    (tCtr700DrvHwInfo* pHwInfo_p);

/** @} */

/**
 * @addtogroup operator_controls Operator controls
 * @brief Set and get operator controls of the device
 * @{
 */

/**
 * @brief Set the RUN LED
 *
 * @param fState_p The state to set
 * @return int32_t Driver result code of type tCtr700DrvResult
 */
int32_t Ctr700DrvSetRunLed          (uint8_t fState_p);

/**
 * @brief Set the ERROR LED
 *
 * @param fState_p The state to set
 * @return int32_t Driver result code of type tCtr700DrvResult
 */
int32_t Ctr700DrvSetErrLed          (uint8_t fState_p);

/**
 * @brief Get value of the RUN switch
 *
 * @param pfRunSwitch_p Pointer to the value destination
 * @return int32_t Driver result code of type tCtr700DrvResult
 */
int32_t Ctr700DrvGetRunSwitch       (uint8_t* pfRunSwitch_p);

/**
 * @brief Get value of the config switch (DIP 4)
 *
 * @param pfConfig_p Pointer to the value destination
 * @return int32_t Driver result code of type tCtr700DrvResult
 */
int32_t Ctr700DrvGetConfigEnabled   (uint8_t* pfConfig_p);

/**
 * @brief Get the state of power fail signal
 *
 * @param pfFail_p Pointer to the value destination
 * @return int32_t Driver result code of type tCtr700DrvResult
 */
int32_t Ctr700DrvGetPowerFail       (uint8_t* pfFail_p);

/**
 * @brief Get current state of diagnostic signals
 *
 * @param pDiagInfo_p Pointer to the value destination
 * @return int32_t Driver result code of type tCtr700DrvResult
 */
int32_t Ctr700DrvGetDiagInfo        (tCtr700DrvDiagInfo* pDiagInfo_p);

/** @} */

/**
 * @addtogroup backplane Backplane bus
 * @brief Access the in-/outputs which are connected to the backplane bus
 * @{
 */

/**
 * @brief Get value of the EXT_FAIL signal on the backplane bus
 *
 * @param pfFail_p Pointer to the value destination
 * @return int32_t Driver result code of type tCtr700DrvResult
 */
int32_t Ctr700DrvGetExtFail         (uint8_t* pfFail_p);

/**
 * @brief Set the value of the EXT_RESET signal on the backplane bus
 *
 * @param fEnable_p The value to set
 * @return int32_t Driver result code of type tCtr700DrvResult
 */
int32_t Ctr700DrvSetExtReset        (uint8_t fEnable_p);

/** @} */

/**
 * @addtogroup gpio GPIO
 * @brief Access to the digital in- and outputs
 * @{
 */

/**
 * @brief Get the value of a digital input
 *
 * @param uChannel_p The channel of the digital input
 * @param pfState_p Pointer to the state destination
 * @return int32_t Driver result code of type tCtr700DrvResult
 */
int32_t Ctr700DrvGetDigiIn          (uint8_t uChannel_p, uint8_t* pfState_p);

/**
 * @brief Set the value of a digital output
 *
 * @param uChannel_p The channel of the digital output
 * @param fEnable_p The value to set
 * @return int32_t Driver result code of type tCtr700DrvResult
 */
int32_t Ctr700DrvSetDigiOut         (uint8_t uChannel_p, uint8_t fEnable_p);

/**
 * @brief Set the value of a relay output
 *
 * @param uChannel_p The channel of the relay
 * @param fEnable_p The value to set
 * @return int32_t Driver result code of type tCtr700DrvResult
 */
int32_t Ctr700DrvSetRelay           (uint8_t uChannel_p, uint8_t fEnable_p);

/** @} */

/**
 * @addtogroup counter Counter
 * @brief Access to the hardware counter for simple counting or A/B decoding a
 *        signal
 * @{
 */

/**
 * @brief Enable/disable a counter channel
 *
 * @param uChannel_p The channel to control
 * @param fEnable_p @see kCtr700Drv_True to enable,
 *                  @see kCtr700Drv_False to disable
 * @return int32_t Driver result code of type tCtr700DrvResult
 */
int32_t Ctr700DrvCntEnable          (uint8_t uChannel_p, uint8_t fEnable_p);

/**
 * @brief Setup the counters mode
 *
 * @param uChannel_p The channel to setup
 * @param uMode_p The mode of the counter, see #tCtr700DrvCounterMode
 * @param uTrigger_p The trigger of the counter, see #tCtr700DrvCounterTrigger
 * @param uDir_p The direction of counting, see #tCtr700DrvCounterDirection
 * @return int32_t Driver result code of type tCtr700DrvResult
 */
int32_t Ctr700DrvCntSetMode         (uint8_t uChannel_p, uint8_t uMode_p,
                                     uint8_t uTrigger_p, uint8_t uDir_p);

/**
 * @brief Set the initial value of the counter
 *
 * @param uChannel_p The channel to setup
 * @param iPreload_p The initial value to set
 * @return int32_t Driver result code of type tCtr700DrvResult
 */
int32_t Ctr700DrvCntSetPreload      (uint8_t uChannel_p, int32_t iPreload_p);

/**
 * @brief Get the value of a counter channel
 *
 * @param uChannel_p The channel to get the value for
 * @param piValue_p Pointer to the value destination
 * @return int32_t Driver result code of type tCtr700DrvResult
 */
int32_t Ctr700DrvCntGetValue        (uint8_t uChannel_p, int32_t* piValue_p);

/** @} */

/**
 * @addtogroup pwm_pto PWM/PTO
 * @brief Control and setup of the PWM channels
 * @{
 */

/**
 * @brief Set the time base for a counter channel
 *
 * @param uChannel_p The channel to setup
 * @param uTimeBase_p Time base of type #tCtr700DrvPwmTimebase
 * @return int32_t Driver result code of type tCtr700DrvResult
 */
int32_t Ctr700DrvPwmSetTimeBase     (uint8_t uChannel_p, uint8_t uTimeBase_p);

/**
 * @brief Setup counter channel configuration
 *
 * @param uChannel_p The channel to setup
 * @param uPeriod_p The period length in units set by #Ctr700DrvPwmSetTimeBase
 * @param uPulseLen_p The pulse length of the signal ('on' time / duty cycle)
 * @return int32_t Driver result code of type tCtr700DrvResult
 */
int32_t Ctr700DrvPwmSetParam        (uint8_t uChannel_p, uint16_t uPeriod_p,
                                     uint16_t uPulseLen_p);

/**
 * @brief Enable / disable a counter channel
 *
 * @param uChannel_p The channel to enable/disable
 * @param fRun_p @see kCtr700Drv_True to enable,
 *               @see kCtr700Drv_False to disable
 * @return int32_t Driver result code of type tCtr700DrvResult
 */
int32_t Ctr700DrvPwmEnable          (uint8_t uChannel_p, uint8_t fRun_p);

/**
 * @brief *Not implemented*
 *
 * @param uChannel_p -
 * @param uPeriod_p -
 * @param iDelta_p -
 * @param uPulseCnt_p -
 * @return int32_t Driver result code of type tCtr700DrvResult
 */
int32_t Ctr700DrvPtoSetParam        (uint8_t uChannel_p, uint16_t uPeriod_p,
                                     int16_t iDelta_p, uint32_t uPulseCnt_p);

/**
 * @brief *Not implemented*
 *
 * @param uChannel_p -
 * @param fRun_p -
 * @return int32_t Driver result code of type tCtr700DrvResult
 */
int32_t Ctr700DrvPtoEnable          (uint8_t uChannel_p, uint8_t fRun_p);

/**
 * @brief *Not implemented*
 *
 * @param uChannel_p -
 * @param pfRun_p -
 * @return int32_t Driver result code of type tCtr700DrvResult
 */
int32_t Ctr700DrvPtoGetState        (uint8_t uChannel_p, uint8_t* pfRun_p);

/** @} */

/**
 * @addtogroup adc ADC, Analog inputs channels
 * @brief Setup and get values of analog inputs channels
 * @{
 */

/**
 * @brief Get the value of an ADC channel
 *
 * @param uChannel_p The channel to get
 * @param puAdcValue_p Pointer to the value destination
 * @return int32_t Driver result code of type tCtr700DrvResult
 */
int32_t Ctr700DrvAdcGetValue        (uint8_t uChannel_p, uint16_t* puAdcValue_p);

/**
 * @brief Setup an ADC channel for voltage or current measurement
 * @note The ADC channel has a default configuration determined by the operating
 *       system configuration. See file /etc/systec/adc_modes.
 *
 * @param uChannel_p The channel to setup
 * @param uMode_p The mode of type #tCtr700DrvAnalogMode
 * @return int32_t Driver result code of type tCtr700DrvResult
 */
int32_t Ctr700DrvAdcSetMode         (uint8_t uChannel_p, uint8_t uMode_p);

/** @} */

/**
 * @addtogroup Temperature Sensor
 * @brief Get the value of the different temperature sensors
 * @{
 */

/**
 * @brief Get the value of a temperature sensor
 *
 * @param uSensor_p The temperature sensor channel
 * @param piValue_p Pointer to the value destination
 * @return int32_t Driver result code of type tCtr700DrvResult
 */
int32_t Ctr700DrvTmpGetValue        (uint8_t uSensor_p, int32_t* piValue_p);

/** @} */

/**
 * @addtogroup interrupt Interrupt handling
 * @brief Asynchronous interrupt handling for digital inputs
 * @{
 */

/**
 * @brief Register a callback to signal changes on an digital input
 * @note Channels 0..15 are used for digital inputs,
 *       Channel 128/0x80 is used for the RUN switch
 *
 * @param uChannel_p The channel of the digital input
 * @param pfnCallback_p The callback function to register of type
 *                      #tCtr700DrvInterruptCallback
 * @param uInterruptTrigger_p Set the kind of trigger for the input
 *                            #tCtr700DrvInterruptTrigger
 * @return int32_t Driver result code of type tCtr700DrvResult
 */
int32_t Ctr700DrvRegisterInterruptCallback (
        uint8_t                     uChannel_p,
        tCtr700DrvInterruptCallback pfnCallback_p,
        uint32_t                    uInterruptTrigger_p);

/**
 * @brief Un-register / disable interrupt handling for a digital input
 *
 * @param uChannel_p Analogous to #Ctr700DrvRegisterInterruptCallback
 * @return int32_t Driver result code of type tCtr700DrvResult
 */
int32_t Ctr700DrvUnregisterInterruptCallback (
        uint8_t                     uChannel_p);

/** @} */

#ifdef __cplusplus
    }
#endif

/** @} */

#endif  // #ifndef _CTR700DRV_H_

