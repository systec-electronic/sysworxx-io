// SPDX-License-Identifier: LGPL-3.0-or-later
// SPDX-FileCopyrightText: 2025 SYS TEC electronic AG <https://www.systec-electronic.com/>

use crate::convert::util;

/// Termocouple lookup table (-50.0°C to 260°C)
const TC_START: f64 = -50.0;
const TC_STEP: f64 = 1.0;
const TC_END: f64 = 260.0;

#[allow(clippy::approx_constant)]
const TC_LUT: [f64; 311] = [
    -1.889, -1.854, -1.818, -1.782, -1.745, -1.709, -1.673, -1.637, -1.600, -1.564, -1.527, -1.490,
    -1.453, -1.417, -1.380, -1.343, -1.305, -1.268, -1.231, -1.194, -1.156, -1.119, -1.081, -1.043,
    -1.006, -0.968, -0.930, -0.892, -0.854, -0.816, -0.778, -0.739, -0.701, -0.663, -0.624, -0.586,
    -0.547, -0.508, -0.470, -0.431, -0.392, -0.353, -0.314, -0.275, -0.236, -0.197, -0.157, -0.118,
    -0.079, -0.039, 0.000, 0.039, 0.079, 0.119, 0.158, 0.198, 0.238, 0.277, 0.317, 0.357, 0.397,
    0.437, 0.477, 0.517, 0.557, 0.597, 0.637, 0.677, 0.718, 0.758, 0.798, 0.838, 0.879, 0.919,
    0.960, 1.000, 1.041, 1.081, 1.122, 1.163, 1.203, 1.244, 1.285, 1.326, 1.366, 1.407, 1.448,
    1.489, 1.530, 1.571, 1.612, 1.653, 1.694, 1.735, 1.776, 1.817, 1.858, 1.899, 1.941, 1.982,
    2.023, 2.064, 2.106, 2.147, 2.188, 2.230, 2.271, 2.312, 2.354, 2.395, 2.436, 2.478, 2.519,
    2.561, 2.602, 2.644, 2.685, 2.727, 2.768, 2.810, 2.851, 2.893, 2.934, 2.976, 3.017, 3.059,
    3.100, 3.142, 3.184, 3.225, 3.267, 3.308, 3.350, 3.391, 3.433, 3.474, 3.516, 3.557, 3.599,
    3.640, 3.682, 3.723, 3.765, 3.806, 3.848, 3.889, 3.931, 3.972, 4.013, 4.055, 4.096, 4.138,
    4.179, 4.220, 4.262, 4.303, 4.344, 4.385, 4.427, 4.468, 4.509, 4.550, 4.591, 4.633, 4.674,
    4.715, 4.756, 4.797, 4.838, 4.879, 4.920, 4.961, 5.002, 5.043, 5.084, 5.124, 5.165, 5.206,
    5.247, 5.288, 5.328, 5.369, 5.410, 5.450, 5.491, 5.532, 5.572, 5.613, 5.653, 5.694, 5.735,
    5.775, 5.815, 5.856, 5.896, 5.937, 5.977, 6.017, 6.058, 6.098, 6.138, 6.179, 6.219, 6.259,
    6.299, 6.339, 6.380, 6.420, 6.460, 6.500, 6.540, 6.580, 6.620, 6.660, 6.701, 6.741, 6.781,
    6.821, 6.861, 6.901, 6.941, 6.981, 7.021, 7.060, 7.100, 7.140, 7.180, 7.220, 7.260, 7.300,
    7.340, 7.380, 7.420, 7.460, 7.500, 7.540, 7.579, 7.619, 7.659, 7.699, 7.739, 7.779, 7.819,
    7.859, 7.899, 7.939, 7.979, 8.019, 8.059, 8.099, 8.138, 8.178, 8.218, 8.258, 8.298, 8.338,
    8.378, 8.418, 8.458, 8.499, 8.539, 8.579, 8.619, 8.659, 8.699, 8.739, 8.779, 8.819, 8.860,
    8.900, 8.940, 8.980, 9.020, 9.061, 9.101, 9.141, 9.181, 9.222, 9.262, 9.302, 9.343, 9.383,
    9.423, 9.464, 9.504, 9.545, 9.585, 9.626, 9.666, 9.707, 9.747, 9.788, 9.828, 9.869, 9.909,
    9.950, 9.991, 10.031, 10.072, 10.113, 10.153, 10.194, 10.235, 10.276, 10.316, 10.357, 10.398,
    10.439, 10.480, 10.520, 10.561,
];

fn mvolts_to_temp(mvolts: f64) -> f64 {
    util::reverse_lookup(&TC_LUT, mvolts, TC_START, TC_STEP)
}

fn temp_to_mvolts(temp: f64) -> f64 {
    if temp <= TC_START {
        std::f64::NEG_INFINITY
    } else if temp >= TC_END {
        std::f64::INFINITY
    } else {
        let float_index1 = (temp - TC_START) / TC_STEP;

        let index1 = float_index1 as usize;
        let index2 = index1 + 1;

        // linear interpolation of the input voltage offset to the temperature offset
        let temp_rounded = ((index1 as f64) * TC_STEP) + TC_START;
        let diff_temp = temp - temp_rounded;
        let step_mvolts = TC_LUT[index2] - TC_LUT[index1];
        let mvolts_offset = diff_temp * step_mvolts / TC_STEP;

        TC_LUT[index1] + mvolts_offset
    }
}

pub fn calc_temperature(ambient_temperature: f64, mvolts: f64) -> f64 {
    let ambient_mvolts = temp_to_mvolts(ambient_temperature);
    let compensated_mvolts = ambient_mvolts + mvolts;
    mvolts_to_temp(compensated_mvolts)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lookup_plausibility_test() {
        let iter1 = TC_LUT.iter();
        let mut iter2 = TC_LUT.iter();
        iter2.next().unwrap();

        for (a, b) in iter1.zip(iter2) {
            assert!(a < b);
        }
    }

    #[test]
    fn mvolts_to_temp_test() {
        assert_eq!(std::f64::NEG_INFINITY, mvolts_to_temp(TC_LUT[0]));

        assert!(approx_eq!(
            f64,
            TC_START,
            mvolts_to_temp(TC_LUT[0] + 1e-10),
            epsilon = 1e-6
        ));

        let testcases = vec![(0.0, 0.0), (4.096, 100.0), (8.138, 200.0)];

        for (mvolts, temperature) in testcases {
            dbg!(mvolts, temperature);
            assert!(approx_eq!(
                f64,
                temperature,
                mvolts_to_temp(mvolts),
                epsilon = 1e-6
            ));
        }

        assert!(approx_eq!(
            f64,
            TC_END,
            mvolts_to_temp(TC_LUT[TC_LUT.len() - 1] - 1e-10),
            epsilon = 1e-6
        ));

        assert_eq!(std::f64::INFINITY, mvolts_to_temp(TC_LUT[TC_LUT.len() - 1]));
    }

    #[test]
    fn mvolts_to_temp_tenths_test() {
        let step_temp = TC_STEP / 10.0;
        let step_mvolts = (TC_LUT[51] - TC_LUT[50]) / 10.0;

        dbg!(step_mvolts, step_temp);

        let testcases = vec![
            (step_mvolts * 0.0, step_temp * 0.0),
            (step_mvolts * 1.0, step_temp * 1.0),
            (step_mvolts * 2.0, step_temp * 2.0),
            (step_mvolts * 3.0, step_temp * 3.0),
            (step_mvolts * 4.0, step_temp * 4.0),
            (step_mvolts * 5.0, step_temp * 5.0),
            (step_mvolts * 6.0, step_temp * 6.0),
            (step_mvolts * 6.0, step_temp * 6.0),
            (step_mvolts * 7.0, step_temp * 7.0),
            (step_mvolts * 8.0, step_temp * 8.0),
            (step_mvolts * 9.0, step_temp * 9.0),
            (step_mvolts * 10.0, step_temp * 10.0),
        ];

        for (mvolts, temperature) in testcases {
            dbg!("---------------------------");
            dbg!(
                mvolts,
                temperature,
                mvolts_to_temp(mvolts),
                mvolts_to_temp(mvolts) - temperature
            );
            assert!(approx_eq!(
                f64,
                temperature,
                mvolts_to_temp(mvolts),
                epsilon = 1e-6
            ));
        }
    }

    #[test]
    fn temp_to_mvolts_test() {
        let testcases = vec![
            (TC_LUT[0], TC_START + 1e-10),
            (0.0, 0.0),
            (TC_LUT[TC_LUT.len() - 1], TC_END - 1e-10),
        ];

        for (expected_mvolts, temperature) in testcases {
            dbg!(
                expected_mvolts,
                temperature,
                expected_mvolts - temp_to_mvolts(temperature)
            );
            assert!(approx_eq!(
                f64,
                expected_mvolts,
                temp_to_mvolts(temperature),
                epsilon = 1e-6
            ));
        }
    }

    #[test]
    fn calc_temperature_test() {
        let ninf = std::f64::NEG_INFINITY;
        let pinf = std::f64::INFINITY;

        let testcases = vec![
            // (expected temperature, measured millivolts, ambient temperature)
            (0.0, 0.0, 0.0),
            (25.0, 0.0, 25.0),
            (ninf, TC_LUT[0] - temp_to_mvolts(-30.0), -30.0),
            (-50.0, TC_LUT[0] - temp_to_mvolts(-30.0) + 1e-10, -30.0),
            (-30.0, 0.0, -30.0),
            (
                260.0,
                TC_LUT[TC_LUT.len() - 1] - temp_to_mvolts(-30.0) - 1e-10,
                -30.0,
            ),
            (
                pinf,
                TC_LUT[TC_LUT.len() - 1] - temp_to_mvolts(-30.0),
                -30.0,
            ),
            (ninf, TC_LUT[0] - temp_to_mvolts(85.0), 85.0),
            (-50.0, TC_LUT[0] - temp_to_mvolts(85.0) + 1e-10, 85.0),
            (85.0, 0.0, 85.0),
            (
                260.0,
                TC_LUT[TC_LUT.len() - 1] - temp_to_mvolts(85.0) - 1e-10,
                85.0,
            ),
            (pinf, TC_LUT[TC_LUT.len() - 1] - temp_to_mvolts(85.0), 85.0),
        ];

        for (expected_temp, mvolts, ambient_temperature) in testcases {
            dbg!("---------------------------------");

            let calculated_temperature = calc_temperature(ambient_temperature, mvolts);

            dbg!(
                expected_temp,
                mvolts,
                ambient_temperature,
                calculated_temperature,
                expected_temp - calculated_temperature
            );
            assert!(approx_eq!(
                f64,
                expected_temp,
                calculated_temperature,
                epsilon = 1e-6
            ));
        }
    }
}
