// SPDX-License-Identifier: LGPL-3.0-or-later
// SPDX-FileCopyrightText: 2025 SYS TEC electronic AG <https://www.systec-electronic.com/>

// ****************************************************************************
//
//   Project:      SYSTEC sysWORXX
//   Description:  C# runlight application for the I/O driver
//
// ****************************************************************************

using System.Runtime.InteropServices;

using Sysworxx;

namespace Program
{
    public static class Program
    {
        enum Mode { Left, Stop, Right };

        static volatile private bool running = true;
        static private Mode mode = Mode.Right;

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
                using (var io = new Io())
                {
                    Init(io);
                    using (var intHandler = PosixSignalRegistration.Create(PosixSignal.SIGINT, Program.SigintHandler))
                    {
                        MainLoop(io);
                    }
                    Exit(io);
                }
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
        /// Startup of application and initialization
        /// </summary>
        private static void Init(Io io)
        {
            Console.WriteLine("********************************************************************************");
            Console.WriteLine($"Driver version:  {io.Version}");
            Console.WriteLine($"PCB revision:    {io.PcbRevision}");
            Console.WriteLine($"IO configuration:");
            Console.WriteLine($"    Digital In:  {io.DigitalInputChannels}");
            Console.WriteLine($"    Digital Out: {io.DigitalOutputChannels}");
            Console.WriteLine($"    Analog In:   {io.AnalogInputChannels}");
            Console.WriteLine($"    Analog Out:  {io.AnalogOutputChannels}");
            Console.WriteLine($"    Counter:     {io.CounterChannels}");
            Console.WriteLine($"    AB Encoder:  {io.EncoderChannels}");
            Console.WriteLine($"    PWM/PTO:     {io.PwmChannels}");
            Console.WriteLine($"    TempSensor:  {io.TemperatureSensors}");
            Console.WriteLine("********************************************************************************");
            Console.WriteLine("");

            for (byte channel = 0; channel < 3; channel++)
            {
                Console.WriteLine($"Register DI interrupt for channel {channel}");
                io.SetDigitalInputEvents(channel, InputTrigger.RisingEdge);
            }
            io.DigitalInputChanged += OnDigitalInputChanged;
        }

        private static void SigintHandler(PosixSignalContext ctx)
        {
            running = false;
            Console.WriteLine("CTRL-C pressed (SIGINT). Stopping now...");
        }

        /// <summary>
        /// Mail loop which runs the runlight
        /// </summary>
        private static void MainLoop(Io io)
        {
            int outputMask = 0x07;
            bool runLed = true;

            while (running)
            {
                if (!io.RunSwitch)
                {
                    Console.Out.WriteLine("Run switch is set to stop: Exit main loop");
                    break;
                }

                if (mode != Mode.Stop)
                {
                    io.RunLed = runLed;
                    runLed = !runLed;
                    io.ErrorLed = false;
                }
                else
                {
                    io.RunLed = false;
                    io.ErrorLed = true;
                }

                switch (mode)
                {
                    case Mode.Left:
                        {
                            outputMask = (outputMask >> 1 | outputMask << (16 - 1)) & 0xffff;
                            break;
                        }
                    case Mode.Right:
                        {
                            outputMask = (outputMask << 1 | outputMask >> (16 - 1)) & 0xffff;
                            break;
                        }
                    case Mode.Stop:
                        {
                            break;
                        }
                }

                foreach (byte channel in Enumerable.Range(0, 16))
                {
                    io.SetDigitalOutput(channel, (outputMask & (1 << channel)) != 0);
                }

                ushort adcValue = io.GetAnalogInput(0);
                int delay = CalcDelay(adcValue);
                Thread.Sleep(delay);
            }
        }

        /// <summary>
        /// De-initialize application
        /// </summary>
        private static void Exit(Io io)
        {
            io.RunLed = false;
            io.ErrorLed = false;

            foreach (byte channel in Enumerable.Range(0, 16))
            {
                io.SetDigitalOutput(channel, false);
            }

            io.DigitalInputChanged -= OnDigitalInputChanged;
        }

        /// <summary>
        /// Handler for digital inputs
        /// </summary>
        private static void OnDigitalInputChanged(object sender, DigitalInputChangedEventArgs args)
        {
            mode = (Mode)args.Channel;
        }

        /// <summary>
        /// Convert ADC value to a delay
        ///
        /// Value range depends on the kind of sysWORXX device.
        /// </summary>
        private static int CalcDelay(ushort adcValue)
        {
            const float adcMin = 0;
            const float adcMax = 12600;
            const float delayMin = 1000; // ms
            const float delayMax = 25;  // ms

            adcValue = Math.Min(adcValue, (ushort) 12600);

            // y = mx + n
            float m = (delayMax - delayMin) / (adcMax - adcMin);
            float n = delayMin;
            float x = ((float)adcValue) - adcMin;

            return (int)((m * x) + n);
        }
    }
}
