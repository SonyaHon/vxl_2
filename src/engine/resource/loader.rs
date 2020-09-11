use std::{
    ffi::CStr,
    io::Error,
    path::{Path, PathBuf},
};

pub struct Loader {
    root_path: PathBuf,
}

impl Loader {
    pub fn new() -> Loader {
        let path = std::env::current_exe().unwrap();
        let path_parent = path.parent().unwrap();
        let content_path = path_parent.join("res");

        Loader {
            root_path: content_path,
        }
    }

    pub fn load_file_as_cstr(&self, asset_path: &str) -> Result<&CStr, Error> {
        let mut file = std::fs::File::open(self.root_path.join(asset_path))?;

        let mut buffer: Vec<u8> = Vec::with_capacity(file.metadata()?.len() as usize + 1);

        file.read_to_end(&mut buffer);
    }
}
