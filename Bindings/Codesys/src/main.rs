// SPDX-License-Identifier: LGPL-3.0-or-later
//
// (c) SYSTEC electronic AG, D-08468 Heinsdorfergrund, Am Windrad 2
//     www.systec-electronic.com

use anyhow::Result;
use clap::{arg, command, Command};

mod connector;
mod generate_xml;
use crate::connector::*;
use crate::generate_xml::*;

fn main() -> Result<()> {
    use env_logger::Env;
    let env = Env::new().filter("IO_LOG").write_style("IO_LOG_STYLE");
    env_logger::init_from_env(env);

    let matches = command!()
        .subcommand_required(true)
        .subcommand(
            Command::new("generate-xml")
                .about("Create XML device information for CODESYS")
                .arg(arg!([PATH] "Path to save file location, e.g. /tmp/")),
        )
        .subcommand(
            Command::new("connector")
                .about("Start connector for CODESYS PLC runtime via Unix Domain Socket"),
        )
        .get_matches();

    if let Some(command) = matches.subcommand_matches("generate-xml") {
        if let Some(path) = command.get_one::<String>("PATH") {
            generate_xml(path)?;
        } else {
            println!("Missing path\nTry '--help' for more information.");
        }
    } else if let Some(_) = matches.subcommand_matches("connector") {
        open_plc_connection()?;
    } else {
        println!("Missing command\nTry '--help' for more information.");
    }

    Ok(())
}
