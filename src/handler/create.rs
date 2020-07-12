use std::{io::ErrorKind, io::Error as IOError, error::Error};

use crate::{container::Container, args};

pub fn create(args: &args::Create) -> Result<(), Box<dyn Error>> {
    if Container::find(args.container_id.clone()).is_ok() {
        return Err(Box::new(IOError::new(ErrorKind::Other, "Container is already exists.")));
    }

    let mut container = Container::new(args.container_id.clone(), args.bundle.clone())?;
    container.create()?;
    Ok(())
}
