extern crate exitcode;

mod args;
mod state;
mod create;
mod start;
mod kill;
mod delete;

use clap::Clap;
use std::process;
use std::env;
use log::{error, info, debug};

use crate::state::state;
use crate::create::create;
use crate::start::start;
use crate::kill::kill;
use crate::delete::delete;

fn main() {
    let log4rs_config = match env::var("LOG4RS_CONFIG") {
        Ok(val) => val,
        Err(_) => "resources/log4rs.yaml".to_string(),
    };

    log4rs::init_file(log4rs_config, Default::default()).unwrap();

    info!("starting runtc.");

    let opts = args::Opts::parse();

    debug!("{:?}", opts);

    let status = match opts.subcmd {
        args::SubCommand::State(s)  => state(s),
        args::SubCommand::Create(c) => create(c),
        args::SubCommand::Start(s)  => start(s),
        args::SubCommand::Kill(k)   => kill(k),
        args::SubCommand::Delete(d) => delete(d)
    };

    match status {
        Err(msg) => {
            error!("{}", msg);
            process::exit(exitcode::SOFTWARE);
        },
        _ => {
            info!("exit runtc successfully.");
        },
    }
}
