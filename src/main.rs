#[macro_use]
mod macros;

mod cmis;
mod config;
mod files;
mod http;
mod repl;
mod util;

#[cfg(test)]
mod unittest;

use clap::{crate_description, crate_name, crate_version};
use std::io::Error;

use crate::{
    config::Config,
    files::{get_config_path, get_history_path},
    repl::Repl,
};

pub struct GlobalState {
    pub config: Config,
}

pub fn main() {
    banner_start();

    if let Err(err) = match Config::load(&get_config_path()) {
        Ok(cfg) => match Repl::new(cfg).run(&get_history_path()) {
            Ok(_) => Ok(()),
            Err(err) => Err(Error::other(err.to_string())),
        },
        Err(err) => Err(err),
    } {
        println!("{err}");
    }

    banner_end();
}

fn banner_start() {
    println!(
        "{} {}\n{}",
        crate_name!(),
        crate_version!(),
        crate_description!()
    );
}

fn banner_end() {
    println!("Exiting...");
}
