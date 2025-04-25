#!/usr/bin/env python
# -*- coding: utf-8 -*-

# SPDX-License-Identifier: LGPL-3.0-or-later
# SPDX-FileCopyrightText: 2025 SYS TEC electronic AG <https://www.systec-electronic.com/>

"""
Project:      SYSTEC sysWORXX CTR-700
Description:  Python driver demo application for counter input
"""

import signal
import time

from ctr700drv import CounterDirection, CounterMode, CounterTrigger, Ctr700Drv

# version information
APP_VER_MAIN = 1  # version 1.xx
APP_VER_REL = 0  # version x.00


class App(object):

    def __init__(self):
        self.ctr700 = Ctr700Drv()
        self.running = True
        signal.signal(signal.SIGINT, self.on_sigint)

    def __enter__(self):
        self.ctr700.init()
        self.ctr700.counter_set_mode(
            0, CounterMode.COUNTER, CounterTrigger.RISING_EDGE, CounterDirection.UP
        )
        self.ctr700.counter_enable(0)
        return self

    def __exit__(self, type, value, traceback):
        self.ctr700.counter_disable(0)
        self.ctr700.shutdown()

    def on_sigint(self, signal, frame):
        print("Got SIGINT, stop application")
        self.running = False

    def run(self):
        while self.running:
            print("Counter value: {}".format(self.ctr700.counter_get_value(0)))
            time.sleep(1.0)


def main():
    print("")
    print("*" * 79)
    print("  Test application for SYSTEC sysWORXX CTR-700 counter")
    print("  Version: {}.{}".format(APP_VER_MAIN, APP_VER_REL))
    print("  (c) 2019 SYS TEC electronic AG, www.systec-electronic.com")
    print("*" * 79)
    print("")

    with App() as app:
        app.run()


if __name__ == "__main__":
    main()
