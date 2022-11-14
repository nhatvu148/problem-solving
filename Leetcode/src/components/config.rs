use std::fs;
use std::io::Error as IoError;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct ConfigToml {
    database: Option<ConfigTomlDatabase>,
    google: Option<ConfigTomlGoogle>,
    jwt: Option<ConfigTomlJwt>,
}

#[derive(Serialize, Deserialize, Debug)]
struct ConfigTomlDatabase {
    username: Option<String>,
    password: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
struct ConfigTomlGoogle {
    api_key: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
struct ConfigTomlJwt {
    secret_token: Option<String>,
}

#[derive(Debug)]
pub struct Config {
    pub username: String,
    pub password: String,
    pub api_key: String,
    pub secret_token: String,
}

impl Config {
    pub fn new() -> Self {
        let config_filepaths: [&str; 2] = ["./config.toml", "./Config.toml"];
        let mut content: String = "".to_owned();

        for filepath in config_filepaths {
            let result: Result<String, IoError> = fs::read_to_string(filepath);

            if result.is_ok() {
                content = result.unwrap();
                break;
            }
        }

        let config_toml: ConfigToml = toml::from_str(&content).unwrap_or_else(|e| {
            println!("Failed to create ConfigToml Object out of config file.");
            ConfigToml {
                database: None,
                google: None,
                jwt: None,
            }
        });

        let (username, password): (String, String) = match config_toml.database {
            Some(database) => {
                let db_username: String = database.username.unwrap_or_else(|| {
                    println!("Missing field username in table database.");
                    "unknown".to_owned()
                });
                let db_password: String = database.password.unwrap_or_else(|| {
                    println!("Missing field password in table database.");
                    "unknown".to_owned()
                });
                (db_username, db_password)
            }
            None => {
                println!("Missing table database");
                ("unknown".to_owned(), "unknown".to_owned())
            }
        };

        let api_key: String = match config_toml.google {
            Some(google) => google.api_key.unwrap_or_else(|| {
                println!("Missing field api_key in table google");
                "unknown".to_owned()
            }),
            None => {
                println!("Missing table google");
                "unknown".to_owned()
            }
        };

        let secret_token: String = match config_toml.jwt {
            Some(jwt) => jwt.secret_token.unwrap_or_else(|| {
                println!("Missing field api_key in table jwt");
                "unknown".to_owned()
            }),
            None => {
                println!("Missing table jwt");
                "unknown".to_owned()
            }
        };

        Config {
            username: username,
            password: password,
            api_key: api_key,
            secret_token: secret_token,
        }
    }
}
