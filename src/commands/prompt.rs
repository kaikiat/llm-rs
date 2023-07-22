use llm_chain::{executor, prompt, parameters, output::Output};

use super::config::Config;

pub struct Prompt{
    pub config: Config
}

impl Prompt {
    pub fn new(config: Config) -> Prompt {
        Prompt{
            config
        }
    }

    pub async fn execute(&self, prompts: String) -> Result<Output, Box<dyn std::error::Error>> {
        match &self.config.get(false) as &str {
            "" => {
                println!("unknown api_key");
            },
            _ => {
            }
        }
    
        let exec = executor!()?;
        let response = prompt!(prompts)
                .run(&parameters!(), &exec)
                .await.unwrap();
    
        Ok(response)
    }

}