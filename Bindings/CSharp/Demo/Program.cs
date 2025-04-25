// SPDX-License-Identifier: LGPL-3.0-or-later
// SPDX-FileCopyrightText: 2025 SYS TEC electronic AG <https://www.systec-electronic.com/>

// ****************************************************************************
//
//   Project:      SYSTEC sysWORXX CTR-700
//   Description:  C# demo application for the I/O driver
//
// ****************************************************************************

using System.Globalization;
using System.Text;
using Sysworxx;

namespace Program
{
    public static class Program
    {
        /// <summary>
        /// Application entry point.
        /// </summary>
        /// <param name="args">The command line arguments.</param>
        /// <returns>The process exit code.</returns>
        public static int Main(string[] args)
        {
            Console.WriteLine(".NET core demo application for SYSTEC sysWORXX devices");
            Console.WriteLine();

            try
            {
                MenuLoop();
                Console.WriteLine();
                return 0;
            }
            catch (IoException ex)
            {
                Console.WriteLine();
                Console.Error.WriteLine("I/O driver exception: " + ex.Message);
                Console.Error.WriteLine(ex.StackTrace);
                return 1;
            }
            catch (Exception ex)
            {
                Console.WriteLine();
                Console.Error.WriteLine(ex.GetType().Name + ": " + ex.Message);
                Console.Error.WriteLine(ex.StackTrace);
                return 1;
            }
        }

        /// <summary>
        /// Runs the application main menu loop.
        /// </summary>
        private static void MenuLoop()
        {
            var commands = new List<(ConsoleKey Key, string Label, Action<Io> Action)>
            {
                (ConsoleKey.A, "Analog input", RunAnalogInput),
                (ConsoleKey.C, "Counter", RunCounter),
                (ConsoleKey.D, "Digital I/O", RunDigitalIO),
                (ConsoleKey.H, "PWM flashing", RunPwmFlashing),
                (ConsoleKey.I, "Tick count", RunTickCount),
                (ConsoleKey.L, "LEDs", RunLeds),
                (ConsoleKey.P, "PWM fading", RunPwmFading),
                (ConsoleKey.S, "Run switch", RunRunSwitch),
                (ConsoleKey.T, "Temperature", RunTemperature),
                // TODO: (ConsoleKey.W, "Watchdog", RunWatchdog),
            };

            using (var driver = new Io())
            {
                ShowInfo(driver);

                bool exit = false;
                do
                {
                    Console.WriteLine();
                    ShowMainMenu(commands);

                    while (true)
                    {
                        var key = Console.ReadKey(true).Key;
                        var commandData = commands.FirstOrDefault(x => x.Key == key);

                        if (commandData.Key == key)
                        {
                            Console.WriteLine(commandData.Label);
                            commandData.Action(driver);
                            break;
                        }
                        else if (key == ConsoleKey.Escape)
                        {
                            exit = true;
                            break;
                        }
                        else
                        {
                            Console.WriteLine("Invalid input!");
                        }
                    }
                }
                while (!exit);
            }
        }

        /// <summary>
        /// Shows general I/O driver information.
        /// </summary>
        /// <param name="driver">The driver.</param>
        private static void ShowInfo(Io driver)
        {
            Console.WriteLine("Driver version:  " + driver.Version);
            Console.WriteLine("PCB revision:    " + driver.PcbRevision);
            Console.WriteLine("IO configuration:");
            Console.WriteLine("    Digital In:  " + driver.DigitalInputChannels);
            Console.WriteLine("    Digital Out: " + driver.DigitalOutputChannels);
            Console.WriteLine("    Analog In:   " + driver.AnalogInputChannels);
            Console.WriteLine("    Analog Out:  " + driver.AnalogOutputChannels);
            Console.WriteLine("    Counter:     " + driver.CounterChannels);
            Console.WriteLine("    AB Encoder:  " + driver.EncoderChannels);
            Console.WriteLine("    PWM:         " + driver.PwmChannels);
            Console.WriteLine("    TempSensor:  " + driver.TemperatureSensors);
        }

        /// <summary>
        /// Shows the main menu.
        /// </summary>
        /// <param name="commands">The available demo commands.</param>
        private static void ShowMainMenu(List<(ConsoleKey Key, string Label, Action<Io> Action)> commands)
        {
            Console.WriteLine("Commands:");

            foreach (var command in commands)
            {
                Console.WriteLine($" {command.Key} - {command.Label}");
            }

            Console.WriteLine(" Press ESC (Escape) to Exit");
        }

        /// <summary>
        /// Prompts the user for a single key input.
        /// </summary>
        /// <param name="message">The message to display. Can be empty.</param>
        /// <param name="keys">The acceptable keys.</param>
        /// <returns>The pressed key.</returns>
        private static ConsoleKey Prompt(string message, params ConsoleKey[] keys)
        {
            Console.Write(message);
            while (true)
            {
                var key = Console.ReadKey(true).Key;
                if (keys.Contains(key))
                {
                    if (message != "")
                        Console.WriteLine();
                    return key;
                }
            }
        }

        private static void RunAnalogInput(Io driver)
        {
            Console.WriteLine();
            if (driver.AnalogInputChannels == 0)
            {
                Console.WriteLine("Analog input channels are not available on this device.");
                return;
            }
            string line;
            byte channel;
            AnalogMode mode = AnalogMode.Voltage;
            string modeText = "";
            do
            {
                Console.Write($"Enter channel number to read (0-{driver.AnalogInputChannels - 1}): ");
                line = Console.ReadLine()!;
                if (line.Trim() == "") return;
            }
            while (!byte.TryParse(line, out channel) || channel >= driver.AnalogInputChannels);
            switch (Prompt("Channel mode: (V)oltage or (C)urrent? ", ConsoleKey.V, ConsoleKey.C, ConsoleKey.Escape))
            {
                case ConsoleKey.V:
                    mode = AnalogMode.Voltage;
                    modeText = "voltage";
                    break;
                case ConsoleKey.C:
                    mode = AnalogMode.Current;
                    modeText = "current";
                    break;
                case ConsoleKey.Escape:
                    return;
            }

            Console.WriteLine();
            Console.WriteLine($"Reading {modeText} from analog input channel {channel}.");
            Console.WriteLine("Press Esc to return");

            driver.SetAnalogMode(channel, mode);

            bool exit = false;
            do
            {
                // Show current analog input value
                ushort value = driver.GetAnalogInput(channel);
                Console.WriteLine(value + " digits");

                // Wait 0.5 seconds or until Esc key is pressed
                for (int timer = 0; timer < 5 && !exit; timer++)
                {
                    Thread.Sleep(100);
                    if (Console.KeyAvailable && Console.ReadKey(true).Key == ConsoleKey.Escape)
                    {
                        exit = true;
                    }
                }
            }
            while (!exit);
        }

        private static void RunCounter(Io driver)
        {
            Console.WriteLine();
            if (driver.CounterChannels == 0)
            {
                Console.WriteLine("Counters are not available on this device.");
                return;
            }
            string line;
            byte channel;
            CounterTrigger trigger = 0;
            do
            {
                Console.Write($"Enter channel number to read (0-{driver.CounterChannels - 1}): ");
                line = Console.ReadLine()!;
                if (line.Trim() == "") return;
            }
            while (!byte.TryParse(line, out channel) || channel >= driver.CounterChannels);
            switch (Prompt("Counter trigger: (R)ising edge, (F)alling edge or (B)oth edges? ", ConsoleKey.R, ConsoleKey.F, ConsoleKey.B, ConsoleKey.Escape))
            {
                case ConsoleKey.R:
                    trigger = CounterTrigger.RisingEdge;
                    break;
                case ConsoleKey.F:
                    trigger = CounterTrigger.FallingEdge;
                    break;
                case ConsoleKey.B:
                    trigger = CounterTrigger.AnyEdge;
                    break;
                case ConsoleKey.Escape:
                    return;
            }

            Console.WriteLine();
            Console.WriteLine($"Counting on channel {channel} upwards, starting at 0.");
            Console.WriteLine("Press Esc to return");

            driver.EnableCounter(channel, true);
            driver.SetCounterMode(channel, CounterMode.Counter, trigger, CounterDirection.Up);

            bool exit = false;
            do
            {
                int value = driver.GetCounterValue(channel);
                Console.WriteLine(value);

                // Wait 5 seconds or until Esc key is pressed
                for (int timer = 0; timer < 50 && !exit; timer++)
                {
                    Thread.Sleep(100);
                    if (Console.KeyAvailable && Console.ReadKey(true).Key == ConsoleKey.Escape)
                    {
                        exit = true;
                    }
                }
            }
            while (!exit);

            // Disable counter again
            driver.EnableCounter(channel, false);
        }

        private static void RunDigitalIO(Io driver)
        {
            Console.WriteLine();
            Console.WriteLine("Reading digital input channels and cycling digital output channels.");
            Console.WriteLine("Press Esc to return");

            // Enable events for all digital input channels on both edges
            for (byte channel = 0; channel < driver.DigitalInputChannels; channel++)
            {
                try
                {
                    driver.SetDigitalInputEvents(channel, InputTrigger.BothEdge);
                    Console.WriteLine($"Registered Input {channel}");
                }
                catch (IoException)
                {
                    // The list of digital inputs is not continuous and not all
                    // inputs support input handler registration.
                }
            }
            driver.DigitalInputChanged += Driver_DigitalInputChanged;

            bool first = true;
            var sb = new StringBuilder();
            byte output = 0;
            bool exit = false;
            do
            {
                // Show all digital input states
                sb.Clear();
                sb.Append("Input: ");
                for (byte channel = 0; channel < driver.DigitalInputChannels; channel++)
                {
                    char c = '_';
                    try
                    {
                        bool state = driver.GetDigitalInput(channel);
                        c = state ? 'x' : 'o';
                    }
                    catch (IoException)
                    {
                        // The list of digital inputs is not continuous and not all
                        // inputs support input handler registration.
                    }

                    sb.Append(c);
                }

                // Activate the next digital output (cycling)
                if (driver.DigitalOutputChannels > 0)
                {
                    if (first)
                    {
                        first = false;
                    }
                    else
                    {
                        driver.SetDigitalOutput(output, false);
                        output = (byte)((output + 1) % 4);
                    }
                    driver.SetDigitalOutput(output, true);
                    sb.Append("    Active output: ");
                    sb.Append(output);
                }

                Console.WriteLine(sb.ToString());

                // Wait 2 seconds or until Esc key is pressed
                for (int timer = 0; timer < 20 && !exit; timer++)
                {
                    Thread.Sleep(100);
                    if (Console.KeyAvailable && Console.ReadKey(true).Key == ConsoleKey.Escape)
                    {
                        exit = true;
                    }
                }
            }
            while (!exit);

            // Turn off currently active output again
            if (driver.DigitalOutputChannels > 0)
            {
                driver.SetDigitalOutput(output, false);
            }

            // Disable events for digital input channels
            for (byte channel = 0; channel < driver.DigitalInputChannels; channel++)
            {
                try
                {
                    driver.SetDigitalInputEvents(channel, InputTrigger.None);
                    Console.WriteLine($"Registered Input {channel}");
                }
                catch (IoException)
                {
                    // The list of digital inputs is not continuous and not all
                    // inputs support input handler registration.
                }

            }
            driver.DigitalInputChanged -= Driver_DigitalInputChanged;
        }

        private static void Driver_DigitalInputChanged(object sender, DigitalInputChangedEventArgs args)
        {
            Console.WriteLine($"Digital input {args.Channel} is now {(args.State ? "on" : "off")}");
        }

        private static void RunLeds(Io driver)
        {
            Console.WriteLine();
            Console.WriteLine("Cycling run and error LEDs.");
            Console.WriteLine("Press Esc to return");

            int phase = -1;
            bool exit = false;
            do
            {
                // De/Activate the next LED
                phase = (phase + 1) % 4;
                switch (phase)
                {
                    case 0:
                        driver.RunLed = true;
                        Console.WriteLine("Run LED:   on");
                        break;
                    case 1:
                        driver.ErrorLed = true;
                        Console.WriteLine("Error LED: on");
                        break;
                    case 2:
                        driver.RunLed = false;
                        Console.WriteLine("Run LED:   off");
                        break;
                    case 3:
                        driver.ErrorLed = false;
                        Console.WriteLine("Error LED: off");
                        break;
                }

                // Wait 1 second or until Esc key is pressed
                for (int timer = 0; timer < 10 && !exit; timer++)
                {
                    Thread.Sleep(100);
                    if (Console.KeyAvailable && Console.ReadKey(true).Key == ConsoleKey.Escape)
                    {
                        exit = true;
                    }
                }
            }
            while (!exit);

            // Turn off all LEDs again
            driver.RunLed = false;
            driver.ErrorLed = false;
        }

        private static void RunPwmFading(Io driver)
        {
            Console.WriteLine();
            if (driver.PwmChannels == 0)
            {
                Console.WriteLine("PWM output channels are not available on this device.");
                return;
            }
            string line;
            byte channel;
            PwmTimebase timeBase = PwmTimebase.Ns800;
            ushort period = 20;
            ushort step = 1;
            int interval = 200;
            do
            {
                Console.Write($"Enter channel number to use for PWM (0-{driver.PwmChannels - 1}): ");
                line = Console.ReadLine()!;
                if (line.Trim() == "") return;
            }
            while (!byte.TryParse(line, out channel) || channel >= driver.PwmChannels);
            switch (Prompt("Time base: (A) 800 ns or (B) 1 ms? ", ConsoleKey.A, ConsoleKey.B, ConsoleKey.Escape))
            {
                case ConsoleKey.A:
                    timeBase = PwmTimebase.Ns800;
                    period = 200;
                    step = 1;
                    interval = 20;
                    break;
                case ConsoleKey.B:
                    timeBase = PwmTimebase.Ms1;
                    period = 20;
                    step = 1;
                    interval = 200;
                    break;
                case ConsoleKey.Escape:
                    return;
            }

            Console.WriteLine();
            Console.WriteLine($"Fading PWM output channel {channel}.");
            Console.WriteLine("Press Esc to return");

            driver.SetPwmTimeBase(channel, timeBase);

            bool inc = true;
            ushort pulseLen = 0;
            bool exit = false;
            Console.WriteLine("Fading up...");
            do
            {
                // Fade up/down the duty cycle
                if (inc)
                {
                    if (pulseLen < period - step)
                    {
                        pulseLen += step;
                    }
                    else if (pulseLen < period)
                    {
                        pulseLen = period;
                        inc = false;
                        Console.WriteLine("Fading down...");
                    }
                }
                else
                {
                    if (pulseLen > step)
                    {
                        pulseLen -= step;
                    }
                    else if (pulseLen > 0)
                    {
                        pulseLen = 0;
                        inc = true;
                        Console.WriteLine("Fading up...");
                    }
                }

                driver.SetPwmParam(channel, period, pulseLen);
                driver.EnablePwm(channel, true);

                Thread.Sleep(interval);
                if (Console.KeyAvailable && Console.ReadKey(true).Key == ConsoleKey.Escape)
                {
                    exit = true;
                }
            }
            while (!exit);

            // Disable PWM again
            driver.EnablePwm(channel, false);
        }

        private static void RunPwmFlashing(Io driver)
        {
            Console.WriteLine();
            if (driver.PwmChannels == 0)
            {
                Console.WriteLine("PWM output channels are not available on this device.");
                return;
            }
            string line;
            byte channel;
            ushort period;
            ushort maxPeriod = 400; // am62x PWM only supports up to 469ms
            ushort dutyCycle;
            do
            {
                Console.Write($"Enter channel number to use for PWM (0-{driver.PwmChannels - 1}): ");
                line = Console.ReadLine()!;
                if (line.Trim() == "") return;
            }
            while (!byte.TryParse(line, out channel) || channel >= driver.PwmChannels);
            do
            {
                Console.Write($"Enter period time in milliseconds (1-{maxPeriod}): ");
                line = Console.ReadLine()!;
                if (line.Trim() == "") return;
            }
            while (!ushort.TryParse(line, out period) || period < 1 || period > maxPeriod);
            do
            {
                Console.Write($"Enter duty cycle in percent (0-100): ");
                line = Console.ReadLine()!;
                if (line.Trim() == "") return;
            }
            while (!ushort.TryParse(line, out dutyCycle) || dutyCycle > 100);
            ushort pulseLen = (ushort)(period * dutyCycle / 100);

            Console.WriteLine();
            Console.WriteLine($"Flashing PWM output channel {channel}.");
            Console.WriteLine("Press Esc to return");

            driver.SetPwmTimeBase(channel, PwmTimebase.Ms1);
            driver.SetPwmParam(channel, period, pulseLen);
            driver.EnablePwm(channel, true);

            bool exit = false;
            do
            {
                if (Console.ReadKey(true).Key == ConsoleKey.Escape)
                {
                    exit = true;
                }
            }
            while (!exit);

            // Disable PWM again
            driver.EnablePwm(channel, false);
        }

        private static void RunRunSwitch(Io driver)
        {
            Console.WriteLine();
            Console.WriteLine("Set run switch to Stop or press Esc to return");
            if (driver.RunSwitch)
            {
                Console.WriteLine("Run switch: Run");
            }

            bool exit = false;
            do
            {
                // Wait until run switch is set to Stop or Esc key is pressed
                for (int timer = 0; timer < 10 && !exit; timer++)
                {
                    Thread.Sleep(100);
                    if (Console.KeyAvailable && Console.ReadKey(true).Key == ConsoleKey.Escape)
                    {
                        exit = true;
                    }
                    if (!driver.RunSwitch)
                    {
                        Console.WriteLine("Run switch: Stop");
                        exit = true;
                    }
                }
            }
            while (!exit);
        }

        private static void RunTemperature(Io driver)
        {
            Console.WriteLine();
            for (byte i = 0; i < driver.TemperatureSensors; i++)
            {
                float temperature = driver.GetTemperature(i);
                Console.WriteLine($"Sensor {i}:    {temperature:F1} °C");
            }
        }

        private static void RunTickCount(Io driver)
        {
            Console.WriteLine();
            uint driverTicks = driver.TickCount;
            uint envTicks = (uint)Environment.TickCount;
            Console.WriteLine("I/O driver:  " + driverTicks + " ms (" + TimeSpan.FromMilliseconds(driverTicks) + ")");
            Console.WriteLine("Environment: " + envTicks + " ms (" + TimeSpan.FromMilliseconds(envTicks) + ")");
        }

        private static void RunWatchdog(Io driver)
        {
            Console.WriteLine();
            bool monitorOnly = false;
            switch (Prompt("Watchdog mode: (M)onitor or (R)eal hardware (with reset)? ", ConsoleKey.M, ConsoleKey.R, ConsoleKey.Escape))
            {
                case ConsoleKey.M:
                    monitorOnly = true;
                    break;
                case ConsoleKey.R:
                    monitorOnly = false;
                    break;
                case ConsoleKey.Escape:
                    return;
            }

            Console.WriteLine();
            if (monitorOnly)
            {
                Console.WriteLine("Watchdog in monitor mode.");
                Console.WriteLine("Press Esc to return without hardware reset.");
            }
            else
            {
                Console.WriteLine("Watchdog in real hardware mode.");
                Console.WriteLine("Press Esc to stop servicing the watchdog, this will result in a hardware reset.");
            }

            driver.EnableWatchdog(monitorOnly);

            bool exit = false;
            do
            {
                Thread.Sleep(100);
                Console.Write("*");
                driver.ServiceWatchdog();

                if (Console.KeyAvailable && Console.ReadKey(true).Key == ConsoleKey.Escape)
                {
                    exit = true;
                }
            }
            while (!exit);

            Console.WriteLine();
            Console.WriteLine("Press Enter to service the watchdog once again.");
            exit = false;
            do
            {
                Thread.Sleep(100);
                Console.Write("*");

                if (Console.KeyAvailable && Console.ReadKey(true).Key == ConsoleKey.Enter)
                {
                    exit = true;
                }
            }
            while (!exit);
            Console.WriteLine();
            try
            {
                driver.ServiceWatchdog();
                Console.WriteLine("No error occurred!");
            }
            catch (IoException ex)
            {
                Console.WriteLine("Expected error occurred: " + ex.Message);
            }
        }
    }
}
