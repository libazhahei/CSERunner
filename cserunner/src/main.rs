mod errors;
mod init;
use std::{borrow::Cow, error::Error, fs, io, rc::Rc};
use init::init_workspace;
use serde::{Deserialize, Serialize};
use clap::{Args, Parser};
#[derive(Serialize, Deserialize, Debug)]
pub struct ServerConfig<'a> {
    #[serde(borrow="'a")]
    host: Cow<'a, str>,
    port: u8,
    #[serde(borrow="'a")]
    username: Cow<'a, str>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AuthConfig<'a> {
    #[serde(borrow="'a")]
    identity_file: Cow<'a, str>,
    #[serde(borrow="'a")]
    password: Cow<'a, str>
}

#[derive(Serialize, Deserialize, Debug)]
pub enum SyncType {
    #[serde(alias="eager", alias="EAGER")]
    Eager,
    #[serde(alias="lazy", alias="LAZY")]
    Lazy
}
#[derive(Serialize, Deserialize, Debug)]
pub struct SyncConfig<'a> {
    sync_type: SyncType,
    frequency: u8,
    early_stop: u16,
    lifetime: u16,

    ignore_binary: bool, 
    #[serde(borrow="'a")]
    ignore: Cow<'a, [Cow<'a, str>]>,
    #[serde(borrow="'a")]
    extra_ignore_file: Cow<'a, [Cow<'a, str>]>,

    n_threads: i8,
    rm_alart: bool,
    timeout: u16,

    max_sync_space_size: u32,
    #[serde(borrow="'a")]
    max_sync_space_unit: Cow<'a, str>,

    compress_while_sync: bool
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Config<'a> {
    root: Option<Cow<'a, str>>,
    #[serde(borrow="'a")]
    server: ServerConfig<'a>,
    #[serde(borrow="'a")]
    auth: AuthConfig<'a>,
    #[serde(borrow="'a")]
    sync: SyncConfig<'a>
}

#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct CommandLine {
    command: String,
    args: Option<Vec<String>>,
    #[arg(short, long)]
    config: Option<String>,
}



fn main() {
    let commandline = Rc::new(CommandLine::parse());
    match commandline.clone().command.as_str() {
        "init" => init_workspace(&commandline.config),
        _ => todo!()
    };
    // let current = std::env::current_dir().unwrap();
    // println!("{:?}", current.to_str().take());
    // let config_file = std::fs::read_to_string("./example_config.toml");
    // let file = fs::File::open("./example_config.json").expect("config file does not exist");
    // let reader = io::BufReader::new(file);
    // let mut deser = serde_json::Deserializer::from_reader(reader);
    // let config = Config::deserialize(&mut deser).expect("Unexpected data format");
    // dbg!(config);
    // println!("Hello, world!");
}
