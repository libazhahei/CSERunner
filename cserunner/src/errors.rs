
use std::{error::Error, fmt::Display};

// pub trait  {
    
// }
#[derive(Debug)]
pub struct FileDoesNotExist {
    file: String
}
impl FileDoesNotExist {
    pub fn new(file: String) -> Self {
        Self {
            file
        }
    }
}
impl From<std::io::Error> for FileDoesNotExist {
    fn from(e: std::io::Error) -> Self {
        Self {
            file: e.to_string()
        }
    }
    
}
impl Display for FileDoesNotExist {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Error: file '{}' does not exist.", self.file)
    }
}
impl Error for FileDoesNotExist {

}

#[derive(Debug)]
pub struct UnexpectedConfigFile {
    error: String
}

impl UnexpectedConfigFile {
    pub fn new(error: String) -> Self {
        Self {
            error
        }
    }
}

impl Display for UnexpectedConfigFile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Error: unexpected config file format. {}", self.error)
    }
}

impl From<serde_json::Error> for UnexpectedConfigFile {
    fn from(e: serde_json::Error) -> Self {
        Self {
            error: e.to_string()
        }
    }
    
}

impl Error for UnexpectedConfigFile {

}


#[derive(Debug)]
pub struct WorkspaceAlreadyExists {
    root: String
}
impl WorkspaceAlreadyExists {
    pub fn new(root: String) -> Self {
        Self {
            root
        }
    }
    
}
impl Display for WorkspaceAlreadyExists {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Error: the directory at {} already exists. You are not allow to init agagin", self.root)
    }
}
impl Error for WorkspaceAlreadyExists  {
    
}

#[derive(Debug)]
pub enum ConfigError {
    InvalidDirectory(String)
}

impl Display for ConfigError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ConfigError::InvalidDirectory(dir) => write!(f, "Error: diectory: \"{}\" is not a valid path.", dir),
        }
    }
}

impl Error for ConfigError {}