use std::fs;
use llm_chain::output::Output;
use llm_chain::{executor, parameters, prompt, Parameters};
use llm_chain::step::Step;
use llm_chain::chains::map_reduce::Chain;
use super::config::Config;

pub struct Llm{
    pub config: Config
}

impl Llm {
    pub fn new(config: Config) -> Llm {
        Llm{
            config
        }
    }

    pub async fn ask(&self, prompts: String) -> Result<Output, Box<dyn std::error::Error>> {
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


    pub async fn summarise(&self, filepath: String) -> Result<Output, Box<dyn std::error::Error>> {
        let exec = executor!()?;

        let map_prompt = Step::for_prompt_template(prompt!(
            "You are a bot for summarizing wikipedia articles, you are terse and focus on accuracy",
            "Summarize this article into bullet points:\n{{text}}"
        ));

        let reduce_prompt = Step::for_prompt_template(prompt!(
            "You are a diligent bot that summarizes text",
            "Please combine the articles below into one summary as bullet points:\n{{text}}"
        ));

        let chain = Chain::new(map_prompt, reduce_prompt);
        let article = match fs::read_to_string(filepath) {
            Ok(content) => content,
            Err(err) => {
                return Err(Box::new(err))
            }
        };
        let docs = vec![parameters!(article)];

        let response = chain.run(docs, Parameters::new(), &exec).await.unwrap();

        Ok(response)
    }





}