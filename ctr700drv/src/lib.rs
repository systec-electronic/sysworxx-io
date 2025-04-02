// SPDX-License-Identifier: LGPL-3.0-or-later
//
// (c) SYSTEC electronic AG, D-08468 Heinsdorfergrund, Am Windrad 2
//     www.systec-electronic.com

#[macro_use]
extern crate log;

#[macro_use]
extern crate lazy_static;

#[macro_use]
#[path = "../../src/macros.rs"]
pub mod macros;

pub mod ffi;
#[path = "../../Bindings/Rust/sysworxx_io.rs"]
pub mod sysworxx_io;
