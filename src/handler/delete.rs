use std::error::Error;

use crate::{container::Container, args};

pub fn delete(args: &args::Delete) -> Result<(), Box<dyn Error>> {
    let mut container = Container::find(args.container_id.clone())?;
    container.delete()?;
    Ok(())
}
