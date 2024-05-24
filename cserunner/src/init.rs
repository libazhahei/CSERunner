use std::path::PathBuf;
use std::{fs, os};
use std::{borrow::Cow, error::Error, io, rc::Rc};

use homedir::{get_home, get_my_home};
use rand::distributions::DistString;
use serde::{Deserialize, Serialize};
use rand::{distributions::Alphanumeric, Rng};

struct WorkspaceList {
    
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

fn ask_confirm_root_dir(root: String) -> Result<String, Box<dyn Error>> {
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
    Ok(confirmed_root)
}

pub fn init_workspace(config: &Option<String>) -> Result<(), Box<dyn Error>>{
    let config = if let Some(config_file_path) = config {
        let file = fs::File::open("./example_config.json").map_err(errors::FileDoesNotExist::from)?; // expect("config file does not exist"
        let reader = io::BufReader::new(file);
        let mut deser = serde_json::Deserializer::from_reader(reader);
        let config = Config::deserialize(&mut deser).map_err(errors::UnexpectedConfigFile::from)?; // expect("Unexpected data format"
        if config.root.is_none() {
            let root = std::env::current_dir()?.as_os_str().to_str().unwrap_or("").to_string();
            let root = ask_confirm_root_dir(root)?;
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
        let root = ask_confirm_root_dir(root)?;
        let config = default_config();
        let config = Config {
            root: Some(Cow::Owned(root)),
            ..config
        };
        config
    };
    let homedir = get_my_home().expect("Cannot get home directory").unwrap_or(PathBuf::from("~"));
    let cserunner_dir = homedir.join(".cserunner");
    if !cserunner_dir.exists() {
        fs::create_dir(&cserunner_dir)?;
    }
    let name = format!("workspace_{}", Alphanumeric.sample_string(&mut rand::thread_rng(), 16));

    let workspace_dir = cserunner_dir.join(name);
    fs::create_dir(&workspace_dir)?;
    serde_json::to_writer(fs::File::create(workspace_dir.join("config.json"))?, &config)?;
    serde_json::to_writer(fs::File::create(workspace_dir.join("config.json.lock"))?, &config)?;
    println!("Workspace initialized at '{}'", workspace_dir.display());
    println!("You can modify the config file in the workspace directory.");
    Ok(())

}