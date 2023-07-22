use clap::{Parser, Subcommand};

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
async fn main() -> Result<(), errors::Error> {
    let cli = Cli::parse();
    let config = commands::config::Config::new();
    let prompt_client = commands::llm::Llm::new(config.clone());
    
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
            let response = prompt_client.ask(prompts.to_string()).await;
            match response {
                Ok(output) => {
                    println!("{}", output);
                    return Ok(());
                }
                Err(err) => {
                    return Err(errors::Error::CustomInput(err.to_string()));
                }
            }
        } 
    }

    if let Some(filepath) = cli.file {

        let response = prompt_client.summarise(filepath.to_string()).await;
            match response {
                Ok(output) => {
                    println!("{}", output);
                    return Ok(());
                }
                Err(err) => {
                    return Err(errors::Error::CustomInput(err.to_string()));
                }
            }
    }

    return Err(errors::Error::CustomInput("an errror occurred, type --help to view the commands".to_string()))
    
}