// SPDX-License-Identifier: LGPL-3.0-or-later
//
// (c) SYSTEC electronic AG, D-08468 Heinsdorfergrund, Am Windrad 2
//     www.systec-electronic.com

// ****************************************************************************
//
//   Project:      SYSTEC sysWORXX CTR-700
//   Description:  C# demo application for the I/O driver
//
// ****************************************************************************


using System;
using System.Collections.Generic;
using System.Globalization;
using System.Linq;
using System.Text;
using System.Threading;
using Systec.Ctr700Driver;

namespace Demo
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
			Console.WriteLine(".NET/Mono demo application for SYSTEC sysWORXX CTR-700");
			Console.WriteLine();

			try
			{
				MenuLoop();
				Console.WriteLine();
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
		/// Runs the application main menu loop.
		/// </summary>
		private static void MenuLoop()
		{
			var commands = new List<(ConsoleKey Key, string Label, Action<Ctr700> Action)>
			{
				(ConsoleKey.A, "Analog input", RunAnalogInput),
				(ConsoleKey.C, "Counter", RunCounter),
				(ConsoleKey.D, "Digital I/O", RunDigitalIO),
				(ConsoleKey.G, "Diagnostics", RunDiagnostics),
				(ConsoleKey.H, "PWM flashing", RunPwmFlashing),
				(ConsoleKey.I, "Tick count", RunTickCount),
				(ConsoleKey.L, "LEDs", RunLeds),
				(ConsoleKey.P, "PWM fading", RunPwmFading),
				(ConsoleKey.R, "Relays", RunRelays),
				(ConsoleKey.S, "Run switch", RunRunSwitch),
				(ConsoleKey.T, "Temperature", RunTemperature),
				(ConsoleKey.W, "Watchdog", RunWatchdog),
			};

			using (var driver = new Ctr700())
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
		private static void ShowInfo(Ctr700 driver)
		{
			Console.WriteLine("Driver version:  " + driver.Version);
			Console.WriteLine("PCB revision:    " + driver.PcbRevision);
			Console.WriteLine("IO configuration:");
			Console.WriteLine("    Digital In:  " + driver.DigitalInputChannels);
			Console.WriteLine("    Digital Out: " + driver.DigitalOutputChannels);
			Console.WriteLine("    Relays:      " + driver.RelayChannels);
			Console.WriteLine("    Analog In:   " + driver.AnalogInputChannels);
			Console.WriteLine("    Analog Out:  " + driver.AnalogOutputChannels);
			Console.WriteLine("    Counter:     " + driver.CounterChannels);
			Console.WriteLine("    AB Encoder:  " + driver.EncoderChannels);
			Console.WriteLine("    PWM/PTO:     " + driver.PwmChannels);
			Console.WriteLine("    TempSensor:  " + driver.TemperatureSensors);
		}

		/// <summary>
		/// Shows the main menu.
		/// </summary>
		/// <param name="commands">The available demo commands.</param>
		private static void ShowMainMenu(List<(ConsoleKey Key, string Label, Action<Ctr700> Action)> commands)
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

		private static void RunAnalogInput(Ctr700 driver)
		{
			Console.WriteLine();
			if (driver.AnalogInputChannels == 0)
			{
				Console.WriteLine("Analog input channels are not available on this device.");
				return;
			}
			string line;
			byte channel;
			Ctr700AdcMode? mode = null;
			string modeText = "";
			string unit = "";
			float unitFactor = 1;
			do
			{
				Console.Write($"Enter channel number to read (0-{driver.AnalogInputChannels - 1}): ");
				line = Console.ReadLine();
				if (line.Trim() == "") return;
			}
			while (!byte.TryParse(line, out channel) || channel >= driver.AnalogInputChannels);
			switch (Prompt("Channel mode: (R)aw, (V)oltage or (C)urrent? ", ConsoleKey.R, ConsoleKey.V, ConsoleKey.C, ConsoleKey.Escape))
			{
				case ConsoleKey.R:
					mode = null;
					modeText = "raw values";
					break;
				case ConsoleKey.V:
					mode = Ctr700AdcMode.Voltage;
					modeText = "voltage";
					unit = "V";
					break;
				case ConsoleKey.C:
					mode = Ctr700AdcMode.Current;
					modeText = "current";
					unit = "mA";
					unitFactor = 1000;
					break;
				case ConsoleKey.Escape:
					return;
			}

			Console.WriteLine();
			Console.WriteLine($"Reading {modeText} from analog input channel {channel}.");
			Console.WriteLine("Press Esc to return");

			if (mode != null)
			{
				driver.SetAnalogMode(channel, (Ctr700AdcMode)mode);
			}

			bool exit = false;
			do
			{
				// Show current analog input value
				if (mode != null)
				{
					float value = driver.GetAnalogInput(channel, (Ctr700AdcMode)mode);
					value /= unitFactor;
					Console.WriteLine(value.ToString("0.0000", CultureInfo.InvariantCulture) + " " + unit);
				}
				else
				{
					ushort value = driver.GetAnalogInput(channel);
					Console.WriteLine(value);
				}

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

		private static void RunCounter(Ctr700 driver)
		{
			Console.WriteLine();
			if (driver.CounterChannels == 0)
			{
				Console.WriteLine("Counters are not available on this device.");
				return;
			}
			string line;
			byte channel;
			Ctr700CounterTrigger trigger = 0;
			do
			{
				Console.Write($"Enter channel number to read (0-{driver.CounterChannels - 1}): ");
				line = Console.ReadLine();
				if (line.Trim() == "") return;
			}
			while (!byte.TryParse(line, out channel) || channel >= driver.CounterChannels);
			switch (Prompt("Counter trigger: (R)ising edge, (F)alling edge or (B)oth edges? ", ConsoleKey.R, ConsoleKey.F, ConsoleKey.B, ConsoleKey.Escape))
			{
				case ConsoleKey.R:
					trigger = Ctr700CounterTrigger.RisingEdge;
					break;
				case ConsoleKey.F:
					trigger = Ctr700CounterTrigger.FallingEdge;
					break;
				case ConsoleKey.B:
					trigger = Ctr700CounterTrigger.BothEdges;
					break;
				case ConsoleKey.Escape:
					return;
			}

			Console.WriteLine();
			Console.WriteLine($"Counting on channel {channel} upwards, starting at 0.");
			Console.WriteLine("Press Esc to return");

			driver.EnableCounter(channel, true);
			driver.SetCounterMode(channel, Ctr700CounterMode.Counter, trigger, Ctr700CounterDirection.Up);

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

		private static void RunDiagnostics(Ctr700 driver)
		{
			Console.WriteLine();
			Console.WriteLine("Digital output power failure: " + (driver.IsDigitalOutputPowerFailure ? "yes" : "no"));
			Console.WriteLine("Digital output error:         " + (driver.IsDigitalOutputError ? "yes" : "no"));
			Console.WriteLine("Digital input error:          " + (driver.IsDigitalInputError ? "yes" : "no"));
			Console.WriteLine("USB over-current:             " + (driver.IsUsbOverCurrent ? "yes" : "no"));
			Console.WriteLine("Backplane bus EXT_FAIL:       " + (driver.IsExtFail ? "yes" : "no"));
		}

		private static void RunDigitalIO(Ctr700 driver)
		{
			Console.WriteLine();
			Console.WriteLine("Reading digital input channels and cycling digital output channels.");
			Console.WriteLine("Press Esc to return");

			// Enable events for all digital input channels on both edges
			for (byte channel = 0; channel < driver.DigitalInputChannels; channel++)
			{
				driver.SetDigitalInputEvents(channel, Ctr700InterruptTrigger.BothEdges);
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
					bool state = driver.GetDigitalInput(channel);
					if (channel > 0)
					{
						if ((channel % 4) == 0)
							sb.Append(" ");
					}
					sb.Append(state ? "x" : "_");
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
						output = (byte)((output + 1) % driver.DigitalOutputChannels);
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
				driver.SetDigitalInputEvents(channel, Ctr700InterruptTrigger.None);
			}
			driver.DigitalInputChanged -= Driver_DigitalInputChanged;
		}

		private static void Driver_DigitalInputChanged(object sender, DigitalInputChangedEventArgs args)
		{
			Console.WriteLine($"Digital input {args.Channel} is now {(args.State ? "on" : "off")}");
		}

		private static void RunLeds(Ctr700 driver)
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

		private static void RunPwmFading(Ctr700 driver)
		{
			Console.WriteLine();
			if (driver.PwmChannels == 0)
			{
				Console.WriteLine("PWM output channels are not available on this device.");
				return;
			}
			string line;
			byte channel;
			Ctr700PwmTimeBase timeBase = 0;
			ushort period = 20;
			ushort step = 1;
			int interval = 200;
			do
			{
				Console.Write($"Enter channel number to use for PWM (0-{driver.PwmChannels - 1}): ");
				line = Console.ReadLine();
				if (line.Trim() == "") return;
			}
			while (!byte.TryParse(line, out channel) || channel >= driver.PwmChannels);
			switch (Prompt("Time base: (A) 800 ns or (B) 1 ms? ", ConsoleKey.A, ConsoleKey.B, ConsoleKey.Escape))
			{
				case ConsoleKey.A:
					timeBase = Ctr700PwmTimeBase.TimeBase800ns;
					period = 200;
					step = 1;
					interval = 20;
					break;
				case ConsoleKey.B:
					timeBase = Ctr700PwmTimeBase.TimeBase1ms;
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

		private static void RunPwmFlashing(Ctr700 driver)
		{
			Ctr700PwmTimeBase timeBase = Ctr700PwmTimeBase.TimeBase1ms;
			ushort maxPeriod = driver.GetMaxPwmPeriod(timeBase);

			Console.WriteLine();
			if (driver.PwmChannels == 0)
			{
				Console.WriteLine("PWM output channels are not available on this device.");
				return;
			}
			string line;
			byte channel;
			ushort period;
			ushort dutyCycle;
			do
			{
				Console.Write($"Enter channel number to use for PWM (0-{driver.PwmChannels - 1}): ");
				line = Console.ReadLine();
				if (line.Trim() == "") return;
			}
			while (!byte.TryParse(line, out channel) || channel >= driver.PwmChannels);
			do
			{
				Console.Write($"Enter period time in milliseconds (1-{maxPeriod}): ");
				line = Console.ReadLine();
				if (line.Trim() == "") return;
			}
			while (!ushort.TryParse(line, out period) || period < 1 || period > maxPeriod);
			do
			{
				Console.Write($"Enter duty cycle in percent (0-100): ");
				line = Console.ReadLine();
				if (line.Trim() == "") return;
			}
			while (!ushort.TryParse(line, out dutyCycle) || dutyCycle > 100);
			ushort pulseLen = (ushort)(period * dutyCycle / 100);

			Console.WriteLine();
			Console.WriteLine($"Flashing PWM output channel {channel}.");
			Console.WriteLine("Press Esc to return");

			driver.SetPwmTimeBase(channel, timeBase);
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

		private static void RunRelays(Ctr700 driver)
		{
			Console.WriteLine();
			if (driver.RelayChannels == 0)
			{
				Console.WriteLine("Relay output channels are not available on this device.");
				return;
			}
			Console.WriteLine("Cycling relay output channels.");
			Console.WriteLine("Press Esc to return");

			int phase = -1;
			bool exit = false;
			do
			{
				// De/Activate the next relay output
				phase = (phase + 1) % (driver.RelayChannels * 2);
				byte channel = (byte)(phase % driver.RelayChannels);
				bool state = phase < driver.RelayChannels;
				driver.SetRelay(channel, state);

				Console.WriteLine($"Relay {channel}: {(state ? "on" : "off")}");

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

			// Turn off all relays again
			for (byte channel = 0; channel < driver.RelayChannels; channel++)
			{
				driver.SetRelay(channel, false);
			}
		}

		private static void RunRunSwitch(Ctr700 driver)
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

		private static void RunTemperature(Ctr700 driver)
		{
			Console.WriteLine();
			Console.WriteLine("CPU temperature:    " + driver.GetTemperature(Ctr700TemperatureSensor.Cpu).ToString("0.0", CultureInfo.InvariantCulture) + " °C");
			Console.WriteLine("System temperature: " + driver.GetTemperature(Ctr700TemperatureSensor.System).ToString("0.0", CultureInfo.InvariantCulture) + " °C");
		}

		private static void RunTickCount(Ctr700 driver)
		{
			Console.WriteLine();
			uint driverTicks = driver.TickCount;
			uint envTicks = (uint)Environment.TickCount;
			Console.WriteLine("I/O driver:  " + driverTicks + " ms (" + TimeSpan.FromMilliseconds(driverTicks) + ")");
			Console.WriteLine("Environment: " + envTicks + " ms (" + TimeSpan.FromMilliseconds(envTicks) + ")");
		}

		private static void RunWatchdog(Ctr700 driver)
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
			catch (Ctr700Exception ex)
			{
				Console.WriteLine("Expected error occurred: " + ex.Message);
			}
		}
	}
}
