// SPDX-License-Identifier: LGPL-3.0-or-later
//
// (c) SYSTEC electronic AG, D-08468 Heinsdorfergrund, Am Windrad 2
//     www.systec-electronic.com

// ****************************************************************************
//
//   Project:      SYSTEC sysWORXX CTR-700
//   Description:  C# bindings for the I/O driver
//
// ****************************************************************************

using System;
using System.Collections.Generic;
using System.Runtime.InteropServices;

namespace Systec.Ctr700Driver
{
	/// <summary>
	/// Provides access to the SYSTEC sysWORXX CTR-700 hardware through the native I/O driver.
	/// </summary>
	public class Ctr700 : IDisposable
	{
		#region Private data

		/// <summary>
		/// Indicates whether an instance of the <see cref="Ctr700"/> class has been initialized and
		/// not yet been disposed. There can only be one driver initialization at a time in a
		/// process.
		/// </summary>
		private static bool isInitialized;

		/// <summary>
		/// Indicates whether the hardware information has been loaded.
		/// </summary>
		private bool haveHardwareInfo;

		/// <summary>
		/// The cached hardware information.
		/// </summary>
		private NativeMethods.HardwareInfo hardwareInfo;

		/// <summary>
		/// Must be kept from GC. See Marshal.GetFunctionPointerForDelegate documentation.
		/// </summary>
		private readonly Ctr700InterruptHandler interruptHandlerDelegate;

		/// <summary>
		/// A function pointer to pass to unmanaged code.
		/// </summary>
		private readonly IntPtr interruptHandlerFnPtr;

		/// <summary>
		/// The channels for which an interrupt handler is currently registered.
		/// </summary>
		private readonly HashSet<byte> interruptChannels = new HashSet<byte>();

		#endregion Private data

		#region Constructors

		/// <summary>
		/// Initializes a new instance of the <see cref="Ctr700"/> class and the native I/O driver.
		/// </summary>
		public Ctr700()
		{
			if (isInitialized)
				throw new InvalidOperationException("There can only be one driver instance at a time in a process.");
			isInitialized = true;

			ThrowOnError(NativeMethods.GetVersion(out byte major, out byte minor));
			if (major != 2 || minor < 0)
				throw new NotSupportedException($"CTR-700 driver version {major}.{minor} is not supported.");
			Version = new Version(major, minor);

			ThrowOnError(NativeMethods.Initialize());

			interruptHandlerDelegate = new Ctr700InterruptHandler(InterruptHandler);
			interruptHandlerFnPtr = Marshal.GetFunctionPointerForDelegate(interruptHandlerDelegate);
		}

		#endregion Constructors

		#region Events

		/// <summary>
		/// Occurs when the state of a digital input has changed. Call
		/// <see cref="SetDigitalInputEvents"/> to enable or disable this event for a channel.
		/// </summary>
		public event EventHandler<DigitalInputChangedEventArgs> DigitalInputChanged;

		#endregion Events

		#region Properties

		/// <summary>
		/// Gets the version of the I/O driver.
		/// </summary>
		public Version Version { get; }

		/// <summary>
		/// Gets a value indicating whether the driver has been shut down.
		/// </summary>
		public bool IsDisposed { get; private set; }

		/// <summary>
		/// Gets the number of milliseconds elapsed since the system started.
		/// </summary>
		public uint TickCount
		{
			get
			{
				ThrowIfDisposed();
				ThrowOnError(NativeMethods.GetTickCount(out uint tickCount));
				return tickCount;
			}
		}

		/// <summary>
		/// Turns the run LED on or off.
		/// </summary>
		public bool RunLed
		{
			set
			{
				ThrowIfDisposed();
				ThrowOnError(NativeMethods.SetRunLed(value));
			}
		}

		/// <summary>
		/// Turns the error LED on or off.
		/// </summary>
		public bool ErrorLed
		{
			set
			{
				ThrowIfDisposed();
				ThrowOnError(NativeMethods.SetErrLed(value));
			}
		}

		/// <summary>
		/// Gets a value indicating whether the run switch is in the Run position.
		/// </summary>
		public bool RunSwitch
		{
			get
			{
				ThrowIfDisposed();
				ThrowOnError(NativeMethods.GetRunSwitch(out bool state));
				// The I/O driver returns true for the Stop position.
				// This method inverts that to return true for the positive Run position.
				return state;
			}
		}

		/// <summary>
		/// Gets a value indicating whether the config switch (DIP 4) is on.
		/// </summary>
		public bool IsConfigEnabled
		{
			get
			{
				ThrowIfDisposed();
				ThrowOnError(NativeMethods.GetConfigEnabled(out bool state));
				return state;
			}
		}

		/// <summary>
		/// Gets the state of the power fail signal. This signal is set when the device power supply
		/// drops below a value of around 18.2 V.
		/// </summary>
		public bool IsPowerFail
		{
			get
			{
				ThrowIfDisposed();
				ThrowOnError(NativeMethods.GetPowerFail(out bool state));
				return state;
			}
		}

		/// <summary>
		/// Gets the state of the EXT_FAIL signal on the backplane bus.
		/// </summary>
		public bool IsExtFail
		{
			get
			{
				ThrowIfDisposed();
				ThrowOnError(NativeMethods.GetExtFail(out bool state));
				return state;
			}
		}

		/// <summary>
		/// Sets the state of the EXT_RESET signal on the backplane bus.
		/// </summary>
		public bool ExtReset
		{
			set
			{
				ThrowIfDisposed();
				ThrowOnError(NativeMethods.SetExtReset(value));
			}
		}

		#endregion Properties

		#region Hardware information properties

		/// <summary>
		/// Gets the PCB revision number.
		/// </summary>
		public byte PcbRevision
		{
			get
			{
				EnsureHardwareInfo();
				return hardwareInfo.PcbRevision;
			}
		}

		/// <summary>
		/// Gets the number of digital input channels supported by the hardware.
		/// </summary>
		public ushort DigitalInputChannels
		{
			get
			{
				EnsureHardwareInfo();
				return hardwareInfo.DiChannels;
			}
		}

		/// <summary>
		/// Gets the number of digital output channels supported by the hardware.
		/// </summary>
		public ushort DigitalOutputChannels
		{
			get
			{
				EnsureHardwareInfo();
				return hardwareInfo.DoChannels;
			}
		}

		/// <summary>
		/// Gets the number of relay output channels supported by the hardware.
		/// </summary>
		public ushort RelayChannels
		{
			get
			{
				EnsureHardwareInfo();
				return hardwareInfo.RelayChannels;
			}
		}

		/// <summary>
		/// Gets the number of analog input channels supported by the hardware.
		/// </summary>
		public ushort AnalogInputChannels
		{
			get
			{
				EnsureHardwareInfo();
				return hardwareInfo.AiChannels;
			}
		}

		/// <summary>
		/// Gets the number of analog output channels supported by the hardware.
		/// </summary>
		public ushort AnalogOutputChannels
		{
			get
			{
				EnsureHardwareInfo();
				return hardwareInfo.AoChannels;
			}
		}

		/// <summary>
		/// Gets the number of counter input channels supported by the hardware.
		/// </summary>
		public ushort CounterChannels
		{
			get
			{
				EnsureHardwareInfo();
				return hardwareInfo.CntChannels;
			}
		}

		/// <summary>
		/// Gets the number of A/B decoder channels supported by the hardware.
		/// </summary>
		public ushort EncoderChannels
		{
			get
			{
				EnsureHardwareInfo();
				return hardwareInfo.EncChannels;
			}
		}

		/// <summary>
		/// Gets the number of PWM output channels supported by the hardware.
		/// </summary>
		public ushort PwmChannels
		{
			get
			{
				EnsureHardwareInfo();
				return hardwareInfo.PwmChannels;
			}
		}

		/// <summary>
		/// Gets the number of temperature sensors supported by the hardware.
		/// </summary>
		public ushort TemperatureSensors
		{
			get
			{
				EnsureHardwareInfo();
				return hardwareInfo.TmpChannels;
			}
		}

		#endregion Hardware information properties

		#region Diagnostic information properties

		/// <summary>
		/// Gets a value indicating whether the power supply for digital outputs is not properly
		/// connected.
		/// </summary>
		public bool IsDigitalOutputPowerFailure
		{
			get
			{
				ThrowIfDisposed();
				ThrowOnError(NativeMethods.GetDiagInfo(out var diagnosticInfo));
				// The I/O driver returns an inverted signal.
				// This method normalises that to true on error and false on normal operation.
				return !diagnosticInfo.DigiOutPowerFail;
			}
		}

		/// <summary>
		/// Gets a value indicating whether there is an error for digital outputs. Returns true if
		/// the driver IC temperature it too high or there is an internal communication error of the
		/// driver IC.
		/// </summary>
		public bool IsDigitalOutputError
		{
			get
			{
				ThrowIfDisposed();
				ThrowOnError(NativeMethods.GetDiagInfo(out var diagnosticInfo));
				return diagnosticInfo.DigiOutDiag;
			}
		}

		/// <summary>
		/// Gets a value indicating whether there is an error for digital outputs. Returns true if
		/// the power supply is not connected to the driver IC or there is an internal communication
		/// error of the driver IC.
		/// </summary>
		public bool IsDigitalInputError
		{
			get
			{
				ThrowIfDisposed();
				ThrowOnError(NativeMethods.GetDiagInfo(out var diagnosticInfo));
				return diagnosticInfo.DigiInError;
			}
		}

		/// <summary>
		/// Gets a value indicating whether the USB interface current is too high.
		/// </summary>
		public bool IsUsbOverCurrent
		{
			get
			{
				ThrowIfDisposed();
				ThrowOnError(NativeMethods.GetDiagInfo(out var diagnosticInfo));
				return diagnosticInfo.UsbOverCurrent;
			}
		}

		#endregion Diagnostic information properties

		#region Public methods

		/// <summary>
		/// Enables the system watchdog. If the watchdog was not serviced in time by calling the
		/// <see cref="ServiceWatchdog"/> method, that method will throw an exception with the code
		/// <see cref="Ctr700Result.WatchdogTimeout"/>. The watchdog timeout is 1000 ms in
		/// non-monitor mode and 900 ms in monitor mode.
		/// </summary>
		/// <param name="monitorOnly">true to start the watchdog in monitor mode; false for real hardware mode.</param>
		public void EnableWatchdog(bool monitorOnly)
		{
			ThrowIfDisposed();
			ThrowOnError(NativeMethods.EnableWatchdog(monitorOnly));
		}

		/// <summary>
		/// Services the watchdog. See <see cref="EnableWatchdog"/> for details.
		/// </summary>
		public void ServiceWatchdog()
		{
			ThrowIfDisposed();
			ThrowOnError(NativeMethods.ServiceWatchdog());
		}

		/// <summary>
		/// Gets the current state of a digital input channel.
		/// </summary>
		/// <param name="channel">The channel number.</param>
		/// <returns>true if the input is on; otherwise, false.</returns>
		public bool GetDigitalInput(byte channel)
		{
			ThrowIfDisposed();
			ThrowOnError(NativeMethods.GetDigiIn(channel, out bool state));
			return state;
		}

		/// <summary>
		/// Enables or disables events for a digital input channel.
		/// </summary>
		/// <param name="channel">The channel number.</param>
		/// <param name="trigger">The trigger to raise the event.</param>
		public void SetDigitalInputEvents(byte channel, Ctr700InterruptTrigger trigger)
		{
			ThrowIfDisposed();
			if (trigger != Ctr700InterruptTrigger.None)
			{
				if (!interruptChannels.Contains(channel))
				{
					ThrowOnError(NativeMethods.RegisterInterruptCallback(channel, interruptHandlerFnPtr, trigger));
					interruptChannels.Add(channel);
				}
			}
			else
			{
				if (interruptChannels.Contains(channel))
				{
					ThrowOnError(NativeMethods.UnregisterInterruptCallback(channel));
					interruptChannels.Remove(channel);
				}
			}
		}

		/// <summary>
		/// Sets the state of a digital output channel.
		/// </summary>
		/// <param name="channel">The channel number.</param>
		/// <param name="state">true to turn on the output, false to turn it off.</param>
		public void SetDigitalOutput(byte channel, bool state)
		{
			ThrowIfDisposed();
			ThrowOnError(NativeMethods.SetDigiOut(channel, state));
		}

		/// <summary>
		/// Sets the state of a relay output channel.
		/// </summary>
		/// <param name="channel">The channel number.</param>
		/// <param name="state">true to turn on the relay, false to turn it off.</param>
		public void SetRelay(byte channel, bool state)
		{
			ThrowIfDisposed();
			ThrowOnError(NativeMethods.SetRelay(channel, state));
		}

		/// <summary>
		/// Enables or disables a counter input channel.
		/// </summary>
		/// <param name="channel">The channel number.</param>
		/// <param name="state">true to enable the counter, false to disable it.</param>
		public void EnableCounter(byte channel, bool state)
		{
			ThrowIfDisposed();
			ThrowOnError(NativeMethods.CntEnable(channel, state));
		}

		/// <summary>
		/// Sets the mode of a counter input channel.
		/// </summary>
		/// <param name="channel">The channel number.</param>
		/// <param name="mode">The counter channel mode.</param>
		/// <param name="trigger">The counter trigger.</param>
		/// <param name="direction">The counter direction.</param>
		public void SetCounterMode(byte channel, Ctr700CounterMode mode, Ctr700CounterTrigger trigger, Ctr700CounterDirection direction)
		{
			ThrowIfDisposed();
			ThrowOnError(NativeMethods.CntSetMode(channel, mode, trigger, direction));
		}

		/// <summary>
		/// Sets the initial value of a counter.
		/// </summary>
		/// <param name="channel">The channel number.</param>
		/// <param name="initialValue">The initial value to set.</param>
		public void SetCounterInitialValue(byte channel, int initialValue)
		{
			ThrowIfDisposed();
			ThrowOnError(NativeMethods.CntSetPreload(channel, initialValue));
		}

		/// <summary>
		/// Gets the current value of a counter.
		/// </summary>
		/// <param name="channel">The channel number.</param>
		/// <returns>The current value of the counter.</returns>
		public int GetCounterValue(byte channel)
		{
			ThrowIfDisposed();
			ThrowOnError(NativeMethods.CntGetValue(channel, out int value));
			return value;
		}

		/// <summary>
		/// Enables or disables a PWM output channel. The parameters must be set with
		/// <see cref="SetPwmParam"/> before enabling the PWM channel.
		/// </summary>
		/// <param name="channel">The channel number.</param>
		/// <param name="state">true to enable the PWM, false to disable it.</param>
		public void EnablePwm(byte channel, bool state)
		{
			ThrowIfDisposed();
			ThrowOnError(NativeMethods.PwmEnable(channel, state));
		}

		/// <summary>
		/// Sets the time base of a PWM output channel.
		/// </summary>
		/// <param name="channel">The channel number.</param>
		/// <param name="timeBase">The time base.</param>
		public void SetPwmTimeBase(byte channel, Ctr700PwmTimeBase timeBase)
		{
			ThrowIfDisposed();
			ThrowOnError(NativeMethods.PwmSetTimeBase(channel, timeBase));
		}

		/// <summary>
		/// Sets the parameters of a PWM output channel. The new parameters are only applied after
		/// calling <see cref="EnablePwm"/> (again).
		/// </summary>
		/// <param name="channel">The channel number.</param>
		/// <param name="period">The period length in units set by <see cref="Ctr700PwmTimeBase"/>.</param>
		/// <param name="pulseLen">The pulse length of the signal ("on" time / duty cycle).</param>
		public void SetPwmParam(byte channel, ushort period, ushort pulseLen)
		{
			ThrowIfDisposed();
			ThrowOnError(NativeMethods.PwmSetParam(channel, period, pulseLen));
		}

		/// <summary>
		/// Gets the maximum supported period for PWM output and the specified time base.
		/// </summary>
		/// <param name="timeBase">The time base.</param>
		/// <returns>The maximum supported period.</returns>
		public ushort GetMaxPwmPeriod(Ctr700PwmTimeBase timeBase)
		{
			switch (timeBase)
			{
				case Ctr700PwmTimeBase.TimeBase800ns: return ushort.MaxValue;
				case Ctr700PwmTimeBase.TimeBase1ms: return 2147;
				default: throw new ArgumentException("Invalid time base.", nameof(timeBase));
			}
		}

		/// <summary>
		/// Gets the current value of an analog input channel.
		/// </summary>
		/// <param name="channel">The channel number.</param>
		/// <returns>The raw value.</returns>
		public ushort GetAnalogInput(byte channel)
		{
			ThrowIfDisposed();
			ThrowOnError(NativeMethods.AdcGetValue(channel, out ushort value));
			return value;
		}

		/// <summary>
		/// Gets the current converted value of an analog input channel.
		/// </summary>
		/// <param name="channel">The channel number.</param>
		/// <param name="mode">The input channel mode.</param>
		/// <returns>The converted value, depending on <paramref name="mode"/>.</returns>
		public float GetAnalogInput(byte channel, Ctr700AdcMode mode)
		{
			ThrowIfDisposed();
			ThrowOnError(NativeMethods.AdcGetValue(channel, out ushort value));
			switch (mode)
			{
				case Ctr700AdcMode.Voltage:
					return (float)value / 28151 * 10;   // Volt
				case Ctr700AdcMode.Current:
					return (float)value / 24394 * 0.02f;   // Ampere
				default:
					throw new ArgumentOutOfRangeException(nameof(mode), mode, "Analog input mode not recognized.");
			}
		}

		/// <summary>
		/// Sets the mode of an analog input channel.
		/// </summary>
		/// <param name="channel">The channel number.</param>
		/// <param name="mode">The input channel mode.</param>
		/// <remarks>
		/// The ADC channel has a default configuration determined by the operating system
		/// configuration. See file /etc/systec/adc_modes.
		/// </remarks>
		public void SetAnalogMode(byte channel, Ctr700AdcMode mode)
		{
			ThrowIfDisposed();
			ThrowOnError(NativeMethods.AdcSetMode(channel, mode));
		}

		/// <summary>
		/// Gets the current value of a temperature sensor.
		/// </summary>
		/// <param name="sensor">The temperature sensor.</param>
		/// <returns>The measured temperature in °C with a resolution of 0.5 °C.</returns>
		public float GetTemperature(Ctr700TemperatureSensor sensor)
		{
			ThrowIfDisposed();
			ThrowOnError(NativeMethods.TmpGetValue(sensor, out int value));
			return value / 10000f;
		}

		#endregion Public methods

		#region Private methods

		private void ThrowIfDisposed()
		{
			if (IsDisposed)
			{
				throw new ObjectDisposedException(nameof(Ctr700));
			}
		}

		private void ThrowOnError(Ctr700Result result)
		{
			if (result != Ctr700Result.Successful)
			{
				throw new Ctr700Exception(result);
			}
		}

		private void EnsureHardwareInfo()
		{
			if (!haveHardwareInfo)
			{
				ThrowIfDisposed();
				ThrowOnError(NativeMethods.GetHardwareInfo(out hardwareInfo));
				haveHardwareInfo = true;
			}
		}

		private void InterruptHandler(byte channel, bool state)
		{
			DigitalInputChanged?.Invoke(this, new DigitalInputChangedEventArgs(channel, state));
		}

		#endregion Private methods

		#region IDisposable members and finalizer

		/// <summary>
		/// Releases all resources used by the driver.
		/// </summary>
		public void Dispose()
		{
			Dispose(true);
			GC.SuppressFinalize(this);
		}

		/// <summary>
		/// Releases all unmanaged resources used by the driver.
		/// </summary>
		~Ctr700()
		{
			Dispose(false);
		}

		private void Dispose(bool disposing)
		{
			if (!IsDisposed)
			{
				if (disposing)
				{
					// Dispose managed resources
				}

				// Unregister callbacks
				foreach (byte channel in interruptChannels)
				{
					NativeMethods.UnregisterInterruptCallback(channel);
				}

				// Free unmanaged resources
				NativeMethods.ShutDown();

				isInitialized = false;
				IsDisposed = true;
			}
		}

		#endregion IDisposable members and finalizer

		#region External native methods

		[UnmanagedFunctionPointer(CallingConvention.Cdecl)]
		private delegate void Ctr700InterruptHandler(byte channel, [MarshalAs(UnmanagedType.I1)] bool state);

		private static class NativeMethods
		{
			public struct HardwareInfo
			{
				public byte PcbRevision;
				public ushort DiChannels;
				public ushort DoChannels;
				public ushort RelayChannels;
				public ushort AiChannels;
				public ushort AoChannels;
				public ushort CntChannels;
				public ushort EncChannels;
				public ushort PwmChannels;
				public ushort TmpChannels;
			}

			public struct DiagnosticInfo
			{
				public bool DigiOutPowerFail;
				public bool DigiOutDiag;
				public bool DigiInError;
				public bool UsbOverCurrent;
			}

			[DllImport("libctr700drv.so", EntryPoint = "Ctr700DrvGetVersion")]
			public static extern Ctr700Result GetVersion(out byte major, out byte minor);

			[DllImport("libctr700drv.so", EntryPoint = "Ctr700DrvInitialize")]
			public static extern Ctr700Result Initialize();

			[DllImport("libctr700drv.so", EntryPoint = "Ctr700DrvShutDown")]
			public static extern Ctr700Result ShutDown();

			[DllImport("libctr700drv.so", EntryPoint = "Ctr700DrvGetTickCount")]
			public static extern Ctr700Result GetTickCount(out uint tickCount);

			[DllImport("libctr700drv.so", EntryPoint = "Ctr700DrvEnableWatchdog")]
			public static extern Ctr700Result EnableWatchdog([MarshalAs(UnmanagedType.I1)] bool monitorOnly);

			[DllImport("libctr700drv.so", EntryPoint = "Ctr700DrvServiceWatchdog")]
			public static extern Ctr700Result ServiceWatchdog();

			[DllImport("libctr700drv.so", EntryPoint = "Ctr700DrvGetHardwareInfo")]
			public static extern Ctr700Result GetHardwareInfo(out HardwareInfo hwInfo);

			[DllImport("libctr700drv.so", EntryPoint = "Ctr700DrvSetRunLed")]
			public static extern Ctr700Result SetRunLed([MarshalAs(UnmanagedType.I1)] bool state);

			[DllImport("libctr700drv.so", EntryPoint = "Ctr700DrvSetErrLed")]
			public static extern Ctr700Result SetErrLed([MarshalAs(UnmanagedType.I1)] bool state);

			[DllImport("libctr700drv.so", EntryPoint = "Ctr700DrvGetRunSwitch")]
			public static extern Ctr700Result GetRunSwitch([MarshalAs(UnmanagedType.I1)] out bool state);

			[DllImport("libctr700drv.so", EntryPoint = "Ctr700DrvGetConfigEnabled")]
			public static extern Ctr700Result GetConfigEnabled([MarshalAs(UnmanagedType.I1)] out bool state);

			[DllImport("libctr700drv.so", EntryPoint = "Ctr700DrvGetPowerFail")]
			public static extern Ctr700Result GetPowerFail([MarshalAs(UnmanagedType.I1)] out bool state);

			[DllImport("libctr700drv.so", EntryPoint = "Ctr700DrvGetDiagInfo")]
			public static extern Ctr700Result GetDiagInfo(out DiagnosticInfo diagnosticInfo);

			[DllImport("libctr700drv.so", EntryPoint = "Ctr700DrvGetExtFail")]
			public static extern Ctr700Result GetExtFail([MarshalAs(UnmanagedType.I1)] out bool state);

			[DllImport("libctr700drv.so", EntryPoint = "Ctr700DrvSetExtReset")]
			public static extern Ctr700Result SetExtReset([MarshalAs(UnmanagedType.I1)] bool state);

			[DllImport("libctr700drv.so", EntryPoint = "Ctr700DrvGetDigiIn")]
			public static extern Ctr700Result GetDigiIn(byte channel, [MarshalAs(UnmanagedType.I1)] out bool state);

			[DllImport("libctr700drv.so", EntryPoint = "Ctr700DrvSetDigiOut")]
			public static extern Ctr700Result SetDigiOut(byte channel, [MarshalAs(UnmanagedType.I1)] bool state);

			[DllImport("libctr700drv.so", EntryPoint = "Ctr700DrvSetRelay")]
			public static extern Ctr700Result SetRelay(byte channel, [MarshalAs(UnmanagedType.I1)] bool state);

			[DllImport("libctr700drv.so", EntryPoint = "Ctr700DrvCntEnable")]
			public static extern Ctr700Result CntEnable(byte channel, [MarshalAs(UnmanagedType.I1)] bool state);

			[DllImport("libctr700drv.so", EntryPoint = "Ctr700DrvCntSetMode")]
			public static extern Ctr700Result CntSetMode(byte channel, Ctr700CounterMode mode, Ctr700CounterTrigger trigger, Ctr700CounterDirection dir);

			[DllImport("libctr700drv.so", EntryPoint = "Ctr700DrvCntSetPreload")]
			public static extern Ctr700Result CntSetPreload(byte channel, int value);

			[DllImport("libctr700drv.so", EntryPoint = "Ctr700DrvCntGetValue")]
			public static extern Ctr700Result CntGetValue(byte channel, out int value);

			[DllImport("libctr700drv.so", EntryPoint = "Ctr700DrvPwmSetTimeBase")]
			public static extern Ctr700Result PwmSetTimeBase(byte channel, Ctr700PwmTimeBase timeBase);

			[DllImport("libctr700drv.so", EntryPoint = "Ctr700DrvPwmSetParam")]
			public static extern Ctr700Result PwmSetParam(byte channel, ushort period, ushort pulseLen);

			[DllImport("libctr700drv.so", EntryPoint = "Ctr700DrvPwmEnable")]
			public static extern Ctr700Result PwmEnable(byte channel, [MarshalAs(UnmanagedType.I1)] bool state);

			// Not implemented in the I/O driver yet
			//[DllImport("libctr700drv.so", EntryPoint = "Ctr700DrvPtoSetParam")]
			//public static extern Ctr700Result PtoSetParam(byte channel, ushort period, short delta, uint pulseCnt);

			// Not implemented in the I/O driver yet
			//[DllImport("libctr700drv.so", EntryPoint = "Ctr700DrvPtoEnable")]
			//public static extern Ctr700Result PtoEnable(byte channel, [MarshalAs(UnmanagedType.I1)] bool run);

			// Not implemented in the I/O driver yet
			//[DllImport("libctr700drv.so", EntryPoint = "Ctr700DrvPtoGetState")]
			//public static extern Ctr700Result PtoGetState(byte channel, [MarshalAs(UnmanagedType.I1)] out bool state);

			[DllImport("libctr700drv.so", EntryPoint = "Ctr700DrvAdcGetValue")]
			public static extern Ctr700Result AdcGetValue(byte channel, out ushort adcValue);

			[DllImport("libctr700drv.so", EntryPoint = "Ctr700DrvAdcSetMode")]
			public static extern Ctr700Result AdcSetMode(byte channel, Ctr700AdcMode mode);

			[DllImport("libctr700drv.so", EntryPoint = "Ctr700DrvTmpGetValue")]
			public static extern Ctr700Result TmpGetValue(Ctr700TemperatureSensor sensor, out int tmpValue);

			[DllImport("libctr700drv.so", EntryPoint = "Ctr700DrvRegisterInterruptCallback")]
			public static extern Ctr700Result RegisterInterruptCallback(byte channel, IntPtr callback, Ctr700InterruptTrigger interruptTrigger);

			[DllImport("libctr700drv.so", EntryPoint = "Ctr700DrvUnregisterInterruptCallback")]
			public static extern Ctr700Result UnregisterInterruptCallback(byte channel);
		}

		#endregion External native methods
	}

	#region Enums

	/// <summary>
	/// Defines common error codes for all native API functions.
	/// </summary>
	public enum Ctr700Result
	{
		/// <summary>The function call succeeded.</summary>
		Successful = 0x00,
		/// <summary>A generic error occurred.</summary>
		Error = 0xFF,
		/// <summary>The functionality is not implemented by the library.</summary>
		NotImplemented = 0xFE,
		/// <summary>One of the given parameters is invalid (e.g. null or out of range).</summary>
		InvalidParameter = 0xFD,
		/// <summary>The provided channel number is invalid.</summary>
		InvalidChannel = 0xFC,
		/// <summary>The provided mode is invalid.</summary>
		InvalidMode = 0xFB,
		/// <summary>The provided time base is invalid.</summary>
		InvalidTimeBase = 0xFA,
		/// <summary>The provided delta parameter is invalid.</summary>
		InvalidDelta = 0xF9,
		/// <summary>The PTO table is completely filled.</summary>
		PtoParamTabFull = 0xF8,
		/// <summary>Access to the device or peripheral has failed.</summary>
		DevAccessFailed = 0xF7,
		/// <summary>Reserved error code; currently unused.</summary>
		InvalidProcImgCfg = 0xF6,
		/// <summary>Reserved error code; currently unused.</summary>
		ProcImgCfgUnknown = 0xF5,
		/// <summary>Reserved error code; currently unused.</summary>
		ShpImgError = 0xF4,
		/// <summary>Reserved error code; currently unused.</summary>
		AddressOutOfRange = 0xF3,
		/// <summary>The watchdog did timeout.</summary>
		WatchdogTimeout = 0xF2
	}

	/// <summary>
	/// Defines interrupt triggers for digital input channels.
	/// </summary>
	public enum Ctr700InterruptTrigger
	{
		/// <summary>
		/// No events are raised.
		/// </summary>
		None = 0,
		/// <summary>
		/// An event is raised on a rising edge of the digital input channel, i.e. the input value
		/// changes from low to high.
		/// </summary>
		RisingEdge = 1,
		/// <summary>
		/// An event is raised on a falling edge of the digital input channel, i.e. the input value
		/// changes from high to low.
		/// </summary>
		FallingEdge = 2,
		/// <summary>
		/// An event is raised on a rising and falling edge of the digital input channel, i.e. all
		/// changes of the input value.
		/// </summary>
		BothEdges = 3
	}

	/// <summary>
	/// Defines analog channel modes.
	/// </summary>
	public enum Ctr700AdcMode : byte
	{
		/// <summary>
		/// The analog input measures the voltage.
		/// </summary>
		Voltage = 0,
		/// <summary>
		/// The analog input measures the current.
		/// </summary>
		Current = 1
	}

	/// <summary>
	/// Defines counter channel modes.
	/// </summary>
	public enum Ctr700CounterMode : byte
	{
		/// <summary>
		/// The counter will count edges on digital input 14. The direction of counting is
		/// determined by the value of digital input 15.
		/// </summary>
		Counter = 0,
		/// <summary>
		/// The counter will count in A/B decoder mode. Digital input 14 is used for the 'A' input
		/// and digital input 15 is used for 'B'. Switching the inputs will result in inverse
		/// counting.
		/// </summary>
		ABDecoder = 1
	}

	/// <summary>
	/// Defines counter triggers for counter channels.
	/// </summary>
	public enum Ctr700CounterTrigger : byte
	{
		/// <summary>
		/// Rising edges of the counter channel are counted.
		/// </summary>
		RisingEdge = 0,
		/// <summary>
		/// Falling edges of the counter channel are counted.
		/// </summary>
		FallingEdge = 1,
		/// <summary>
		/// Rising and falling edges of the counter channel are counted.
		/// </summary>
		BothEdges = 2
	}

	/// <summary>
	/// Defines counter directions for counter channels.
	/// </summary>
	public enum Ctr700CounterDirection : byte
	{
		/// <summary>
		/// The counter value is incremented.
		/// </summary>
		Up = 0,
		/// <summary>
		/// The counter value is decremented.
		/// </summary>
		Down = 1
	}

	/// <summary>
	/// Defines time bases (periods) for PWM output channels.
	/// </summary>
	public enum Ctr700PwmTimeBase : byte
	{
		/// <summary>
		/// The time base (period) is 800 nanoseconds. This corresponds to a frequency of 1.25 MHz.
		/// </summary>
		TimeBase800ns = 1,
		/// <summary>
		/// The time base (period) is 1 millisecond. This corresponds to a frequency of 1 kHz.
		/// </summary>
		TimeBase1ms = 2
	}

	/// <summary>
	/// Defines temperature sensors.
	/// </summary>
	public enum Ctr700TemperatureSensor : byte
	{
		/// <summary>
		/// The internal temperature sensor of the i.MX7 CPU.
		/// </summary>
		Cpu = 0,
		/// <summary>
		/// The temperature sensor on the system PCB of sysWORXX CTR-700.
		/// </summary>
		System = 1
	}

	#endregion Enums

	#region Support classes

	/// <summary>
	/// The exception that is thrown when a native driver call returns an error value.
	/// </summary>
	public class Ctr700Exception : Exception
	{
		/// <summary>
		/// Initializes a new instance of the <see cref="Ctr700Exception"/> class.
		/// </summary>
		/// <param name="code">The return value of the native driver call.</param>
		public Ctr700Exception(Ctr700Result code)
			: base(GetErrorMessage(code))
		{
			Code = code;
		}

		/// <summary>
		/// Gets the return value of the native driver call.
		/// </summary>
		public Ctr700Result Code { get; }

		private static string GetErrorMessage(Ctr700Result code)
		{
			switch (code)
			{
				case Ctr700Result.NotImplemented: return "The functionality is not implemented by the library.";
				case Ctr700Result.InvalidParameter: return "One of the given parameters is invalid (e.g. null or out of range).";
				case Ctr700Result.InvalidChannel: return "The provided channel number is invalid.";
				case Ctr700Result.InvalidMode: return "The provided mode is invalid.";
				case Ctr700Result.InvalidTimeBase: return "The provided time base is invalid.";
				case Ctr700Result.InvalidDelta: return "The provided delta parameter is invalid.";
				case Ctr700Result.PtoParamTabFull: return "The PTO table is completely filled.";
				case Ctr700Result.DevAccessFailed: return "Access to the device or peripheral has failed.";
				case Ctr700Result.InvalidProcImgCfg: return "Reserved error code; currently unused.";
				case Ctr700Result.ProcImgCfgUnknown: return "Reserved error code; currently unused.";
				case Ctr700Result.ShpImgError: return "Reserved error code; currently unused.";
				case Ctr700Result.AddressOutOfRange: return "Reserved error code; currently unused.";
				case Ctr700Result.WatchdogTimeout: return "The watchdog did timeout.";
				case Ctr700Result.Error:
				default:
					return "A generic error occurred.";
			}
		}
	}

	/// <summary>
	/// Provides data for the <see cref="Ctr700.DigitalInputChanged"/> event.
	/// </summary>
	public class DigitalInputChangedEventArgs : EventArgs
	{
		/// <summary>
		/// Initializes a new instance of the <see cref="DigitalInputChangedEventArgs"/> class.
		/// </summary>
		/// <param name="channel">The digital input channel number.</param>
		/// <param name="state">The new digital input state.</param>
		public DigitalInputChangedEventArgs(byte channel, bool state)
		{
			Channel = channel;
			State = state;
		}

		/// <summary>
		/// Gets the digital input channel number.
		/// </summary>
		public byte Channel { get; }

		/// <summary>
		/// Gets the new digital input state.
		/// </summary>
		public bool State { get; }
	}

	#endregion Support classes
}
