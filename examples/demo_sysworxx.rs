// SPDX-License-Identifier: LGPL-3.0-or-later
//
// (c) SYSTEC electronic AG, D-08468 Heinsdorfergrund, Am Windrad 2
//     www.systec-electronic.com

use sysworxx_io::{definition::load_device_definition, hw_rev::get_device_name};

#[allow(unused_must_use)]
fn main() {
    let mut io = load_device_definition(&get_device_name().expect("definition"));
    io.init().expect("init");
    io.write_json_info("/tmp/io.json").expect("io.json");

    for i in 0..8 {
        dbg!(&i, io.output_set(i, true));
    }

    for i in 0..8 {
        dbg!(&i, io.input_get(i));
    }

    io.shutdown().expect("shutdown");
}
