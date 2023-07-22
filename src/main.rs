use std::fs;
use clap::{Parser, Subcommand};
use llm_chain::{executor, parameters, prompt, Parameters};
use llm_chain::step::Step;
use llm_chain::chains::map_reduce::Chain;

mod commands;
mod errors;


#[derive(Parser)]
#[command(name = "llm-rs cli tool")]
#[command(author = "Kai Kiat Poh")]
#[command(author, version, about, long_about = None)]
struct Cli {
    prompts: Option<String>,

    #[command(subcommand)]
    configure: Option<Config>,

    #[arg(long)]
    file: Option<String>
}

#[derive(Subcommand,Debug)] 
enum Config {
    #[clap(about = "View OpenAI API key")]
    View,
    #[clap(about = "Set OpenAI API key")]
    Set,
}

#[tokio::main(flavor = "current_thread")]
// async fn main() -> Result<(), Box<dyn std::error::Error>> {
async fn main() -> Result<(), errors::Error> {
    let cli = Cli::parse();
    let config = commands::config::Config::new();
    let prompt_client = commands::prompt::Prompt::new(config.clone());
    
    match cli.configure {
        Some(Config::View) => {
            config.get(true);                
            return Ok(())
        }
        Some(Config::Set) => {
            config.set();
            return Ok(())
        }
        None => {}
    };
    
    if let Some(prompts) = cli.prompts.as_deref() {
        if prompts.len() > 0{
            let response = prompt_client.execute(prompts.to_string()).await;
            match response {
                Ok(output) => {
                    println!("{}", output);
                    return Ok(());
                }
                Err(err) => {
                    return Err(errors::Error::CustomInput(err.to_string()));
                }
            }
            // prompts.execute(prompts);
            // match &config.get(false) as &str {
            //     "" => {
            //         println!("unknown api_key");
            //     },
            //     _ => {
            //     }
            // }
            // let exec = executor!()?;
            // let response = prompt!(prompts)
            //         .run(&parameters!(), &exec)
            //         .await.unwrap();
            // println!("{}", response);
            // return Ok(())
        } 
    }

    if let Some(filepath) = cli.file {
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
                return Err(errors::Error::CustomInput(err.to_string()))
            }
        };
        let docs = vec![parameters!(article)];

        let res = chain.run(docs, Parameters::new(), &exec).await.unwrap();

        println!("{}", res);

        return Ok(())
    }

    return Err(errors::Error::CustomInput("an errror occurred, type --help to view the commands".to_string()))
    
}