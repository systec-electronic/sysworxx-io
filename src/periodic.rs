// SPDX-License-Identifier: LGPL-3.0-or-later
//
// (c) SYSTEC electronic AG, D-08468 Heinsdorfergrund, Am Windrad 2
//     www.systec-electronic.com

use std::thread;
use std::time::{Duration, Instant};

pub struct Periodic {
    interval: Duration,
    last: Option<Instant>,
}

impl Periodic {
    pub fn new(interval: Duration) -> Periodic {
        Periodic {
            interval,
            last: None,
        }
    }

    pub fn next(&mut self) {
        let now = Instant::now();

        match self.last {
            None => {
                // first execution runs immediately
                self.last = Some(now);
            }
            Some(last) => {
                let remaining = self.calc_delay(now, last);
                thread::sleep(remaining);
            }
        }
    }

    #[inline(always)]
    fn calc_delay(&mut self, now: Instant, last: Instant) -> Duration {
        let mut diff = now - last;
        let mut last = last;

        while diff > self.interval {
            error!(
                "missed interval (thread: {:?}, {:?})",
                thread::current().name(),
                diff
            );
            last += self.interval;
            diff = now - last;
        }

        self.last = Some(last + self.interval);
        self.interval - diff
    }

    pub fn elapsed(&self) -> Duration {
        Instant::now() - self.last.unwrap_or_else(Instant::now)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn calc_delay_immediately_test() {
        let now = Instant::now();
        let mut i = Periodic::new(Duration::from_millis(100));
        i.last = Some(now);
        assert_eq!(i.calc_delay(now, now), Duration::from_millis(100));
    }

    #[test]
    fn calc_delay_on_exact_interval_test() {
        let now = Instant::now();
        let t = now + Duration::from_millis(100);
        let mut i = Periodic::new(Duration::from_millis(100));
        i.last = Some(now);
        assert_eq!(i.calc_delay(t, now), Duration::from_millis(0));
    }

    #[test]
    fn calc_delay_in_time_test() {
        let now = Instant::now();
        let t = now + Duration::from_millis(50);
        let mut i = Periodic::new(Duration::from_millis(100));
        i.last = Some(now);
        assert_eq!(i.calc_delay(t, now), Duration::from_millis(50));
    }

    #[test]
    fn calc_delay_missed_one_interval_test() {
        let now = Instant::now();
        let t = now + Duration::from_millis(130);
        let mut i = Periodic::new(Duration::from_millis(100));
        i.last = Some(now);
        assert_eq!(i.calc_delay(t, now), Duration::from_millis(70));
    }
}
