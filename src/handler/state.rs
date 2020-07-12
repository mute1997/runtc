use std::error::Error;

use crate::{container::Container, args};

pub fn state(args: &args::State) -> Result<(), Box<dyn Error>> {
    let container = Container::find(args.container_id.clone())?;
    container.state();
    Ok(())
}
