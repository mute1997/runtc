use std::error::Error;

use crate::{container::Container, args};

pub fn kill(args: &args::Kill) -> Result<(), Box<dyn Error>> {
    let mut container = Container::find(args.container_id.clone())?;
    container.kill()?;
    Ok(())
}
