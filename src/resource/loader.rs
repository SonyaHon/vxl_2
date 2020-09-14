use std::ffi::CString;
use std::io::{self, Read};
use std::path::PathBuf;
use std::{fs, path::Path};

#[derive(Debug)]
pub enum Error {
    Io(io::Error),
    FileContainsNil,
}

impl From<io::Error> for Error {
    fn from(other: io::Error) -> Self {
        Error::Io(other)
    }
}

pub struct Loader {
    root_path: PathBuf,
}

impl Loader {
    pub fn new() -> Loader {
        let path = std::env::current_exe().unwrap();
        let path_parent = path.parent().unwrap();
        let content_path = path_parent.join("res\\");

        Loader {
            root_path: content_path,
        }
    }

    pub fn load_file_as_cstring(&self, asset_path: &str) -> Result<CString, Error> {
        let mut file = fs::File::open(self.root_path.join(Path::new(asset_path)))?;
        let mut buffer: Vec<u8> = Vec::with_capacity(file.metadata()?.len() as usize + 1);

        file.read_to_end(&mut buffer)?;

        if buffer.iter().find(|i| **i == 0).is_some() {
            return Err(Error::FileContainsNil);
        }

        Ok(unsafe { CString::from_vec_unchecked(buffer) })
    }
}
