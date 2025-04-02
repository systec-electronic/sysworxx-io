// SPDX-License-Identifier: LGPL-3.0-or-later
//
// (c) SYSTEC electronic AG, D-08468 Heinsdorfergrund, Am Windrad 2
//     www.systec-electronic.com

extern crate industrial_io as iio;

use sysworxx_io::io::iio::lookup_id_for_spi;

fn main() {
    let ctx = iio::Context::create_local().unwrap();
    let name = lookup_id_for_spi(0, 0).unwrap();

    let dev = ctx.find_device(&name).unwrap();

    println!("Device: {}", dev.name().unwrap());
    println!("Id:     {}", dev.id().unwrap());

    for c in dev.channels() {
        println!("Channel: {:?}", c.id());
        println!("         {:?}", c.index());
        println!("         {:?}", c.attr_read_int("raw"));
        println!("         {:?}", c.attr_read_float("input"));
    }
}
