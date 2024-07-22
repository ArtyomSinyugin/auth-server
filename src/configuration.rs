use std::path::PathBuf;

use config::*;
use serde::Deserialize;

#[derive(Deserialize, Clone)]
#[allow(non_snake_case)]
pub struct Configuration {
   // pub templates: String,
    pub ip: String,
    pub port: String, 
}

pub fn deserialize_config (raw_path: &PathBuf) -> Configuration {
    let path = raw_path.join("server_config");
    println!("{:?}", path);
    let path = path.as_os_str().to_str().unwrap();
    Config::builder()
        .add_source(File::new(path, FileFormat::Ini))
        .build()
        .unwrap()
        .try_deserialize::<Configuration>()
        .unwrap()
}