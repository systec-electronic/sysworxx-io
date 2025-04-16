// SPDX-License-Identifier: LGPL-3.0-or-later
// SPDX-FileCopyrightText: 2025 SYS TEC electronic AG <https://www.systec-electronic.com/>

#include <stdio.h>
#include <stdint.h>
#include <stdlib.h>
#include <unistd.h>
#include <signal.h>

#include "ctr700drv.h"

/***************************************************************************/
/*                                                                         */
/*                                                                         */
/*          G L O B A L   D E F I N I T I O N S                            */
/*                                                                         */
/*                                                                         */
/***************************************************************************/

//---------------------------------------------------------------------------
//  Configuration
//---------------------------------------------------------------------------

#define APP_VER_MAJOR 2
#define APP_VER_MINOR 0

#define RUNLIGHT_START_VALUE 7

//---------------------------------------------------------------------------
//  Constant definitions
//---------------------------------------------------------------------------

#define eprintf(args...) fprintf(stderr, ##args)

//---------------------------------------------------------------------------
//  Local types
//---------------------------------------------------------------------------

typedef enum
{
    kRunlightMode_Left,
    kRunlightMode_Right,
    kRunlightMode_Stop,
} tRunlightMode;

//---------------------------------------------------------------------------
//  Global variables
//---------------------------------------------------------------------------

//---------------------------------------------------------------------------
//  Local variables
//---------------------------------------------------------------------------

static tCtr700DrvHwInfo Ctr700DrvHwInfo_l;
static volatile tRunlightMode Mode_l = kRunlightMode_Right;
static volatile uint8_t fRun_l = kCtr700Drv_True;

//---------------------------------------------------------------------------
//  Prototypes of internal functions
//---------------------------------------------------------------------------

static int Init(void);
static int EnterMainLoop(void);
static void Exit(void);
static void SigHandler(int nSignalNum_p);
static void CbDigitalInput(uint8_t uChannel_p, uint8_t fEnable_p);

//=========================================================================//
//                                                                         //
//          P U B L I C   F U N C T I O N S                                //
//                                                                         //
//=========================================================================//

//---------------------------------------------------------------------------
//  Main function of demo application
//---------------------------------------------------------------------------

int main(int argc, char const *argv[])
{
    int iResult;

    (void) argc;
    (void) argv;

    printf("\n");
    printf("********************************************************************\n");
    printf("  Test application for SYSTEC sysWORXX CTR-700 board driver\n");
    printf("  Version: %u.%02u\n", APP_VER_MAJOR, APP_VER_MINOR);
    printf("  (c) 2019 SYS TEC electronic AG, www.systec-electronic.com\n");
    printf("********************************************************************\n");
    printf("\n");

    iResult = Init();
    if (iResult != EXIT_SUCCESS)
    {
        eprintf("Failed to initialize application!\n");
        return EXIT_FAILURE;
    }

    iResult = EnterMainLoop();
    if (iResult != EXIT_SUCCESS)
    {
        eprintf("Error in main loop!\n");
        return EXIT_FAILURE;
    }

    // cleanup and stop application
    Exit();

    return iResult;
}

static int Init(void)
{
    tCtr700DrvResult Result;
    uint8_t uDrvVersionMajor;
    uint8_t uDrvVersionMinor;

    Result = Ctr700DrvInitialize();
    if (Result != kCtr700DrvResult_Success)
    {
        eprintf("ERROR: Failed to initialize CTR-700 driver!\n");
        return EXIT_FAILURE;
    }

    Result = Ctr700DrvGetVersion(&uDrvVersionMajor, &uDrvVersionMinor);
    if (Result != kCtr700DrvResult_Success)
    {
        eprintf("ERROR: Failed to get the CTR-700 driver version!\n");
        return EXIT_FAILURE;
    }

    Result = Ctr700DrvGetHardwareInfo(&Ctr700DrvHwInfo_l);
    if (Result != kCtr700DrvResult_Success)
    {
        eprintf("ERROR: Failed to get CTR-700 hardware information!\n");
        return EXIT_FAILURE;
    }

    printf("********************************************************************\n");
    printf("  I/O Driver version: %u.%02u\n", uDrvVersionMajor, uDrvVersionMinor);
    printf("  PCB Revision:       %u\n", Ctr700DrvHwInfo_l.m_uPcbRevision);
    printf("  IO configuration:\n");
    printf("    Digital In:  %u\n", Ctr700DrvHwInfo_l.m_uDiChannels);
    printf("    Digital Out: %u\n", Ctr700DrvHwInfo_l.m_uDoChannels);
    printf("    Relay:       %u\n", Ctr700DrvHwInfo_l.m_uRelayChannels);
    printf("    Analog In:   %u\n", Ctr700DrvHwInfo_l.m_uAiChannels);
    printf("    Analog Out:  %u\n", Ctr700DrvHwInfo_l.m_uAoChannels);
    printf("    Counter:     %u\n", Ctr700DrvHwInfo_l.m_uCntChannels);
    printf("    A/B Encoder: %u\n", Ctr700DrvHwInfo_l.m_uEncChannels);
    printf("    PWM/PTO:     %u\n", Ctr700DrvHwInfo_l.m_uPwmChannels);
    printf("    TempSensor:  %u\n", Ctr700DrvHwInfo_l.m_uTmpChannels);
    printf("********************************************************************\n");
    printf("\n");

    for (uint8_t i = 0; i < 3; i++)
    {
        printf("Register DI interrupt for channel %u\n", i);
        Result = Ctr700DrvRegisterInterruptCallback (i, CbDigitalInput,
                kCtr700DrvInterruptRisingEdge);
        if (Result != kCtr700DrvResult_Success)
        {
            eprintf("ERROR: Ctr700DrvRegisterInterruptCallback() returned error code 0x%02X\n", Result);
            return EXIT_FAILURE;
        }
    }

    signal(SIGINT, SigHandler);

    return EXIT_SUCCESS;
}

//---------------------------------------------------------------------------
//  Execute the runlight loop
//---------------------------------------------------------------------------

static int EnterMainLoop(void)
{
    uint8_t fDoState;
    uint8_t fRunSwitch;
    uint8_t uChannel;
    uint8_t fRunLedState = kCtr700Drv_True;
    uint16_t uAdcValue;
    uint16_t uDigiOut = RUNLIGHT_START_VALUE;
    int32_t iResult = EXIT_SUCCESS;
    tCtr700DrvResult Result;

    while (fRun_l)
    {
        // check state of RUN/STOP switch
        Result = Ctr700DrvGetRunSwitch(&fRunSwitch);
        if (Result != kCtr700DrvResult_Success)
        {
            eprintf("ERROR: Ctr700DrvGetRunSwitch() returned error code 0x%02X\n", Result);
            iResult = EXIT_FAILURE;
            goto EXIT;
        }

        if (fRunSwitch != kCtr700Drv_True)
        {
            printf("Run switch is set to stop: Exit main loop\n");
            break;
        }

        if (Mode_l != kRunlightMode_Stop)
        {
            // toggle green RUN LED
            Result = Ctr700DrvSetRunLed(fRunLedState);
            if (Result != kCtr700DrvResult_Success)
            {
                eprintf("ERROR: Ctr700DrvSetRunLed() returned error code 0x%02X\n", Result);
                iResult = EXIT_FAILURE;
                goto EXIT;
            }
            fRunLedState = !fRunLedState;

            Result = Ctr700DrvSetErrLed(kCtr700Drv_False);
            if (Result != kCtr700DrvResult_Success)
            {
                eprintf("ERROR: Ctr700DrvSetErrLed() returned error code 0x%02X\n", Result);
                iResult = EXIT_FAILURE;
                goto EXIT;
            }
        } else {
            Result = Ctr700DrvSetRunLed(kCtr700Drv_False);
            if (Result != kCtr700DrvResult_Success)
            {
                eprintf("ERROR: Ctr700DrvSetRunLed() returned error code 0x%02X\n", Result);
                iResult = EXIT_FAILURE;
                goto EXIT;
            }

            Result = Ctr700DrvSetErrLed(kCtr700Drv_True);
            if (Result != kCtr700DrvResult_Success)
            {
                eprintf("ERROR: Ctr700DrvSetErrLed() returned error code 0x%02X\n", Result);
                iResult = EXIT_FAILURE;
                goto EXIT;
            }
        }

        // process runlight
        switch (Mode_l)
        {
            case kRunlightMode_Left:
            {
                // shift right
                uDigiOut = uDigiOut >> 1 | uDigiOut << (16 - 1);
                break;
            }
            case kRunlightMode_Right:
            {
                // shift left
                uDigiOut = uDigiOut << 1 | uDigiOut >> (16 - 1);
                break;
            }
            case kRunlightMode_Stop:
            default:
            {
                // do nothing, the runlight is stopped
            }
        }

        // output runligth (set digital outputs)
        for (uChannel = 0; uChannel < Ctr700DrvHwInfo_l.m_uDiChannels; uChannel++)
        {
            fDoState = (uDigiOut & (1 << uChannel)) ? 1 : 0;

            Result = Ctr700DrvSetDigiOut(uChannel, fDoState);
            if (Result != kCtr700DrvResult_Success)
            {
                eprintf("ERROR: Ctr700DrvSetDigiOut() returned error code 0x%02X\n", Result);
                iResult = EXIT_FAILURE;
                goto EXIT;
            }
        }

        // get ADC0 value
        Result = Ctr700DrvAdcGetValue(kCtr700DrvAnalogIn_Channel0, &uAdcValue);
        if (Result != kCtr700DrvResult_Success)
        {
            eprintf("ERROR: Ctr700DrvAdcGetValue() returned error code 0x%02X\n", Result);
            iResult = EXIT_FAILURE;
            goto EXIT;
        }

        // delay based on ADC0 value
        const int iAdcMin = 0;            // 0 V
        const int iAdcMax = 28151;        // 10 V
        const int iDelayMin = 500 * 1000; // 500 ms
        const int iDelayMax = 25 * 1000;  // 25 ms

        // y = mx + n
        int m = (iDelayMax - iDelayMin) / (iAdcMax - iAdcMin);
        int n = iDelayMin;
        int x = ((int) uAdcValue) - iAdcMin;

        int iDelay = (m * x) + n;

        usleep(iDelay);
    }

EXIT:
    return iResult;
}

//---------------------------------------------------------------------------
//  Reset all outputs
//---------------------------------------------------------------------------
static void Exit(void)
{
    int8_t uChannel;

    // reset RUN LED and ERR LED
    Ctr700DrvSetRunLed(0);
    Ctr700DrvSetErrLed(0);

    // reset DOs
    for (uChannel = 0; uChannel < Ctr700DrvHwInfo_l.m_uDiChannels; uChannel++)
    {
        Ctr700DrvSetDigiOut(uChannel, 0);
    }

    Ctr700DrvShutDown();
}

//---------------------------------------------------------------------------
//  Callback function, called if RUN switch turned to STOP
//---------------------------------------------------------------------------
static void CbDigitalInput(uint8_t uChannel_p, uint8_t fEnable_p)
{
    (void) fEnable_p; // callback will only be called on rising edge

    switch (uChannel_p)
    {
        case 0:
        {
            printf("Set mode: LEFT\n");
            Mode_l = kRunlightMode_Left;
            break;
        }
        case 1:
        {
            printf("Set mode: STOP\n");
            Mode_l = kRunlightMode_Stop;
            break;
        }
        case 2:
        {
            printf("Set mode: RIGHT\n");
            Mode_l = kRunlightMode_Right;
            break;
        }
        default:
        {
            eprintf("Unexpected interrupt event, stop application");
            fRun_l = 0;
        }
    }
}

//---------------------------------------------------------------------------
//  Application signal handler
//---------------------------------------------------------------------------
static void SigHandler(int nSignalNum_p)
{
    (void) nSignalNum_p;

    printf("Got SIGINT, stop application\n\n");
    fRun_l = 0;
}

