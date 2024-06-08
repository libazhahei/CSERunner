use core::time;
use std::collections::HashMap;
use std::fs::{self, File};
use std::path::PathBuf;
use std::thread::sleep;
use std::{borrow::Cow, error::Error, io};
use rand::distributions::DistString;
use serde::{Deserialize, Serialize};
use rand::distributions::Alphanumeric;
use crate::check::{get_root_dir, get_workspace_list_lock_path, get_workspace_list_path};

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkspaceList<'a> {
    workspace_mapping: HashMap<Cow<'a, str>, Cow<'a, str>>,
}

impl WorkspaceList<'_> {
    pub fn new() -> WorkspaceList<'static> {
        WorkspaceList {
            workspace_mapping: HashMap::new(),
        }
    }
}


use crate::{errors, AuthConfig, Config, ServerConfig, SyncConfig, SyncType};
/// Returns a Config struct with default configuration values.

/// This function provides a starting point for the application's configuration.
/// You can customize these defaults to suit your specific needs.
fn default_config() -> Config<'static> {
    Config {
        root: Some(Cow::Borrowed("/")),
        server: ServerConfig {
            host: Cow::Borrowed("localhost"),
            port: 22,
            username: Cow::Borrowed("root")
        },
        auth: AuthConfig {
            identity_file: Cow::Borrowed("~/.ssh/id_rsa"),
            password: Cow::Borrowed("password")
        },
        sync: SyncConfig {
            sync_type: SyncType::Eager,
            frequency: 5,
            early_stop: 20,
            lifetime: 600,
            ignore_binary: true,
            ignore: Cow::Borrowed(&[std::borrow::Cow::Borrowed("*.tmp"), std::borrow::Cow::Borrowed("target")]),
            extra_ignore_file: Cow::Borrowed(&[std::borrow::Cow::Borrowed(".gitignore")]),
            n_threads: -1,
            rm_alart: true,
            timeout: 30,
            max_sync_space_size: 1024,
            max_sync_space_unit: Cow::Borrowed("MB"),
            compress_while_sync: true
        }
    }
}

/// This function is use to add a workspace to the workspace list.
/// It takes the workspace name and the workspace directory as arguments.
/// The workspace name is used as the key in the workspace list, while the workspace directory is the value.
/// The function returns a Result object with an empty tuple as the success value.
/// If an error occurs, the function returns a Box<dyn Error> object.
/// 
/// If there is a lock file in the workspace list directory, the function waits for the lock file to be removed before proceeding.
/// The function then reads the workspace list file and deserializes it into a WorkspaceList object.
/// each time will will wait for 33 microseconds before checking if the lock file is removed
fn add_workspace_to_list(workspace: &str, workspace_dir: &str) -> Result<(), Box<dyn Error>> {
    // Waiting for the lock file is removed (waiting for unlocking) 
    let lock_path = get_workspace_list_lock_path()?;
    sleep(time::Duration::from_micros(33));
    while lock_path.exists() {
        sleep(time::Duration::from_micros(30));
    }
    // Lock the file 
    File::create(lock_path.clone()).expect("Lock file creation failed");

    // Add the workspace and workspace_dir pait into the workspace list file 
    let workspace_file_path = get_workspace_list_path()?;
    let file = fs::File::open(workspace_file_path.clone())?;
    let reader = io::BufReader::new(file);
    let mut deser = serde_json::Deserializer::from_reader(reader);
    let mut list = WorkspaceList::deserialize(&mut deser).map_err(errors::UnexpectedConfigFile::from)?; // expect("Unexpected data format"
    list.workspace_mapping.insert(Cow::Owned(workspace.to_string()), Cow::Owned(workspace_dir.to_string()));
    serde_json::to_writer(fs::File::create(workspace_file_path)?, &list)?;
    
    // Unlock the file 
    std::fs::remove_file(lock_path)?;
    Ok(())
}

/// This function prompts the user to confirm the root directory.
fn ask_confirm_new_root_dir(root: &str) -> Result<bool, Box<dyn Error>> {
    let mut buffer = String::new();
    println!("The root directory is set to '{}'. Is this correct? [Y/N]", root);
    loop {
        io::stdin().read_line(&mut buffer)?;
        match buffer.trim().to_lowercase().as_str() {
            "y" => return Ok(true),
            "" => return Ok(true),
            "n" => return Ok(false),
            _ => {
                println!("Invalid input. Please enter 'y' or 'n'.");
                buffer.clear();
            }
        }
    }
}

/// Returns the workspace configuration even you are in a subdirectory of one of the node.
/// This function returns the workspace configuration.
/// It takes the current path as an argument and returns a Result object with a Config object as the success value.
/// If an error occurs, the function returns a Box<dyn Error> object.
pub fn get_workspace_config(current_path: &PathBuf) -> Result<Config, Box<dyn Error>> {
    let file = fs::File::open(get_workspace_list_path()?)?;
    let reader = io::BufReader::new(file);
    let mut deser: serde_json::Deserializer<serde_json::de::IoRead<io::BufReader<File>>> = serde_json::Deserializer::from_reader(reader);
    let list = WorkspaceList::deserialize(&mut deser).map_err(errors::UnexpectedConfigFile::from)?; // expect("Unexpected data format"

    // use value as a regex to match if the current path is a workspace
    if let Some(workspace) = list.workspace_mapping.iter().find(|(_, value)| current_path.starts_with(value.as_ref())) {
        let workspace_dir = workspace.1.as_ref();
        let file = fs::File::open(get_root_dir()?.join(workspace_dir).join("config.json"))?;
        let reader = io::BufReader::new(file);
        let mut deser = serde_json::Deserializer::from_reader(reader);
        let config = Config::deserialize(&mut deser).map_err(errors::UnexpectedConfigFile::from)?; // expect("Unexpected data format"
        Ok(config)
    } else {
        println!("The current directory is not a workspace directory. Please run cserunner init first");
        Err(Box::new(errors::UnexpectedConfigFile::new("The current directory is not a workspace directory.".to_string())))
    }
} 

/// This function checks if the root directory already exists.
/// It takes the root directory as an argument and returns a Result object with the root directory as the success value.
/// If the root directory already exists, the function returns an error.
/// If the root directory does not exist, the function will check if the root is a vliad diectory. 
/// If the root is a valid directory, prompts the user to confirm the root directory. 
/// Otherwise, return an error. 
fn check_workspace_is_valid(root: &str) -> Result<bool, Box<dyn Error>> {
    let file = fs::File::open(get_workspace_list_path()?)?;
    let reader = io::BufReader::new(file);
    let mut deser = serde_json::Deserializer::from_reader(reader);
    let list = WorkspaceList::deserialize(&mut deser).map_err(errors::UnexpectedConfigFile::from)?; // expect("Unexpected data format"
    if list.workspace_mapping.contains_key(root) {
        println!("Workspace already exists at '{}', please revisit config at {}", root, list.workspace_mapping.get(root).unwrap());
        return Err(Box::new(errors::WorkspaceAlreadyExists::new(root.to_string())));
    } else {
        if !fs::metadata(root)?.is_dir() {
            println!("Invalid directory. Please enter a valid directory.");
            return Err(Box::new(errors::ConfigError::InvalidDirectory(root.to_string())));
        }
        ask_confirm_new_root_dir(root)?;
    }
    Ok(true)
}

/// Initializes a new workspace.
/// This function initializes a new workspace.
/// It takes an optional configuration file path as an argument.
/// If a configuration file path is provided, the function reads the configuration file and uses the configuration settings.
/// If a configuration file path is not provided, the function uses default configuration settings.
/// The function creates a new workspace directory and writes the configuration settings to the workspace directory.
/// The function also adds the workspace to the workspace list.
/// The function prints a success message after the workspace is initialized.
/// If an error occurs, the function returns a Box<dyn Error> object.
pub fn init_workspace(config_path: &Option<String>) -> Result<(), Box<dyn Error>> {
    let config = if let Some(config_file_path) = config_path {
        let file = fs::File::open(config_file_path).map_err(errors::FileDoesNotExist::from)?; // expect("config file does not exist"
        let reader = io::BufReader::new(file);
        let mut deser = serde_json::Deserializer::from_reader(reader);
        let config = Config::deserialize(&mut deser).map_err(errors::UnexpectedConfigFile::from)?; // expect("Unexpected data format"
        if config.root.is_none() {
            let root = std::env::current_dir()?.as_os_str().to_str().unwrap_or("").to_string();
            check_workspace_is_valid(&root)?;
            let config = Config {
                root: Some(Cow::Owned(root)),
                ..config
            };
            config
        } else {
            config
        }
    } else {
        let root = std::env::current_dir()?.as_os_str().to_str().unwrap_or("").to_string();
        check_workspace_is_valid(&root)?;
        let config = default_config();
        let config = Config {
            root: Some(Cow::Owned(root)),
            ..config
        };
        config
    };

    let cserunner_dir = get_root_dir()?;
    let name = format!("workspace_{}", Alphanumeric.sample_string(&mut rand::thread_rng(), 16));
    let workspace_dir = cserunner_dir.join(&name);
    fs::create_dir(&workspace_dir)?;
    serde_json::to_writer(fs::File::create(workspace_dir.join("config.json"))?, &config)?;
    serde_json::to_writer(fs::File::create(workspace_dir.join("config.json.lock"))?, &config)?;
    add_workspace_to_list(&name, config.root.as_ref().unwrap().as_ref())?;
    println!("Workspace initialized at '{}'", workspace_dir.display());
    println!("You can modify the config file in the workspace directory.");
    Ok(())

}