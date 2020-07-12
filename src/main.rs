#![feature(backtrace)]
extern crate exitcode;

mod args;
mod utils;
mod config;
mod handler;
mod container;

use clap::Clap;
use std::process;
use std::env;

use handler::state::state;
use handler::create::create;
use handler::start::start;
use handler::kill::kill;
use handler::delete::delete;

fn main() {
    let log4rs_config = match env::var("LOG4RS_CONFIG") {
        Ok(val) => val,
        Err(_) => "resources/log4rs.yaml".to_string(),
    };

    match log4rs::init_file(log4rs_config, Default::default()) {
        Err(e) => {
            log::error!("{}", e);
            return
        },
        _ => {},
    }

    let opts = args::Opts::parse();

    log::info!("starting runtc. options: {:?}", opts);

    let status = match &opts.subcmd {
        args::SubCommand::State(s)  => state(s),
        args::SubCommand::Create(c) => create(c),
        args::SubCommand::Start(s)  => start(s),
        args::SubCommand::Kill(k)   => kill(k),
        args::SubCommand::Delete(d) => delete(d)
    };

    match status {
        Err(e) => {
            log::error!("subcmd: {}, msg: {}", opts.subcmd, e);
            process::exit(exitcode::SOFTWARE);
        },
        _ => {
            log::info!("exit runtc successfully.");
        },
    }
}
