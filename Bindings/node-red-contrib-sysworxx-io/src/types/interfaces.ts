// SPDX-License-Identifier: LGPL-3.0-or-later
//
// (c) SYSTEC electronic AG, D-08468 Heinsdorfergrund, Am Windrad 2
//     www.systec-electronic.com

export interface Output {
    [key: string]: string;
};

export interface Input {
    [key: string]: string;
};

export interface SysworxxInterfaces {
    outputs: Output;
    inputs: Input
    analog_inputs: Input;
};

export interface NodeRedMsg {
    payload: any;
    topic: string;
}

export interface NodeConfig {
    name: string;
    channel: string;
    savedOuput: string;
    sampleRate?: any;
    sampleUnit?: any;
    enableModeSetting?: boolean;
    topic: string;
    typeHigh?: string,
    typeLow?: string,
    optAltTopic?: boolean,
    dataHigh?: string | number | boolean,
    dataLow?: string | number | boolean,
    initialState?: string;
}

export enum ADC_STATE {
    ACTIVE = 1,
    IDLE = 0,
    ERROR = -1,
    UNDEF = -2
}

// The *_USER enums have the same value, because for the mode selection,
// we still use the regular mode configurations for each variant.
export enum ADC_MODE {
    VOLTAGE, VOLTAGE_USER = 0,
    CURRENT, CURRENT_USER = 1
}

export enum SAMPLE_UNIT {
    SAMPLE_UNIT_MS = 1,
    SAMPLE_UNIT_S = 1000
}

export enum DECIMAL_PLACES {
    DECIMALPLACES_ALL = -1,
    DECIMALPLACES_0 = 0,
    DECIMALPLACES_1 = 1,
    DECIMALPLACES_2 = 2,
    DECIMALPLACES_3 = 3
}

export enum TMP_MODE {
    TWOWIRE = 0,
    THREEWIRE = 1,
    FOURWIRE = 2
}

export enum TMP_SENSOR_TYPE {
    PT100 = 0,
    PT1000 = 1
}
