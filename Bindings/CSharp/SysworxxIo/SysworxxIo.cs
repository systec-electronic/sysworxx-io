// SPDX-License-Identifier: LGPL-3.0-or-later
// SPDX-FileCopyrightText: 2025 SYS TEC electronic AG <https://www.systec-electronic.com/>

// ****************************************************************************
//
//   Project:      SYSTEC sysWORXX
//   Description:  C# bindings for I/O driver
//
// ****************************************************************************

using System.Runtime.CompilerServices;
using System.Runtime.InteropServices;

namespace Sysworxx
{

    /// <summary>
    /// Provides access to SYSTEC sysWORXX devices via sysworxx-io.
    /// </summary>
    public class Io : IDisposable
    {
        #region Private data

        /// <summary>
        /// Singleton instance of this class.
        /// </summary>
        private static Io? instance;

        /// <summary>
        /// The cached hardware information.
        /// </summary>
        private IoHwInfo? hardwareInfo;

        /// <summary>
        /// The channels for which an interrupt handler is currently registered.
        /// </summary>
        private readonly HashSet<byte> interruptChannels = new HashSet<byte>();

        #endregion Private data

        #region Events

        /// <summary>
        /// Occurs when the state of a digital input has changed. Call
        /// <see cref="SetDigitalInputEvents"/> to setup events for a channel.
        /// </summary>
        public event EventHandler<DigitalInputChangedEventArgs>? DigitalInputChanged;

        #endregion Events

        #region Constructors

        /// <summary>
        /// Initializes a new instance of the <see cref="Io"/> class and the native I/O driver.
        /// </summary>
        public Io()
        {
            if (instance != null)
                throw new InvalidOperationException("There can only be one driver instance at a time in a process.");

            byte major;
            byte minor;
            unsafe
            {
                ThrowOnError(SysworxxIoSys.IoGetVersion(&major, &minor));
            }
            if (major != 2 || minor < 1)
                throw new NotSupportedException($"CTR-700 driver version {major}.{minor} is not supported.");
            Version = new Version(major, minor);

            ThrowOnError(SysworxxIoSys.IoInit());

            instance = this;
        }

        #endregion Constructors

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
        ~Io()
        {
            Dispose(false);
        }

        private void Dispose(bool disposing)
        {
            if (instance != null)
            {
                if (disposing)
                {
                    // Dispose managed resources
                }

                // Unregister callbacks
                foreach (byte channel in interruptChannels)
                {
                    SysworxxIoSys.IoUnregisterInputCallback(channel);
                }

                // Free unmanaged resources
                SysworxxIoSys.IoShutdown();

                instance = null;
            }
        }

        #endregion IDisposable members and finalizer

        #region Properties

        /// <summary>
        /// Gets the version of the I/O driver.
        /// </summary>
        public Version Version { get; }

        /// <summary>
        /// Gets the number of milliseconds elapsed since the system started.
        /// </summary>
        public uint TickCount
        {
            get
            {
                uint tickCount;
                ThrowIfDisposed();
                unsafe
                {
                    ThrowOnError(SysworxxIoSys.IoGetTickCount(&tickCount));
                }
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
                ThrowOnError(SysworxxIoSys.IoSetRunLed(value.ToInternal()));
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
                ThrowOnError(SysworxxIoSys.IoSetErrLed(value.ToInternal()));
            }
        }

        /// <summary>
        /// Gets a value indicating whether the run switch is in the Run position.
        /// </summary>
        public bool RunSwitch
        {
            get
            {
                IoBool state;
                ThrowIfDisposed();
                unsafe
                {
                    ThrowOnError(SysworxxIoSys.IoGetRunSwitch(&state));
                }
                // The I/O driver returns true for the Stop position.
                // This method inverts that to return true for the positive Run position.
                return state.ToBool();
            }
        }

        /// <summary>
        /// Gets a value indicating whether the config switch (DIP 4) is on.
        /// </summary>
        public bool IsConfigEnabled
        {
            get
            {
                IoBool state;
                ThrowIfDisposed();
                unsafe
                {
                    ThrowOnError(SysworxxIoSys.IoGetConfigEnabled(&state));
                }
                return state.ToBool();
            }
        }

        #endregion Properties

        #region Hardware information properties

        private IoHwInfo HardwareInfo
        {
            get
            {
                if (hardwareInfo == null)
                {
                    ThrowIfDisposed();
                    IoHwInfo info;
                    unsafe
                    {
                        ThrowOnError(SysworxxIoSys.IoGetHardwareInfo(&info));
                    }
                    hardwareInfo = info;
                }

                return (IoHwInfo)hardwareInfo!;
            }
        }


        /// <summary>
        /// Gets the PCB revision number.
        /// </summary>
        public byte PcbRevision
        {
            get
            {
                return HardwareInfo.m_uPcbRevision;
            }
        }

        /// <summary>
        /// Gets the number of digital input channels supported by the hardware.
        /// </summary>
        public ushort DigitalInputChannels
        {
            get
            {
                return HardwareInfo.m_uDiChannels;
            }
        }

        /// <summary>
        /// Gets the number of digital output channels supported by the hardware.
        /// </summary>
        public ushort DigitalOutputChannels
        {
            get
            {
                return HardwareInfo.m_uDoChannels;
            }
        }

        /// <summary>
        /// Gets the number of analog input channels supported by the hardware.
        /// </summary>
        public ushort AnalogInputChannels
        {
            get
            {
                return HardwareInfo.m_uAiChannels;
            }
        }

        /// <summary>
        /// Gets the number of analog output channels supported by the hardware.
        /// </summary>
        public ushort AnalogOutputChannels
        {
            get
            {
                return HardwareInfo.m_uAoChannels;
            }
        }

        /// <summary>
        /// Gets the number of counter input channels supported by the hardware.
        /// </summary>
        public ushort CounterChannels
        {
            get
            {
                return HardwareInfo.m_uCntChannels;
            }
        }

        /// <summary>
        /// Gets the number of A/B decoder channels supported by the hardware.
        /// </summary>
        public ushort EncoderChannels
        {
            get
            {
                return HardwareInfo.m_uEncChannels;
            }
        }

        /// <summary>
        /// Gets the number of PWM output channels supported by the hardware.
        /// </summary>
        public ushort PwmChannels
        {
            get
            {
                return HardwareInfo.m_uPwmChannels;
            }
        }

        /// <summary>
        /// Gets the number of temperature sensors supported by the hardware.
        /// </summary>
        public ushort TemperatureSensors
        {
            get
            {
                return HardwareInfo.m_uTmpChannels;
            }
        }

        #endregion Hardware information properties

        #region Public methods

        /// <summary>
        /// Enables the system watchdog. If the watchdog was not serviced in time by calling the
        /// <see cref="ServiceWatchdog"/> method, that method will throw an exception with the code
        /// <see cref="IoResult.WatchdogTimeout"/>. The watchdog timeout is 1000 ms in
        /// non-monitor mode and 900 ms in monitor mode.
        /// </summary>
        /// <param name="monitorOnly">true to start the watchdog in monitor mode; false for real hardware mode.</param>
        public void EnableWatchdog(bool monitorOnly)
        {
            ThrowIfDisposed();
            ThrowOnError(SysworxxIoSys.IoEnableWatchdog(monitorOnly.ToInternal()));
        }

        /// <summary>
        /// Services the watchdog. See <see cref="EnableWatchdog"/> for details.
        /// </summary>
        public void ServiceWatchdog()
        {
            ThrowIfDisposed();
            ThrowOnError(SysworxxIoSys.IoServiceWatchdog());
        }

        /// <summary>
        /// Gets the current state of a digital input channel.
        /// </summary>
        /// <param name="channel">The channel number.</param>
        /// <returns>true if the input is on; otherwise, false.</returns>
        public bool GetDigitalInput(byte channel)
        {
            ThrowIfDisposed();
            IoBool state;
            unsafe
            {
                ThrowOnError(SysworxxIoSys.IoGetInput(channel, &state));
            }
            return state.ToBool();
        }

        /// <summary>
        /// Enables events for a digital input channel.
        /// </summary>
        /// <param name="channel">The channel number.</param>
        /// <param name="trigger">The trigger to raise the event.</param>
        public void SetDigitalInputEvents(byte channel, InputTrigger trigger)
        {
            ThrowIfDisposed();
            if (!interruptChannels.Contains(channel))
            {
                unsafe {
                    ThrowOnError(SysworxxIoSys.IoRegisterInputCallback(channel, &InputHandler, trigger.ToInternal()));
                }
                interruptChannels.Add(channel);
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
            ThrowOnError(SysworxxIoSys.IoSetOutput(channel, state.ToInternal()));
        }

        /// <summary>
        /// Enables or disables a counter input channel.
        /// </summary>
        /// <param name="channel">The channel number.</param>
        /// <param name="state">true to enable the counter, false to disable it.</param>
        public void EnableCounter(byte channel, bool state)
        {
            ThrowIfDisposed();
            ThrowOnError(SysworxxIoSys.IoCntEnable(channel, state.ToInternal()));
        }

        /// <summary>
        /// Sets the mode of a counter input channel.
        /// </summary>
        /// <param name="channel">The channel number.</param>
        /// <param name="mode">The counter channel mode.</param>
        /// <param name="trigger">The counter trigger.</param>
        /// <param name="direction">The counter direction.</param>
        public void SetCounterMode(byte channel, CounterMode mode, CounterTrigger trigger, CounterDirection direction)
        {
            ThrowIfDisposed();
            ThrowOnError(SysworxxIoSys.IoCntSetup(channel, mode.ToInternal(), trigger.ToInternal(), direction.ToInternal()));
        }

        /// <summary>
        /// Sets the initial value of a counter.
        /// </summary>
        /// <param name="channel">The channel number.</param>
        /// <param name="initialValue">The initial value to set.</param>
        public void SetCounterInitialValue(byte channel, int initialValue)
        {
            ThrowIfDisposed();
            ThrowOnError(SysworxxIoSys.IoCntSetPreload(channel, initialValue));
        }

        /// <summary>
        /// Gets the current value of a counter.
        /// </summary>
        /// <param name="channel">The channel number.</param>
        /// <returns>The current value of the counter.</returns>
        public int GetCounterValue(byte channel)
        {
            ThrowIfDisposed();
            int value;
            unsafe {
                ThrowOnError(SysworxxIoSys.IoCntGetValue(channel, &value));
            }
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
            ThrowOnError(SysworxxIoSys.IoPwmEnable(channel, state));
        }

        /// <summary>
        /// Sets the time base of a PWM output channel.
        /// </summary>
        /// <param name="channel">The channel number.</param>
        /// <param name="timeBase">The time base.</param>
        public void SetPwmTimeBase(byte channel, PwmTimebase timeBase)
        {
            ThrowIfDisposed();
            ThrowOnError(SysworxxIoSys.IoPwmSetTimebase(channel, timeBase.ToInternal()));
        }

        /// <summary>
        /// Sets the parameters of a PWM output channel. The new parameters are only applied after
        /// calling <see cref="EnablePwm"/> (again).
        /// </summary>
        /// <param name="channel">The channel number.</param>
        /// <param name="period">The period length in units set by <see cref="PwmTimebase"/>.</param>
        /// <param name="pulseLen">The pulse length of the signal ("on" time / duty cycle).</param>
        public void SetPwmParam(byte channel, ushort period, ushort pulseLen)
        {
            ThrowIfDisposed();
            ThrowOnError(SysworxxIoSys.IoPwmSetup(channel, period, pulseLen));
        }

        /// <summary>
        /// Gets the current value of an analog input channel.
        /// </summary>
        /// <param name="channel">The channel number.</param>
        /// <returns>The raw value.</returns>
        public ushort GetAnalogInput(byte channel)
        {
            ThrowIfDisposed();
            ushort value;
            unsafe {
                ThrowOnError(SysworxxIoSys.IoAdcGetValue(channel, &value));
            }
            return value;
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
        public void SetAnalogMode(byte channel, AnalogMode mode)
        {
            ThrowIfDisposed();
            ThrowOnError(SysworxxIoSys.IoAdcSetMode(channel, mode.ToInternal()));
        }

        /// <summary>
        /// Gets the current value of a temperature sensor.
        /// </summary>
        /// <param name="channel">The temperature sensor channel.</param>
        public float GetTemperature(byte channel)
        {
            ThrowIfDisposed();
            int value;
            unsafe {
                ThrowOnError(SysworxxIoSys.IoTmpGetValue(channel, &value));
            }
            return value / 10000f;
        }

        #endregion Public methods

        #region Private methods

        private void ThrowIfDisposed()
        {
            if (instance == null)
            {
                throw new ObjectDisposedException(nameof(Io));
            }
        }

        private void ThrowOnError(IoResult result)
        {
            if (result != IoResult.Success)
            {
                throw new IoException(result);
            }
        }

        // [UnmanagedFunctionPointer(CallingConvention.Cdecl)]
        [UnmanagedCallersOnly(CallConvs = new[] { typeof(CallConvCdecl) })]
        static private void InputHandler(byte channel, byte state)
        {
            if (instance != null) {
                var args = new DigitalInputChangedEventArgs(channel, (state != 0));
                instance.DigitalInputChanged?.Invoke(instance, args);
            }
        }

        #endregion Private methods
    }

    #region Helpers

    /// <summary>
    /// The exception that is thrown when a native driver call returns an error value.
    /// </summary>
    public class IoException : Exception
    {
        /// <summary>
        /// Initializes a new instance of the <see cref="IoException"/> class.
        /// </summary>
        /// <param name="code">The return value of the native driver call.</param>
        internal IoException(IoResult code)
            : base(GetErrorMessage(code))
        {
            Code = (uint) code;
        }

        /// <summary>
        /// Gets the return value of the native driver call.
        /// </summary>
        public uint Code { get; }

        private static string GetErrorMessage(IoResult code)
        {
            switch (code)
            {
                case IoResult.NotImplemented: return "The functionality is not implemented by the library.";
                case IoResult.InvalidParameter: return "One of the given parameters is invalid (e.g. null or out of range).";
                case IoResult.InvalidChannel: return "The provided channel number is invalid.";
                case IoResult.InvalidMode: return "The provided mode is invalid.";
                case IoResult.InvalidTimebase: return "The provided time base is invalid.";
                case IoResult.InvalidDelta: return "The provided delta parameter is invalid.";
                case IoResult.PtoParamTabFull: return "The PTO table is completely filled.";
                case IoResult.DevAccessFailed: return "Access to the device or peripheral has failed.";
                case IoResult.Reserved0: return "Reserved error code; currently unused.";
                case IoResult.Reserved1: return "Reserved error code; currently unused.";
                case IoResult.ShpImgError: return "Reserved error code; currently unused.";
                case IoResult.AddressOutOfRange: return "Reserved error code; currently unused.";
                case IoResult.WatchdogTimeout: return "The watchdog did timeout.";
                case IoResult.Error:
                default:
                    return "A generic error occurred.";
            }
        }
    }

    /// <summary>
    /// Provides data for the <see cref="Io.DigitalInputChanged"/> event.
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

    public enum CounterMode
    {
        Counter,
        ABEncoder,
    }

    public enum CounterTrigger
    {
        RisingEdge,
        FallingEdge,
        AnyEdge,
    }

    public enum CounterDirection
    {
        Up,
        Down,
    }

    public enum PwmTimebase
    {
        Ns800,
        Ms1,
    }

    public enum AnalogMode
    {
        Voltage,
        Current,
    }

    public enum InputTrigger {
        None,
        RisingEdge,
        FallingEdge,
        BothEdge,
    }

    internal static class SysworxxIoSysConversionsExt
    {
        #region primitive to SysworxxIoSys

        internal static bool ToBool(this IoBool value)
        {
            if (value == IoBool.True)
            {
                return true;
            }
            else
            {
                return false;
            }
        }

        #endregion primitive to SysworxxIoSys

        #region SysworxxIoSys to primitive

        internal static IoBool ToInternal(this bool value)
        {
            switch (value)
            {
                case true: return IoBool.True;
                case false: return IoBool.False;
            }
        }

        internal static IoCntMode ToInternal(this CounterMode value)
        {
            switch (value)
            {
                case CounterMode.Counter: return IoCntMode.Counter;
                case CounterMode.ABEncoder: return IoCntMode.ABEncoder;
                default: throw new ArgumentOutOfRangeException();
            }
        }

        internal static IoCntTrigger ToInternal(this CounterTrigger value)
        {
            switch (value)
            {
                case CounterTrigger.RisingEdge: return IoCntTrigger.RisingEdge;
                case CounterTrigger.FallingEdge: return IoCntTrigger.FallingEdge;
                case CounterTrigger.AnyEdge: return IoCntTrigger.AnyEdge;
                default: throw new ArgumentOutOfRangeException();
            }
        }

        internal static IoCntDirection ToInternal(this CounterDirection value)
        {
            switch (value)
            {
                case CounterDirection.Up: return IoCntDirection.Up;
                case CounterDirection.Down: return IoCntDirection.Down;
                default: throw new ArgumentOutOfRangeException();
            }
        }

        internal static IoPwmTimebase ToInternal(this PwmTimebase value)
        {
            switch (value)
            {
                case PwmTimebase.Ns800: return IoPwmTimebase.Ns800;
                case PwmTimebase.Ms1 : return IoPwmTimebase.Ms1;
                default: throw new ArgumentOutOfRangeException();
            }
        }

        internal static IoAnalogMode ToInternal(this AnalogMode value)
        {
            switch (value)
            {
                case AnalogMode.Voltage: return IoAnalogMode.Voltage;
                case AnalogMode.Current : return IoAnalogMode.Current;
                default: throw new ArgumentOutOfRangeException();
            }
        }

        internal static IoInputTrigger ToInternal(this InputTrigger value)
        {
            switch (value)
            {
                case InputTrigger.None: return IoInputTrigger.None;
                case InputTrigger.RisingEdge: return IoInputTrigger.RisingEdge;
                case InputTrigger.FallingEdge: return IoInputTrigger.FallingEdge;
                case InputTrigger.BothEdge: return IoInputTrigger.BothEdge;
                default: throw new ArgumentOutOfRangeException();
            }
        }

        #endregion SysworxxIoSys to primitive
    }


    #endregion Helpers
}
