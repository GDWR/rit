use std::path::PathBuf;
use std::{fs, io};

pub struct Repositories {
    location: PathBuf,
}

impl Repositories {
    pub fn new(location: PathBuf) -> Self {
        Repositories { location }
    }

    pub fn all_namespaces(&self) -> io::Result<Vec<String>> {
        fs::read_dir(self.location.as_path())?
            .map(|x| x.map(|entry| entry.file_name().into_string().unwrap()))
            .collect()
    }

    pub fn all_projects(&self, namespace: &str) -> io::Result<Vec<String>> {
        fs::read_dir(self.location.as_path().join(namespace))?
            .map(|x| x.map(|entry| entry.file_name().into_string().unwrap()))
            .collect()
    }
}
