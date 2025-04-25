// SPDX-License-Identifier: LGPL-3.0-or-later
// SPDX-FileCopyrightText: 2025 SYS TEC electronic AG <https://www.systec-electronic.com/>

#[inline(always)]
fn binary_search(offset: usize, slice: &[f64], value: f64) -> usize {
    let index = slice.len() / 2;

    if slice.len() > 1 {
        if slice[index] > value {
            binary_search(offset, &slice[0..index], value)
        } else {
            binary_search(offset + index, &slice[index..], value)
        }
    } else {
        offset
    }
}

#[inline(always)]
fn find_index(haystack: &[f64], resistance: f64) -> (usize, usize) {
    let index_near = binary_search(0, haystack, resistance);

    if haystack[index_near] < resistance {
        (index_near, index_near + 1)
    } else {
        (index_near - 1, index_near)
    }
}

#[inline(always)]
pub fn reverse_lookup(haystack: &[f64], value: f64, start_value: f64, stepsize: f64) -> f64 {
    if value <= haystack[0] {
        std::f64::NEG_INFINITY
    } else if value >= haystack[haystack.len() - 1] {
        std::f64::INFINITY
    } else {
        let (i, j) = find_index(haystack, value);

        let diff_value = haystack[j] - haystack[i];
        let offset_value = value - haystack[i];

        start_value + ((i as f64) * stepsize) + (offset_value * stepsize / diff_value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn binary_search_test() {
        let values = [1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0];
        assert_eq!(0, binary_search(0, &values, 1.00001));
        assert_eq!(1, binary_search(0, &values, 2.00001));
        assert_eq!(2, binary_search(0, &values, 3.00001));
        assert_eq!(3, binary_search(0, &values, 4.00001));
        assert_eq!(4, binary_search(0, &values, 5.00001));
        assert_eq!(5, binary_search(0, &values, 6.00001));
        assert_eq!(6, binary_search(0, &values, 7.00001));
        assert_eq!(7, binary_search(0, &values, 8.00001));
        assert_eq!(8, binary_search(0, &values, 9.00001));
    }

    #[test]
    fn find_index_test() {
        let values = [1.0, 2.0, 3.0];

        assert_eq!((0, 1), find_index(&values, values[0] + 1e-10));
        assert_eq!((0, 1), find_index(&values, values[1] - 1e-10));

        assert_eq!((0, 1), find_index(&values, values[1]));

        assert_eq!((1, 2), find_index(&values, values[1] + 1e-10));
        assert_eq!((1, 2), find_index(&values, values[2] - 1e-10));
    }

    #[test]
    fn reverse_lookup_test() {
        // mapping:
        // x | -2 | -1.5 | -1 | -0.5 | 0 | 0.5 | 1 | 1.5 | 2
        // y | -6 |   -4 | -2 |   -1 | 0 |   1 | 2 | 2.5 | 3

        let values = [-6.0, -4.0, -2.0, -1.0, 0.0, 1.0, 2.0, 2.5, 3.0];
        let start = -2.0;
        let stepsize = 0.5;

        let testcases = vec![
            // (expected_x, y)
            (-2.0, -6.0 + 10e-10),
            (-1.5, -4.0),
            (-1.0, -2.0),
            (-0.5, -1.0),
            (0.0, 0.0),
            (0.5, 1.0),
            (1.0, 2.0),
            (1.5, 2.5),
            (2.0, 3.0 - 10e-10),
        ];

        for (expected_x, y) in testcases {
            dbg!(
                "--------------------",
                (expected_x, y),
                reverse_lookup(&values, y, start, stepsize)
            );
            assert!(approx_eq!(
                f64,
                expected_x,
                reverse_lookup(&values, y, start, stepsize),
                epsilon = 10e-9
            ));
        }
    }
}
