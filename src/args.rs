use clap::Clap;

#[derive(Clap, Debug)]
pub struct Opts {
    #[clap(subcommand)]
    pub subcmd: SubCommand,
}

#[derive(Clap, Debug)]
pub enum SubCommand {
    State(State),
    Create(Create),
    Start(Start),
    Kill(Kill),
    Delete(Delete),
}

impl std::fmt::Display for SubCommand {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
       match *self {
           SubCommand::State(_) => write!(f, "state"),
           SubCommand::Create(_) => write!(f, "create"),
           SubCommand::Start(_) => write!(f, "start"),
           SubCommand::Kill(_) => write!(f, "kill"),
           SubCommand::Delete(_) => write!(f, "delete"),
       }
    }
}

#[derive(Clap, Debug)]
pub struct State {
    pub container_id: String,
}

#[derive(Clap, Debug)]
pub struct Create {
    pub container_id: String,

    #[clap(long, default_value=".")]
    pub bundle: String,

    #[clap(long, default_value="-1")]
    pub pid_file: i64,
}

#[derive(Clap, Debug)]
pub struct Start {
    pub container_id: String,
}

#[derive(Clap, Debug)]
pub struct Kill {
    pub container_id: String,
    pub signal: String
}

#[derive(Clap, Debug)]
pub struct Delete {
    pub container_id: String,
}
