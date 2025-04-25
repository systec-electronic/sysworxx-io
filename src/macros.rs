// SPDX-License-Identifier: LGPL-3.0-or-later
// SPDX-FileCopyrightText: 2025 SYS TEC electronic AG <https://www.systec-electronic.com/>

/// Checks a single pointer, if it is null. If so the second expression will be evaluated and
/// returned.
macro_rules! check_ptr {
    ( $i:expr, $e:expr ) => {{
        if $i.is_null() {
            debug!("Error: invalid pointer!");
            return $e;
        }
    }};
}

/// Evaluate the given expression and convert its `Result<T>` to `IoResult`.
/// The first argument specifies the identifier name of the `Io` instance used in the (second)
/// expression argument.
#[allow(unused_macros)]
macro_rules! io_do {
    ( $i:ident, $e:expr ) => {{
        let $i = INSTANCE.lock();
        match $i {
            Ok(mut $i) => {
                let res = $e;

                match &res {
                    Ok(_) => {}
                    Err(e) => {
                        debug!("Error: {}", e);
                    }
                }

                IoResult::from(res)
            }
            Err(_) => IoResult::Error,
        }
    }};
}

#[allow(unused_macros)]
macro_rules! channels_init {
    ( $v:expr ) => {{
        $v.iter_mut()
            .enumerate()
            .map(|(i, c)| c.init(i))
            .collect::<Result<Vec<()>>>()?;
    }};
}

#[allow(unused_macros)]
macro_rules! channels_shutdown {
    ( $v:expr ) => {{
        $v.iter_mut()
            .map(|c| c.shutdown())
            .collect::<Result<Vec<()>>>()?;
    }};
}

macro_rules! catch_unwind {
    ( $b:block ) => {{
        use std::panic;

        let res = panic::catch_unwind(|| $b);

        match res {
            Ok(v) => v,
            Err(_) => IoResult::Error,
        }
    }};
}
