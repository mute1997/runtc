use std::error::Error;

use crate::{container::Container, args};

pub fn start(args: &args::Start) -> Result<(), Box<dyn Error>> {
    let mut container = Container::find(args.container_id.clone())?;
    container.start()?;
    Ok(())
}
