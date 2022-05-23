use slint::Model;
use std::rc::Rc;
use serde::{Serialize, Deserialize};
use serde_json;
use reqwest;
use std::fs;
use once_cell::sync::OnceCell;

slint::include_modules!();

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    authentication: Auth
}

impl Config {
        fn from_toml(file_name: &str) -> Self {
                toml::from_str(&fs::read_to_string(file_name).unwrap()).unwrap()
            }
    }


pub static CONFIG: OnceCell<Config> = OnceCell::new();

#[derive(Serialize, Deserialize, Debug)]
pub struct File {
    name: String,
    path: String,
    size: u64,
    creation_date: u64,
    modified_date: u64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Auth {
        username: String,
        password: String,
    }

// fn get_files(dir: &str) -> Vec<File> {
    
// }

#[tokio::main]
pub fn main() {
CONFIG.set(Config::from_toml("config.toml")).unwrap();    
    let client = reqwest::Client::new();
    let resp = client.get("http://localhost:8080/api/files")
.basic_auth(CONFIG.get().unwrap().authentication.username, Some(CONFIG.get().unwrap().authentication.password))        
        .send().await.unwrap()
            .json::<Vec<File>>()
            .await.unwrap();    
    let main_window = MainWindow::new();
    main_window.run();
}