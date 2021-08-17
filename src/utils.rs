//! general utilities

use std::{
    env,
    fs::read_dir,
    io::{self, ErrorKind},
    path::PathBuf,
};

/// Returns a path relative to the root of the project.
pub(crate) fn root_path(relative: &str) -> String {
    let mut path = get_project_root().expect("get_project_root()");
    path.push(relative);
    path.to_str().expect("path.to_str()").to_owned()
}

/// Get the project root
pub(crate) fn get_project_root() -> io::Result<PathBuf> {
    let path = env::current_dir()?;
    let path_ancestors = path.as_path().ancestors();

    for p in path_ancestors {
        let has_cargo = read_dir(p)?
            .into_iter()
            .any(|p| p.unwrap().file_name() == "Cargo.toml");
        if has_cargo {
            return Ok(PathBuf::from(p));
        }
    }
    Err(io::Error::new(
        ErrorKind::NotFound,
        "Ran out of places to find Cargo.toml",
    ))
}
