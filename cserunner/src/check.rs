use std::{error::Error, fs, path::PathBuf};
use homedir::get_my_home;
use crate::init::WorkspaceList;


/// Check the existance of "./cserunner" folder in the root directory. 
/// If the directory does not exist, the function will try to create the folder. 
/// If the function failed to create the folder, an Error is retuned. 
pub fn root_dir_check() -> Result<(), Box<dyn Error>> {
    let cserunner_dir = get_root_dir()?;
    if !cserunner_dir.exists() {
        println!("Root directory not found.");
        fs::create_dir(&cserunner_dir)?;
        println!("Created a root directory for cserunner in location: {:?}", &cserunner_dir);
    } else {
        println!("Found directory in location: {:?}", &cserunner_dir);
    }
    Ok(())
}

/// Check the existance of "./cserunner/workspace_list.json". 
/// If the file does not exist, the function will try to create the folder. 
/// If the function failed to create the file, an Error is retuned. 
pub fn config_file_check() -> Result<(), Box<dyn Error>> {
    let workspace_list_path = get_workspace_list_path()?;
    if !workspace_list_path.exists() {
        println!("Config file not found.");
        let list = WorkspaceList::new();
        // Create a workspace_list.json file. 
        serde_json::to_writer(fs::File::create(workspace_list_path.clone())?, &list)?;
        // println!("Created a configuration file for cserunner in location: {:?}", &workspace_list_path);
    } else {
        // println!("Configuration file in location: {:?}", &workspace_list_path);
    }
    Ok(())
}

/// Get the root directory of the .cserunner 
pub fn get_root_dir() -> Result<PathBuf, Box<dyn Error>> {
    let homedir = get_my_home()?.unwrap_or(PathBuf::from("~"));
    let cserunner_root_dir = homedir.join(".cserunner");
    Ok(cserunner_root_dir)
}

pub fn get_workspace_list_path() -> Result<PathBuf, Box<dyn Error>> {
    Ok(get_root_dir()?.join("workspace_list.json"))
}

pub fn get_workspace_list_lock_path() -> Result<PathBuf, Box<dyn Error>> {
    Ok(get_root_dir()?.join("workspace_list.json.lock"))
}
