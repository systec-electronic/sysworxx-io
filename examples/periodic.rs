// SPDX-License-Identifier: LGPL-3.0-or-later
//
// (c) SYSTEC electronic AG, D-08468 Heinsdorfergrund, Am Windrad 2
//     www.systec-electronic.com

use std::time::{Duration, Instant};
use sysworxx_io::periodic::Periodic;

use rand::prelude::*;

pub fn main() {
    let mut interval = Periodic::new(Duration::from_millis(100));

    let first = Instant::now();

    let mut count = 0;

    let mut rng = rand::thread_rng();
    let mut next_jitter = rng.gen_range(1, 10);
    let mut next_delay = rng.gen_range(1, 300);

    loop {
        interval.next();
        println!("{}: {:?}", count, (Instant::now() - first).as_millis());

        count += 1;
        std::thread::sleep(Duration::from_millis(50));

        if count % next_jitter == 0 {
            std::thread::sleep(Duration::from_millis(next_delay as u64));
            next_jitter = rng.gen_range(1, 10);
            next_delay = rng.gen_range(1, 300);
        }
    }
}
