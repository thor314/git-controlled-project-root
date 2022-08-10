use std::{env, ffi::OsString, fs::read_dir, path::PathBuf, io::{ErrorKind, self}};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let path = env::current_dir()?;
    let mut path_ancestors = path.as_path().ancestors();
    while let Some(p) = path_ancestors.next() {
        let is_root = read_dir(p)?
            .into_iter()
            .any(|p| p.unwrap().file_name() == OsString::from(".git"));
        if is_root {
            println!("project root: {:?}", PathBuf::from(p));
            return Ok(())
        }
    }
    Err(io::Error::new(ErrorKind::NotFound, "No git repository found").into())
}
