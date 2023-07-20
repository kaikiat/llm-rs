use serde_json::Value;
use std::env;
use std::io::{BufReader, Read};
use std::fs::File;
use std::{io, io::Write};
use serde_json::json;

pub struct Config;

impl Config {
    pub fn new() -> Config {
        Config{}
    }

    pub fn get(&self, debug: bool) ->  String{
        let file = File::open("config.json")
                            .expect("Failed to open config.json");
        let mut buf_reader = BufReader::new(file);
        let mut contents = String::new();

        buf_reader
            .read_to_string(&mut contents)
            .expect("Failed to read file");
        
        let data: Value = serde_json::from_str(&contents)
            .expect("Failed to parse JSON");
        
        if let Some(value) = data.get("OPENAI_API_KEY") {
            if let Some(api_key) = value.as_str() {
                if debug == true {
                    println!("[OPENAI_API_KEY]: {}", api_key);
                }
                env::set_var("OPENAI_API_KEY", api_key);
                return api_key.to_string();
            }
        }
        "".to_string()
    }

    pub fn set(&self) {
        let mut api_key = String::new();
        print!("Please enter OpenAI API key [OPENAI_API_KEY]: ");
                    
        io::stdout()
            .flush()
            .unwrap();

        io::stdin()
            .read_line(&mut api_key)
            .expect("Error in setting [OPENAI_API_KEY]");

        api_key = api_key.trim_end_matches('\n').to_string();

        let data: Value = json!({
            "OPENAI_API_KEY": api_key,
        });

        let mut file = File::create("config.json")
                            .expect("failed to create config.json");

        file
            .write_all(data.to_string().as_bytes())
            .expect("failed to write to config.json");

    }
}