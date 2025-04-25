// SPDX-License-Identifier: LGPL-3.0-or-later
// SPDX-FileCopyrightText: 2025 SYS TEC electronic AG <https://www.systec-electronic.com/>

use std::thread;

use libc::c_int;

pub use signal_hook::{
    SIGABRT, SIGALRM, SIGBUS, SIGCHLD, SIGCONT, SIGFPE, SIGHUP, SIGILL, SIGINT, SIGIO, SIGKILL,
    SIGPIPE, SIGPROF, SIGQUIT, SIGSEGV, SIGSTOP, SIGSYS, SIGTERM, SIGTRAP, SIGUSR1, SIGUSR2,
    SIGWINCH,
};

pub fn notify(signals: &[c_int]) -> Result<crossbeam_channel::Receiver<c_int>, std::io::Error> {
    let (s, r) = crossbeam_channel::bounded(10);
    let signals = signal_hook::iterator::Signals::new(signals)?;

    thread::spawn(move || {
        for signal in signals.forever() {
            if s.send(signal).is_err() {
                break;
            }
        }
    });

    Ok(r)
}
