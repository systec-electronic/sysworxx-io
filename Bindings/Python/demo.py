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

import signal
import time

from ctr700drv import Ctr700Drv

# version information
APP_VER_MAIN = 2    # version 1.xx
APP_VER_REL = 0     # version x.00


class Modes(object):
    LEFT = "LEFT"
    STOP = "STOP"
    RIGHT = "RIGHT"

    @staticmethod
    def get(index):
        return (Modes.LEFT, Modes.STOP, Modes.RIGHT)[index]


class Runlight(object):

    def __init__(self):
        self.mode = Modes.RIGHT
        self.running = True
        self.ctr700 = Ctr700Drv()
        self.hwinfo = None

    def __enter__(self):
        self.ctr700.init()

        major, minor = self.ctr700.get_version()
        hwinfo = self.ctr700.get_hardware_info()
        self.hwinfo = hwinfo

        print("*" * 79)
        print("I/O Driver version: {}.{}".format(major, minor))
        print("  PCB Revision:     {}".format(hwinfo['PcbRevision']))
        print("  IO configuration:")
        print("    Digital In:  {}".format(hwinfo['DiChannels']))
        print("    Digital Out: {}".format(hwinfo['DoChannels']))
        print("    Relay:       {}".format(hwinfo['RelayChannels']))
        print("    Analog In:   {}".format(hwinfo['AiChannels']))
        print("    Analog Out:  {}".format(hwinfo['AoChannels']))
        print("    Counter:     {}".format(hwinfo['CntChannels']))
        print("    A/B Encoder: {}".format(hwinfo['EncChannels']))
        print("    PWM/PTO:     {}".format(hwinfo['PwmChannels']))
        print("    TempSensor:  {}".format(hwinfo['TmpChannels']))
        print("*" * 79)
        print("")

        for channel in range(3):
            print("Register DI interrupt for channel {}".format(channel))
            self.ctr700.register_interrupt(channel, True, False,
                                           self.on_digital_input)

        signal.signal(signal.SIGINT, self.on_sigint)

        return self

    def __exit__(self, type, value, traceback): #pylint: disable=redefined-builtin
        self.ctr700.set_run_led(False)
        self.ctr700.set_err_led(False)

        for channel in range(self.hwinfo['DoChannels']):
            self.ctr700.set_digi_out(channel, False)

        self.ctr700.shutdown()

    def on_sigint(self, _signal, _frame):
        print("Got SIGINT, stop application")
        self.running = False

    def on_digital_input(self, channel, _value):
        self.mode = Modes.get(channel)
        print("Set mode: {}".format(self.mode))

    def run(self):
        output_mask = 0x07
        run_led = True

        while self.running:
            if not self.ctr700.get_run_switch():
                print("Run switch is set to stop: Exit main loop")
                break

            if self.mode != Modes.STOP:
                self.ctr700.set_run_led(run_led)
                run_led = not run_led
                self.ctr700.set_err_led(False)
            else:
                self.ctr700.set_run_led(False)
                self.ctr700.set_err_led(True)

            if self.mode == Modes.LEFT:
                output_mask = (output_mask >> 1 | output_mask << (16 - 1)) \
                                & 0xffff
            elif self.mode == Modes.RIGHT:
                output_mask = (output_mask << 1 | output_mask >> (16 - 1)) \
                                & 0xffff
            else:
                pass

            for channel in range(self.hwinfo['DoChannels']):
                self.ctr700.set_digi_out(channel, output_mask & (1 << channel))

            def calc_delay(value):
                adc_min = 0        # 0 V
                adc_max = 28151    # 10 V
                delay_min = 0.500  # 500 ms
                delay_max = 0.025  # 25 ms

                # y = mx + n
                #pylint: disable=invalid-name
                m = (delay_max - delay_min) / (adc_max - adc_min)
                n = delay_min
                x = value - adc_min
                #pylint: enable=invalid-name

                return (m * x) + n

            adc_value = self.ctr700.adc_get_value(0)
            delay = calc_delay(adc_value)
            time.sleep(delay)


def main():
    print("")
    print("*" * 79)
    print("  Test application for SYSTEC sysWORXX CTR-700 board driver")
    print("  Version: {}.{}".format(APP_VER_MAIN, APP_VER_REL))
    print("  (c) 2019 SYS TEC electronic AG, www.systec-electronic.com")
    print("*" * 79)
    print("")

    with Runlight() as app:
        app.run()


if __name__ == "__main__":
    main()
