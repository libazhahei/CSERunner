use std::{borrow::Cow, fs, io};

use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct ServerConfig<'a> {
    #[serde(borrow="'a")]
    host: Cow<'a, str>,
    port: u8,
    #[serde(borrow="'a")]
    username: Cow<'a, str>
}

#[derive(Deserialize, Debug)]
pub struct AuthConfig<'a> {
    #[serde(borrow="'a")]
    identity_file: Cow<'a, str>,
    #[serde(borrow="'a")]
    password: Cow<'a, str>
}

#[derive(Deserialize, Debug)]
pub enum SyncType {
    #[serde(alias="eager", alias="EAGER")]
    Eager,
    #[serde(alias="lazy", alias="LAZY")]
    Lazy
}
#[derive(Deserialize, Debug)]
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

    max_node: u8,
    max_sync_space_size: u32,
    #[serde(borrow="'a")]
    max_sync_space_unit: Cow<'a, str>,

    compress_while_sync: bool
}

#[derive(Deserialize, Debug)]
pub struct Config<'a> {
    root: Cow<'a, str>,
    #[serde(borrow="'a")]
    server: ServerConfig<'a>,
    #[serde(borrow="'a")]
    auth: AuthConfig<'a>,
    #[serde(borrow="'a")]
    sync: SyncConfig<'a>
}


fn main() {
    // let config_file = std::fs::read_to_string("./example_config.toml");
    let file = fs::File::open("./example_config.json").expect("config file does not exist");
    let reader = io::BufReader::new(file);
    let mut deser = serde_json::Deserializer::from_reader(reader);
    let config = Config::deserialize(&mut deser).expect("Unexpected data format");
    dbg!(config);
    println!("Hello, world!");
}
