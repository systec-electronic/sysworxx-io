// SPDX-License-Identifier: LGPL-3.0-or-later
// SPDX-FileCopyrightText: 2025 SYS TEC electronic AG <https://www.systec-electronic.com/>

/****************************************************************************

  Project:      SYSTEC sysWORXX CTR-700
  Description:  Java bindings for the I/O driver

****************************************************************************/

package com.systec;

import java.util.List;
import java.util.Arrays;

import com.sun.jna.Library;
import com.sun.jna.Native;
import com.sun.jna.Structure;
import com.sun.jna.Callback;
import com.sun.jna.ptr.ByteByReference;
import com.sun.jna.ptr.ShortByReference;
import com.sun.jna.ptr.IntByReference;

public class Ctr700Drv {

	// Public accessible types ---------------------------------------------------

	/**
	 * Interface of a callback function for digital inputs
	 */
	public interface DiCallback extends Callback {
		public void invoke(byte channel, byte value);
	}

	/**
	 * Enumeration to specify the possible conditions
	 * for triggering a registered digital input callback
	 */
	public enum InterruptTrigger {
		RISING_EDGE(1), FALLING_EDGE(2), BOTH_EDGE(3);

		private final int value;

		InterruptTrigger(final int newValue) {
			value = newValue;
		}

		public int getValue() {
			return value;
		}
	}

	/**
	 * Enumeration to specify the possible counter mods
	 */
	public enum CounterMode{
        COUNTER((byte)0), AB_DECODER((byte)1);

        private final byte value;

        CounterMode(final byte newValue) {
            value = newValue;
        }

        public byte getValue() { return value; }
    }

	/**
	 * Enumeration to specify the possible counter direction
	 */
	public enum CounterDirection{
        UP((byte)0), DOWN((byte)1);

        private final byte value;

        CounterDirection(final byte newValue) {
            value = newValue;
        }

        public byte getValue() { return value; }
    }

	/**
	 * Enumeration to specify the possible counter trigger
	 */
	public enum CounterTrigger{
        RISING_EDGE((byte)0), FALLING_EDGE((byte)1), BOTH_EDGE((byte)2);

        private final byte value;

        CounterTrigger(final byte newValue) {
            value = newValue;
        }

        public byte getValue() { return value; }
    }

	/**
	 * Generic Exception class for public functions of the class
	 */
	public static class Ctr700Exception extends RuntimeException {

		private static final long serialVersionUID = 1L;

		Ctr700Exception(String message) {
			super(message);
		}
	}

	/**
	 * This class provides information of the available
	 * hardware features.
	 */
	public class HardwareInfo {
		public short PcbRevision;
		public short DiChannels;
		public short DoChannels;
		public short RelayChannels;
		public short AiChannels;
		public short AoChannels;
		public short CntChannels;
		public short EncChannels;
		public short PwmChannels;
		public short TmpChannels;
	}

	/**
	 * Simple information class for getting self-diagnose signals
	 */
	public class DiagInformation {
		public boolean DigiOutPowerFail;
		public boolean DigiOutDiag;
		public boolean DigiInError;
		public boolean UsbOverCurrent;
	}

	/**
	 * Version information wrapper class
	 */
	public class Version {
		public int major;
		public int minor;
	}

	// Internal types ------------------------------------------------------------

	protected class tCtr700DrvHwInfo extends Structure {
		public short m_uPcbRevision;
		public short m_uDiChannels;
		public short m_uDoChannels;
		public short m_uRelayChannels;
		public short m_uAiChannels;
		public short m_uAoChannels;
		public short m_uCntChannels;
		public short m_uEncChannels;
		public short m_uPwmChannels;
		public short m_uTmpChannels;

		protected List<String> getFieldOrder() {
			return Arrays.asList("m_uPcbRevision", "m_uDiChannels", "m_uDoChannels", "m_uRelayChannels", "m_uAiChannels", "m_uAoChannels",
					"m_uCntChannels", "m_uEncChannels", "m_uPwmChannels", "m_uTmpChannels");
		}
	}

	protected class tCtr700DrvDiagInfo {
		public byte m_fDigiOutPowerFail;
		public byte m_fDigiOutDiag;
		public byte m_fDigiInError;
		public byte m_fUsbOverCurrent;

		protected List<String> getFieldOrder() {
			return Arrays.asList("m_fDigiOutPowerFail", "m_fDigiOutDiag", "m_fDigiInError", "m_fUsbOverCurrent");
		}
	}

	// Interface to the C library ------------------------------------------------

	private interface Ctr700DrvLib extends Library {

		Ctr700DrvLib INSTANCE = (Ctr700DrvLib) Native.load("ctr700drv", Ctr700DrvLib.class);

		int Ctr700DrvInitialize();

		int Ctr700DrvShutDown();

		int Ctr700DrvGetVersion(ByteByReference puMajor, ByteByReference puMinor);

		int Ctr700DrvGetTickCount(IntByReference puTickCount_p);

		int Ctr700DrvEnableWatchdog(byte fMonitorOnly_p);

		int Ctr700DrvServiceWatchdog();

		int Ctr700DrvGetHardwareInfo(tCtr700DrvHwInfo pHwInfo_p);

		int Ctr700DrvSetRunLed(byte fState_p);

		int Ctr700DrvSetErrLed(byte fState_p);

		int Ctr700DrvGetRunSwitch(ByteByReference pfRunSwitch_p);

		int Ctr700DrvGetConfigEnabled(ByteByReference pfConfig_p);

		int Ctr700DrvGetPowerFail(ByteByReference pfFail_p);

		int Ctr700DrvGetDiagInfo(tCtr700DrvDiagInfo pDiagInfo_p);

		int Ctr700DrvGetExtFail(ByteByReference pfFail_p);

		int Ctr700DrvSetExtReset(byte fEnable_p);

		int Ctr700DrvGetDigiIn(byte uChannel_p, ByteByReference fState_p);

		int Ctr700DrvSetDigiOut(byte uChannel_p, byte fEnable_p);

		int Ctr700DrvSetRelay(byte uChannel_p, byte fEnable_p);

		int Ctr700DrvCntEnable(byte uChannel_p, byte fEnable_p);

		int Ctr700DrvCntSetMode(byte uChannel_p, byte uMode_p, byte uTrigger, byte uDir_p);

		int Ctr700DrvCntSetPreload(byte uChannel_p, int iPreload_p);

		int Ctr700DrvCntGetValue(byte uChannel_p, IntByReference piValue_p);

		int Ctr700DrvPwmSetTimeBase(byte uChannel_p, byte uTimeBase_p);

		int Ctr700DrvPwmSetParam(byte uChannel_p, short uPeriod_p, short uPulseLen_p);

		int Ctr700DrvPwmEnable(byte uChannel_p, byte fRun_p);

		int Ctr700DrvAdcGetValue(byte uChannel_p, ShortByReference puAdcValue_p);

		int Ctr700DrvAdcSetMode(byte uChannel_p, byte uMode_p);

		int Ctr700DrvTmpGetValue(byte uSensor_p, IntByReference piValue_p);

		int Ctr700DrvRegisterInterruptCallback(short uChannel_p, DiCallback pfnCallback_p, int uInterruptTrigger_p);

		int Ctr700DrvUnregisterInterruptCallback(byte uChannel_p);

	}

	// Domain logic --------------------------------------------------------------

	private static Ctr700Drv instance = null;
	private static int instanceCount = 0;

	// Singleton contructor
	private Ctr700Drv() { }

	public static Ctr700Drv getInstance() {
		if (instance == null) {
			instance = new Ctr700Drv();
		}

		return instance;
	}

	/**
	 * Initialize the I/O driver
	 */
	public void init() {
		if (instanceCount == 0) {
			int result = Ctr700DrvLib.INSTANCE.Ctr700DrvInitialize();
			checkResult(result);
		}

		instanceCount++;
	}

	/**
	 * De-Initialize the I/O driver
	 */
	public void shutDown() {
		instanceCount--;

		if (instanceCount == 0) {
			int result = Ctr700DrvLib.INSTANCE.Ctr700DrvShutDown();
			checkResult(result);
		}

		if (instanceCount < 0) {
			throw new RuntimeException("Ctr700Drv has not been initialized!");
		}
	}

	/**
	 * Get version of the driver library
	 * @param major	Version major reference, must be != null
	 * @param minor Version major reference, must be != null
	 */
	public Version getVersion() {
		ByteByReference majorVersion = new ByteByReference();
		ByteByReference minorVersion = new ByteByReference();

		int result = Ctr700DrvLib.INSTANCE.Ctr700DrvGetVersion(majorVersion, minorVersion);
		checkResult(result);

		Version version = new Version();
		version.major = (int) majorVersion.getValue();
		version.minor = (int) minorVersion.getValue();
		return version;
	}

	/**
	 * Get the monotonic increasing time in milliseconds
	 */
	public int getTickCount() {
		IntByReference tickCount = new IntByReference();
		int result = Ctr700DrvLib.INSTANCE.Ctr700DrvGetTickCount(tickCount);
		checkResult(result);
		return tickCount.getValue();
	}

	/**
	 * Enable the watchdog
	 */
	public void enableWatchdog(boolean monitorOnly) {
		byte monitorOnlyNumeric = (byte) (monitorOnly ? 1 : 0);
		int result = Ctr700DrvLib.INSTANCE.Ctr700DrvEnableWatchdog(monitorOnlyNumeric);
		checkResult(result);
	}

	/**
	 * Service the watchdog
	 */
	public void serviceWatchdog(boolean monitorOnly) {
		int result = Ctr700DrvLib.INSTANCE.Ctr700DrvServiceWatchdog();
		checkResult(result);
	}

	/**
	 * Get information about the supported I/O hardware of the device
	 */
	public final HardwareInfo getHardwareInfo() {
		tCtr700DrvHwInfo HwInfo = new tCtr700DrvHwInfo();
		int result = Ctr700DrvLib.INSTANCE.Ctr700DrvGetHardwareInfo(HwInfo);
		checkResult(result);

		HardwareInfo Info = new HardwareInfo();
		Info.PcbRevision = HwInfo.m_uPcbRevision;
		Info.DiChannels = HwInfo.m_uDiChannels;
		Info.DoChannels = HwInfo.m_uDoChannels;
		Info.RelayChannels = HwInfo.m_uRelayChannels;
		Info.AiChannels = HwInfo.m_uAiChannels;
		Info.AoChannels = HwInfo.m_uAoChannels;
		Info.CntChannels = HwInfo.m_uCntChannels;
		Info.EncChannels = HwInfo.m_uEncChannels;
		Info.PwmChannels = HwInfo.m_uPwmChannels;
		Info.TmpChannels = HwInfo.m_uTmpChannels;
		return Info;
	}

	/**
	 * Set state of the run LED
	 */
	public void setRunLed(boolean state) {
		byte ctrState = (byte) ((state != false) ? 1 : 0);
		int result = Ctr700DrvLib.INSTANCE.Ctr700DrvSetRunLed(ctrState);
		checkResult(result);
	}

	/**
	 * Set state of the error LED
	 */
	public void setErrLed(boolean state) {
		byte ctrState = (byte) ((state != false) ? 1 : 0);
		int result = Ctr700DrvLib.INSTANCE.Ctr700DrvSetErrLed(ctrState);
		checkResult(result);
	}

	/**
	 * Get the state of the run switch
	 */
	public boolean getRunSwitch() {
		ByteByReference ctrState = new ByteByReference();

		int result = Ctr700DrvLib.INSTANCE.Ctr700DrvGetRunSwitch(ctrState);
		checkResult(result);

		if (ctrState.getValue() != 0) {
			return true;
		} else {
			return false;
		}
	}

	/**
	 * Get the state of the config DIP switch
	 */
	public boolean getConfigEnabled() {
		ByteByReference ctrState = new ByteByReference();
		int result = Ctr700DrvLib.INSTANCE.Ctr700DrvGetConfigEnabled(ctrState);
		checkResult(result);
		return ctrState.getValue() != 0 ? true : false;
	}

	/**
	 * Get the state of the power fail signal
	 */
	public boolean getPowerFail() {
		ByteByReference ctrState = new ByteByReference();
		int result = Ctr700DrvLib.INSTANCE.Ctr700DrvGetPowerFail(ctrState);
		checkResult(result);
		return ctrState.getValue() != 0 ? true : false;
	}

	/**
	 * Get state of self-diagnose signals.
	 */
	public final DiagInformation getDiagInfo() {
		tCtr700DrvDiagInfo ctrDiagInfo = new tCtr700DrvDiagInfo();
		int result = Ctr700DrvLib.INSTANCE.Ctr700DrvGetDiagInfo(ctrDiagInfo);
		checkResult(result);

		DiagInformation diagInfo = new DiagInformation();
		diagInfo.DigiOutPowerFail = ctrDiagInfo.m_fDigiOutPowerFail != 0 ? true : false;
		diagInfo.DigiOutDiag = ctrDiagInfo.m_fDigiOutDiag != 0 ? true : false;
		diagInfo.DigiInError = ctrDiagInfo.m_fDigiInError != 0 ? true : false;
		diagInfo.UsbOverCurrent = ctrDiagInfo.m_fUsbOverCurrent != 0 ? true : false;
		return diagInfo;
	}

	/**
	 * Get state of the external fail signal of the backplane bus
	 */
	public boolean getExtFail() {
		ByteByReference ctrState = new ByteByReference();
		int result = Ctr700DrvLib.INSTANCE.Ctr700DrvGetExtFail(ctrState);
		checkResult(result);
		return ctrState.getValue() != 0 ? true : false;
	}

	/**
	 * Set the reset signal of the backplane bus
	 */
	public void setExtReset(boolean resetState) {
		byte resetValue = (byte) (resetState ? 1 : 0);
		int result = Ctr700DrvLib.INSTANCE.Ctr700DrvSetExtReset(resetValue);
		checkResult(result);
	}

	/**
	 * Get state of a single digital input channel
	 */
	public boolean getDigiIn(int channel) {
		int result;

		ByteByReference ctrValue = new ByteByReference();
		result = Ctr700DrvLib.INSTANCE.Ctr700DrvGetDigiIn((byte) channel, ctrValue);
		checkResult(result);

		return (ctrValue.getValue() != 0) ? true : false;
	}

	/**
	 * Set state of a single digital output channel
	 */
	public void setDigiOut(int channel, boolean value) {
		byte ctrValue;
		int result;

		ctrValue = (byte) (value ? 1 : 0);
		result = Ctr700DrvLib.INSTANCE.Ctr700DrvSetDigiOut((byte) channel, ctrValue);
		checkResult(result);
	}

	/**
	 * Set state of a relay
	 */
	public void setRelay(int channel, boolean enable) {
		byte value = (byte) (enable ? 1 : 0);
		int result = Ctr700DrvLib.INSTANCE.Ctr700DrvSetRelay((byte) channel, value);
		checkResult(result);
	}

	/**
	 * Enable counter channel
	 */
	public void counterEnable(int channel, boolean enable){
		int result = Ctr700DrvLib.INSTANCE.Ctr700DrvCntEnable((byte) channel, (byte) 1);
		checkResult(result);
	}

	/**
	 * Disable counter channel
	 */
	public void counterDisable(int channel, boolean enable){
		int result = Ctr700DrvLib.INSTANCE.Ctr700DrvCntEnable((byte) channel, (byte) 0);
		checkResult(result);
	}

	/**
	 * Set mode of counter
	 */
	public void counterSetMode(int channel, CounterMode mode, CounterTrigger trigger, CounterDirection direction){
		int result = Ctr700DrvLib.INSTANCE.Ctr700DrvCntSetMode((byte) channel, mode.getValue(), trigger.getValue(), direction.getValue());
		checkResult(result);
	}

	/**
	 * Set preload of counter
	 */
	public void counterSetPreload(int channel, int preload){
		int result = Ctr700DrvLib.INSTANCE.Ctr700DrvCntSetPreload((byte) channel, preload);
		checkResult(result);
	}

	/**
	 * Get value of counter
	 */
	public int counterGetValue(int channel){
		IntByReference cntValue = new IntByReference();
		int result = Ctr700DrvLib.INSTANCE.Ctr700DrvCntGetValue((byte)channel, cntValue);
		checkResult(result);
		return cntValue.getValue();
	}

	/**
	 * Enable one of the PWM outputs
	 *
	 * @param channel	The channel to use
	 * @param period	The period of the signal in milliseconds
	 * @param duty		The duty time of the signal in milliseconds
	 */
	public void pwmEnable(int channel, short period, short duty) {
		// Set timebase to ms
		int result = Ctr700DrvLib.INSTANCE.Ctr700DrvPwmSetTimeBase((byte) channel, (byte) 2);
		checkResult(result);

		result = Ctr700DrvLib.INSTANCE.Ctr700DrvPwmSetParam((byte) channel, period, duty);
		checkResult(result);

		result = Ctr700DrvLib.INSTANCE.Ctr700DrvPwmEnable((byte) channel, (byte) 1);
		checkResult(result);
	}

	/**
	 * Disable a PWM channel
	 */
	public void pwmDisable(int channel) {
		int result = Ctr700DrvLib.INSTANCE.Ctr700DrvPwmEnable((byte) channel, (byte) 0);
		checkResult(result);
	}

	/**
	 * Get value of an analog input
	 */
	public short adcGetValue(int channel) {
		ShortByReference ctrValue = new ShortByReference();
		int result = Ctr700DrvLib.INSTANCE.Ctr700DrvAdcGetValue((byte) channel, ctrValue);
		checkResult(result);
		return ctrValue.getValue();
	}

	/**
	 * Setup an ADC for measuring voltage
	 */
	public void setupAdcForVoltage(int channel) {
		int result = Ctr700DrvLib.INSTANCE.Ctr700DrvAdcSetMode((byte) channel, (byte) 0);
		checkResult(result);
	}

	/**
	 * Setup an ADC for measuring current
	 */
	public void setupAdcForCurrent(int channel) {
		int result = Ctr700DrvLib.INSTANCE.Ctr700DrvAdcSetMode((byte) channel, (byte) 1);
		checkResult(result);
	}

	/**
	 * Get temperature of a temperature sensor
	 */
	public int temperatureGet(int channel) {
		IntByReference temperature = new IntByReference();
		int result = Ctr700DrvLib.INSTANCE.Ctr700DrvTmpGetValue((byte) channel, temperature);
		checkResult(result);
		return temperature.getValue();
	}

	/**
	 * Register a callback function for a digital input
	 */
	public void registerInterrupt(int channel, DiCallback callback, InterruptTrigger trigger) {
		int result = Ctr700DrvLib.INSTANCE.Ctr700DrvRegisterInterruptCallback((short) channel, callback,
				trigger.getValue());
		checkResult(result);
	}

	/**
	 * Unregister a callback function for a digital input
	 */
	public void unregisterInterrupt(int channel) {
		int result = Ctr700DrvLib.INSTANCE.Ctr700DrvUnregisterInterruptCallback((byte) channel);
		checkResult(result);
	}

	// Map the error codes to exceptions as needed -------------------------------

	static private void checkResult(int resultCode) {
		if (resultCode != 0) {
			String errorMessage;

			switch (resultCode) {

				case 0xff: {
					errorMessage = "Generic error";
					break;
				}
				case 0xfe: {
					errorMessage = "Not implemented";
					break;
				}
				case 0xfd: {
					errorMessage = "Invalid parameter";
					break;
				}
				case 0xfc: {
					errorMessage = "Invalid channel";
					break;
				}
				case 0xfb: {
					errorMessage = "Invalid mode";
					break;
				}
				case 0xfa: {
					errorMessage = "Invalid timebase";
					break;
				}
				case 0xf9: {
					errorMessage = "Invalid delta";
					break;
				}
				case 0xf8: {
					errorMessage = "PTO tab is is completely filled";
					break;
				}
				case 0xf7: {
					errorMessage = "Access to device failed";
					break;
				}
				case 0xf6: {
					errorMessage = "Process image configuration invalid";
					break;
				}
				case 0xf5: {
					errorMessage = "Process image configuration unknown";
					break;
				}
				case 0xf4: {
					errorMessage = "Shared process image error";
					break;
				}
				case 0xf3: {
					errorMessage = "Address out of range";
					break;
				}
				case 0xf2: {
					errorMessage = "Watchdog timeout";
					break;
				}

				// (The default case should never be reached)
				default: {
					errorMessage = "Unknown error";
					break;
				}
			}

			throw new Ctr700Exception(errorMessage);
		}
	}
}
