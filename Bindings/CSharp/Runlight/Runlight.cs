// SPDX-License-Identifier: LGPL-3.0-or-later
//
// (c) SYSTEC electronic AG, D-08468 Heinsdorfergrund, Am Windrad 2
//     www.systec-electronic.com

// ****************************************************************************
//
//   Project:      SYSTEC sysWORXX CTR-700
//   Description:  C# runlight application for the I/O driver
//
// ****************************************************************************

using System;
using System.Collections.Generic;
using System.Globalization;
using System.Linq;
using System.Text;
using System.Threading;
using Systec.Ctr700Driver;
using Mono.Unix;
using Mono.Unix.Native;

namespace Demo
{
	public static class Runlight
	{
		enum Mode { Left, Stop, Right };

		const int APP_VER_MAIN = 2;
		const int APP_VER_REL =  0;

		static private bool running = true;
		static private Mode mode = Mode.Right;

		/// <summary>
		/// Application entry point.
		/// </summary>
		/// <param name="args">The command line arguments.</param>
		/// <returns>The process exit code.</returns>
		public static int Main(string[] args)
		{
			Console.WriteLine("");
			Console.WriteLine("********************************************************************************");
			Console.WriteLine("  Test application for SYSTEC sysWORXX CTR-700 board driver");
			Console.WriteLine($"  Version: {APP_VER_MAIN}.{APP_VER_REL}");
			Console.WriteLine("  (c) 2019 SYS TEC electronic AG, www.systec-electronic.com");
			Console.WriteLine("********************************************************************************");
			Console.WriteLine("");

			try
			{
				using (var ctr700 = new Ctr700())
				{
					Init(ctr700);
					MainLoop(ctr700);
					Exit(ctr700);
				}
				return 0;
			}
			catch (Ctr700Exception ex)
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
		private static void Init(Ctr700 ctr700)
		{
			Console.WriteLine("********************************************************************************");
			Console.WriteLine($"Driver version:  {ctr700.Version}");
			Console.WriteLine($"PCB revision:    {ctr700.PcbRevision}");
			Console.WriteLine($"IO configuration:");
			Console.WriteLine($"    Digital In:  {ctr700.DigitalInputChannels}");
			Console.WriteLine($"    Digital Out: {ctr700.DigitalOutputChannels}");
			Console.WriteLine($"    Relays:      {ctr700.RelayChannels}");
			Console.WriteLine($"    Analog In:   {ctr700.AnalogInputChannels}");
			Console.WriteLine($"    Analog Out:  {ctr700.AnalogOutputChannels}");
			Console.WriteLine($"    Counter:     {ctr700.CounterChannels}");
			Console.WriteLine($"    AB Encoder:  {ctr700.EncoderChannels}");
			Console.WriteLine($"    PWM/PTO:     {ctr700.PwmChannels}");
			Console.WriteLine($"    TempSensor:  {ctr700.TemperatureSensors}");
			Console.WriteLine("********************************************************************************");
			Console.WriteLine("");

			for (byte channel = 0; channel < 3; channel++)
			{
            	Console.WriteLine($"Register DI interrupt for channel {channel}");
				ctr700.SetDigitalInputEvents(channel, Ctr700InterruptTrigger.RisingEdge);
			}

			ctr700.DigitalInputChanged += OnDigitalInputChanged;
		}

		/// <summary>
		/// Mail loop which runs the runlight
		/// </summary>
		private static void MainLoop(Ctr700 ctr700)
		{
			int outputMask = 0x07;
        	bool runLed = true;

			UnixSignal sigint = new UnixSignal(Signum.SIGINT);

			while (running)
			{
				if (sigint.WaitOne(0))
				{
					Console.Out.WriteLine("Got SIGINT, stop application");
					break;
				}

				if (!ctr700.RunSwitch)
				{
					Console.Out.WriteLine("Run switch is set to stop: Exit main loop");
					break;
				}

				if (mode != Mode.Stop)
				{
					ctr700.RunLed = runLed;
					runLed = !runLed;
					ctr700.ErrorLed = false;
				}
				else
				{
					ctr700.RunLed = false;
					ctr700.ErrorLed = true;
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

				foreach (byte channel in Enumerable.Range(0, ctr700.DigitalOutputChannels))
				{
					ctr700.SetDigitalOutput(channel, (outputMask & (1 << channel)) != 0);
				}

				ushort AdcValue = ctr700.GetAnalogInput(0);
				int delay = CalcDelay(AdcValue);
				Thread.Sleep(delay);
			}
		}

		/// <summary>
		/// De-initialize application
		/// </summary>
		private static void Exit(Ctr700 ctr700)
		{
			ctr700.RunLed = false;
			ctr700.ErrorLed = false;

			for (byte channel = 0; channel < ctr700.DigitalOutputChannels; channel++)
			{
				ctr700.SetDigitalOutput(channel, false);
			}

			ctr700.DigitalInputChanged -= OnDigitalInputChanged;
		}

		/// <summary>
		/// Handler for digital inputs
		/// </summary>
		private static void OnDigitalInputChanged(object sender, DigitalInputChangedEventArgs args)
		{
			mode = (Mode) args.Channel;
		}

		/// <summary>
		/// Convert ADC value to a delay
		/// </summary>
		private static int CalcDelay(ushort adcValue)
		{
			const float adcMin = 0;	// 0 V
			const float adcMax = 28151;	// 10 V
			const float delayMin = 500;	// 500 ms
			const float delayMax = 25;	// 25 ms

			// y = mx + n
			float m = (delayMax - delayMin) / (adcMax - adcMin);
			float n = delayMin;
			float x = ((int)adcValue) - adcMin;

			return (int) ((m * x) + n);
		}
	}
}
