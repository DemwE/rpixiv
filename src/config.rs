use std::fs;
use std::io::{self, Write};
use std::path::PathBuf;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct User {
    pub username: String,
    pub password: String,
}

impl User {
    pub fn new(username: String, password: String) -> Self {
        User { username, password }
    }
}

fn prompt_user_details() -> User {
    print!("Enter your Pixiv username: ");
    io::stdout().flush().unwrap();
    let mut username = String::new();
    io::stdin().read_line(&mut username).expect("Failed to read line");

    print!("Enter your Pixiv password: ");
    io::stdout().flush().unwrap();
    let password = rpassword::read_password().expect("Failed to read password");

    User::new(username.trim().to_owned(), password)
}

pub struct Config {
    #[allow(dead_code)]
    config_dir: PathBuf,
    config_file: PathBuf,
}

impl Config {
    pub fn new() -> Self {
        let dir = dirs::config_dir().unwrap().join("rpixiv");
        let file = dir.join("config.toml");

        Config {
            config_dir: dir,
            config_file: file,
        }
    }

    pub fn check(&self) {
        if !self.config_file.exists() {
            println!("Creating config file...");
            let user = prompt_user_details();
            fs::create_dir_all(self.config_file.parent().unwrap())
                .expect("Failed to create directory");
            let toml = toml::to_string(&user).expect("Failed to serialize user");
            fs::write(&self.config_file, toml).expect("Failed to create config file");
            println!("Config file created at {:?}", self.config_file);
        }
    }

    pub fn read(&self) -> User {
        if !self.config_file.exists() {
            self.check();
        }

        let contents = fs::read_to_string(&self.config_file).expect("Failed to read config file");
        let user: User = toml::from_str(&contents).expect("Failed to deserialize config file");
        user
    }
}