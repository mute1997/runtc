use clap::Clap;

#[derive(Clap, Debug)]
pub struct Opts {
    #[clap(subcommand)]
    pub subcmd: SubCommand,
}

impl ToString for Opts {
    fn to_string(&self) -> String {
        "".to_string()
    }
}

#[derive(Clap, Debug)]
pub enum SubCommand {
    State(State),
    Create(Create),
    Start(Start),
    Kill(Kill),
    Delete(Delete),
}

#[derive(Clap, Debug)]
pub struct State {
    container_id: String,
}

#[derive(Clap, Debug)]
pub struct Create {
    container_id: String,

    #[clap(long, default_value="/")]
    bundle: String,
}

#[derive(Clap, Debug)]
pub struct Start {
    container_id: String,
}

#[derive(Clap, Debug)]
pub struct Kill {
    container_id: String,
    signal: String
}

#[derive(Clap, Debug)]
pub struct Delete {
    container_id: String,
}
