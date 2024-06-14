mod errors;
mod init;
mod check;
use std::{borrow::Cow, env, error::Error, fs, io, process::exit, rc::Rc};
use check::{config_file_check, root_dir_check};
use init::{get_workspace_config, init_workspace};
use serde::{Deserialize, Serialize};
use clap::{Args, Parser};

/// ServerConfig holds configuration details for connecting to a server.
#[derive(Serialize, Deserialize, Debug)]
pub struct ServerConfig<'a> {

    /// The server's hostname or IP address.
    #[serde(borrow="'a")]
    host: Cow<'a, str>,
    /// The server's port number.
    port: u8,
    /// The username for authentication on the server.
    #[serde(borrow="'a")]
    username: Cow<'a, str>
}

/// AuthConfig holds configuration details for authentication.
#[derive(Serialize, Deserialize, Debug)]
pub struct AuthConfig<'a> {
    /// Path to the authentication identity file.
    #[serde(borrow="'a")]
    identity_file: Cow<'a, str>,

    /// The password for authentication.
    #[serde(borrow="'a")]
    password: Cow<'a, str>
}

/// SyncType defines the type of data synchronization to perform.
#[derive(Serialize, Deserialize, Debug)]
pub enum SyncType {
    /// Eager synchronization performs a full sync immediately.
    #[serde(alias="eager", alias="EAGER")]
    Eager,
    /// Lazy synchronization performs the initial sync and defers further updates.
    #[serde(alias="lazy", alias="LAZY")]
    Lazy
}

/// SyncConfig holds configuration details for data synchronization.
#[derive(Serialize, Deserialize, Debug)]
pub struct SyncConfig<'a> {
    /// The type of data synchronization to perform (eager or lazy).
    ///   - `Eager`: Performs the synchronization periodicly within a a maxium lifetime.
    ///   - `Lazy`: Performs an sync everytime we run cserun.
    sync_type: SyncType,
    /// Frequency of checks for synchronization updates (in some unit).
    frequency: u8,
    /// Threshold for stopping a sync early if changes are minimal.
    early_stop: u16,
    /// Maximum time of synchronization.
    lifetime: u16,

    // Whether to ignore binary files during synchronization.
    ignore_binary: bool, 
    #[serde(borrow="'a")]
    /// List of file patterns to ignore during synchronization.
    ignore: Cow<'a, [Cow<'a, str>]>,

    /// Path to a file containing additional file patterns to ignore during synchronization.
    #[serde(borrow="'a")]
    extra_ignore_file: Cow<'a, [Cow<'a, str>]>,

    /// Number of threads to use for parallel synchronization.
    n_threads: i8,
    /// Whether to send alerts (notifications) when removal operations occur during synchronization. 
    rm_alart: bool,
    // Maximum allowed time for a synchronization operation to complete.
    timeout: u16,

    /// Maximum size allowed for the synchronization workspace,
    max_sync_space_size: u32,
    /// Unit of measurement for the `max_sync_space_size` value. 
    #[serde(borrow="'a")]
    max_sync_space_unit: Cow<'a, str>,

    /// Whether to compress data during synchronization. 
    compress_while_sync: bool
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Config<'a> {
    /// Optional root directory path. If specified, relative paths in other configuration
    /// options (e.g., `ignore` patterns in `sync`) will be resolved relative to this root.
    root: Option<Cow<'a, str>>,
    /// Server configuration details for connecting to the remote server.
    #[serde(borrow="'a")]
    server: ServerConfig<'a>,
    /// Authentication configuration for accessing the remote server.
    #[serde(borrow="'a")]
    auth: AuthConfig<'a>,
    /// Synchronization configuration for managing data transfer between local and remote.
    #[serde(borrow="'a")]
    sync: SyncConfig<'a>
}

#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct CommandLine {
    /// The main command to be executed. This is typically the subcommand or action
    /// the user wants to perform.
    command: String,
    /// Optional list of additional arguments for the specified command.
    args: Option<Vec<String>>,
    /// Optional path to a configuration file. The application can load configuration
    /// settings from this file.
    #[arg(short, long)]
    config: Option<String>,
}

// Set panic handler

fn main() {
    // Checking 
    std::panic::set_hook(Box::new(|panic_info| {
        let location = panic_info.location().unwrap();
        let message = panic_info.payload().downcast_ref::<&str>().unwrap();
        eprintln!("Panic occurred at {}:{}: {}", location.file(), location.line(), message);
        exit(1);
    }));
    root_dir_check().unwrap();
    let commandline = Rc::new(CommandLine::parse());
    // if commandline is init 
    if commandline.command == "init" {
        // Exit with 1 if error 
        if let Err(_) = init_workspace(&commandline.config) {
            exit(1);
        } 
        exit(0);
    }
    config_file_check().unwrap();
    let current_dir = env::current_dir().unwrap();
    let config = get_workspace_config(&current_dir).expect("You have to init the workspace first. Run `cserun init` to initialize the workspace.");

}
