// SPDX-License-Identifier: LGPL-3.0-or-later
// SPDX-FileCopyrightText: 2025 SYS TEC electronic AG <https://www.systec-electronic.com/>

/****************************************************************************

  Project:      SYSTEC sysWORXX CTR-700
  Description:  Java driver demo application

****************************************************************************/

package demo;

import com.systec.Ctr700Drv;
import com.systec.Ctr700Drv.DiCallback;
import com.systec.Ctr700Drv.HardwareInfo;
import com.systec.Ctr700Drv.InterruptTrigger;
import com.systec.Ctr700Drv.Version;

public class Demo {
    final private static byte APP_VER_MAIN = 2;
    final private static byte APP_VER_REL = 0;

    private Ctr700Drv ctr700;
    private HardwareInfo info;
    private Mode mode;

    enum Mode {
        LEFT, STOP, RIGHT;

        private static Mode[] allValues = values();

        public static Mode fromOrdinal(int n) {
            return allValues[n];
        }
    }

    public static void main(String[] args) {
        System.out.println("");
        System.out.println("********************************************************************");
        System.out.println("  Test application for SYSTEC sysWORXX CTR-700 board driver");
        System.out.println("  Version: " + APP_VER_MAIN + "." + APP_VER_REL);
        System.out.println("  (c) 2019 SYS TEC electronic AG, www.systec-electronic.com");
        System.out.println("********************************************************************");
        System.out.println("");

        Demo app = new Demo();
        app.init();
        app.runLoop();
    }

    Demo() {
        ctr700 = Ctr700Drv.getInstance();
        info = null;
        mode = Mode.RIGHT;
    }

    private void init() {
        ctr700.init();

        Version version = ctr700.getVersion();
        info = ctr700.getHardwareInfo();

        System.out.format("********************************************************************\n");
        System.out.format("I/O Driver version: %d.%d\n", version.major, version.minor);
        System.out.format("  PCB Revision:    %d\n", info.PcbRevision);
        System.out.format("  IO configuration:\n");
        System.out.format("    Digital In:  %d\n", info.DiChannels);
        System.out.format("    Digital Out: %d\n", info.DoChannels);
        System.out.format("    Relay:       %d\n", info.RelayChannels);
        System.out.format("    Analog In:   %d\n", info.AiChannels);
        System.out.format("    Analog Out:  %d\n", info.AoChannels);
        System.out.format("    Counter:     %d\n", info.CntChannels);
        System.out.format("    A/B Encoder: %d\n", info.EncChannels);
        System.out.format("    PWM/PTO:     %d\n", info.PwmChannels);
        System.out.format("    TempSensor:  %d\n", info.TmpChannels);
        System.out.format("********************************************************************\n");
        System.out.format("\n");

        // handle SIGINT
        Runtime.getRuntime().addShutdownHook(new Thread(new Runnable() {
            public void run() {
                System.out.println("Stop application");
                exit();
            }
        }));

        for (int channel = 0; channel < 3; channel++) {

            DiCallback cb = new DiCallback() {
                public void invoke(byte channel, byte value) {
                    System.out.format("Set mode: %s\n", Mode.fromOrdinal(channel));
                    mode = Mode.fromOrdinal((int) channel);
                }
            };

            System.out.format("Register DI interrupt for channel %d\n", channel);
            ctr700.registerInterrupt(channel, cb, InterruptTrigger.RISING_EDGE);
        }
    }

    private void runLoop() {
        boolean runLed = true;
        int outputMask = 0x07;

        while (true) {
            if (!ctr700.getRunSwitch()) {
                System.out.println("Run switch is set to stop: Exit main loop");
                break;
            }

            if (this.mode != Mode.STOP) {
                this.ctr700.setRunLed(runLed);
                runLed = !runLed;
                this.ctr700.setErrLed(false);
            } else {
                this.ctr700.setRunLed(false);
                this.ctr700.setErrLed(true);
            }

            switch (mode) {
            case LEFT:
                outputMask = (outputMask >> 1 | outputMask << (16 - 1)) & 0xffff;
                break;
            case RIGHT:
                outputMask = (outputMask << 1 | outputMask >> (16 - 1)) & 0xffff;
                break;
            default:
                break;
            }

            for (int channel = 0; channel < info.DoChannels; channel++) {
                boolean state = ((outputMask & (1 << channel)) != 0) ? true : false;
                ctr700.setDigiOut(channel, state);
            }

            short adcValue = ctr700.adcGetValue(0);
            long delay = calcDelay(adcValue);

            try {
                Thread.sleep(delay);
            } catch (InterruptedException e) {
                e.printStackTrace();
            }
        }
    }

    private void exit() {
        ctr700.setRunLed(false);
        ctr700.setErrLed(false);

        for (int channel = 0; channel < info.DoChannels; channel++) {
            ctr700.setDigiOut(channel, false);
        }

        ctr700.shutDown();
    }

    private long calcDelay(short adcValue) {
        final float adcMin = 0; // 0 V
        final float adcMax = 28151; // 10 V
        final float delayMin = 500; // 500 ms
        final float delayMax = 25; // 25 ms

        // y = mx + n
        final float m = (delayMax - delayMin) / (adcMax - adcMin);
        final float n = delayMin;
        float x = adcValue - adcMin;

        return (long) ((m * x) + n);
    }
}
