use core::time;
use std::collections::HashMap;
use std::fs::File;
use std::path::PathBuf;
use std::thread::sleep;
use std::{fs};
use std::{borrow::Cow, error::Error, io};

use homedir::{get_home, get_my_home};
use rand::distributions::DistString;
use serde::{Deserialize, Serialize};
use rand::distributions::Alphanumeric;

#[derive(Debug, Serialize, Deserialize)]
struct WorkspaceList<'a> {
    workspace_mapping: HashMap<Cow<'a, str>, Cow<'a, str>>,
    workspace_counter: u32,
}

enum WorkSpaceStatus {
    Exist(String),
    New(String),
}

use crate::{errors, AuthConfig, Config, ServerConfig, SyncConfig, SyncType};
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

fn add_workspace_to_list(workspace: &str, workspace_dir: &str) -> Result<(), Box<dyn Error>> {
    let cserunner_dir = to_cserunner_dir()?;
    // if there is a workspace.json.lock file exists, 
    sleep(time::Duration::from_micros(33));
    while (cserunner_dir.join("workspace_list.json.lock").exists()) {
        sleep(time::Duration::from_micros(30));
    }
    File::create(cserunner_dir.join("workspace_list.json.lock")).expect("Lock file creation failed");
    let file = fs::File::open(cserunner_dir.join("workspace_list.json"))?;
    let reader = io::BufReader::new(file);
    let mut deser = serde_json::Deserializer::from_reader(reader);
    let mut list = WorkspaceList::deserialize(&mut deser).map_err(errors::UnexpectedConfigFile::from)?; // expect("Unexpected data format"
    list.workspace_mapping.insert(Cow::Owned(workspace.to_string()), Cow::Owned(workspace_dir.to_string()));
    list.workspace_counter += 1;
    serde_json::to_writer(fs::File::create(cserunner_dir.join("workspace_list.json"))?, &list)?;
    std::fs::remove_file(cserunner_dir.join("workspace_list.json.lock"));
    Ok(())
}


fn ask_confirm_new_root_dir(root: String) -> Result<WorkSpaceStatus, Box<dyn Error>> {
    let mut confirmed_root = root;
    loop {
        println!("The root directory is set to '{}'. Is this correct? [Y/n]", confirmed_root);
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        if input.trim().to_lowercase() == "n" {
            println!("Please enter the correct root directory: ");
            input.clear();
            io::stdin().read_line(&mut input)?;
            confirmed_root = input.trim().to_string();
            if !fs::metadata(&confirmed_root)?.is_dir() {
                println!("Invalid directory. Please enter a valid directory.");
                continue;
            }
        } else if input.trim().to_lowercase() == "y" || input.trim().is_empty() {
            break;
        } else {
            println!("Invalid input. Please enter 'y' or 'n'.");
        }
    }
    Ok(WorkSpaceStatus::New((confirmed_root)))
}

pub fn to_cserunner_dir() -> Result<PathBuf, Box<dyn Error>>{
    let homedir = get_my_home().expect("Cannot get home directory").unwrap_or(PathBuf::from("~"));
    let cserunner_dir = homedir.join(".cserunner");
    if !cserunner_dir.exists() {
        fs::create_dir(&cserunner_dir)?;
        let list = WorkspaceList {
            workspace_mapping: HashMap::new(),
            workspace_counter: 0
        };
        serde_json::to_writer(fs::File::create(cserunner_dir.join("workspace_list.json"))?, &list)?;
    }
    Ok(cserunner_dir)
}


pub fn get_workspace_config(config_file: &File) -> Result<Config, Box<dyn Error>> {
    let cserunner_dir = to_cserunner_dir()?;
    let file = fs::File::open(cserunner_dir.join("workspace_list.json"))?;
    let reader = io::BufReader::new(file);
    let mut deser = serde_json::Deserializer::from_reader(reader);
    let config = Config::deserialize(&mut deser).map_err(errors::UnexpectedConfigFile::from)?; // expect("Unexpected data format"
    Ok(config)
}


fn check_root_dir(root: &str) -> Result<String, Box<dyn Error>> {
    let cserunner_dir = to_cserunner_dir()?;
    let file = fs::File::open(cserunner_dir.join("workspace_list.json"))?;
    let reader = io::BufReader::new(file);
    let mut deser = serde_json::Deserializer::from_reader(reader);
    let list = WorkspaceList::deserialize(&mut deser).map_err(errors::UnexpectedConfigFile::from)?; // expect("Unexpected data format"
    if list.workspace_mapping.contains_key(root) {
        println!("Workspace already exists at '{}', please revisit config at {}", root, list.workspace_mapping.get(root).unwrap());
        return Err(Box::new(errors::WorkspaceAlreadyExists::new(root.to_string())));
    } else {
        ask_confirm_new_root_dir(root.to_string())?;
    }
    todo!()
}

pub fn init_workspace(config: &Option<String>) -> Result<(), Box<dyn Error>> {
    let cserunner_dir = to_cserunner_dir()?;
    let config = if let Some(config_file_path) = config {
        let file = fs::File::open(config_file_path).map_err(errors::FileDoesNotExist::from)?; // expect("config file does not exist"
        let reader = io::BufReader::new(file);
        let mut deser = serde_json::Deserializer::from_reader(reader);
        let config = Config::deserialize(&mut deser).map_err(errors::UnexpectedConfigFile::from)?; // expect("Unexpected data format"
        if config.root.is_none() {
            let root = std::env::current_dir()?.as_os_str().to_str().unwrap_or("").to_string();
            let root = check_root_dir(&root)?;
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
        let root = check_root_dir(&root)?;
        let config = default_config();
        let config = Config {
            root: Some(Cow::Owned(root)),
            ..config
        };
        config
    };
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