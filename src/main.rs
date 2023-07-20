
use clap::{Parser, Subcommand};
use llm_chain::{executor, parameters, prompt};

mod commands;

#[derive(Parser)]
#[command(name = "llm-rs cli tool")]
#[command(author = "Kai Kiat Poh")]
#[command(author, version, about, long_about = None)]
struct Cli {
    prompts: Option<String>,
    #[command(subcommand)]
    command: Option<Config>,
}

#[derive(Subcommand,Debug)] 
enum Config {
    #[clap(about = "View OpenAI API key")]
    View,
    #[clap(about = "Set OpenAI API key")]
    Set,
}

// cargo run -- help
// cargo run -- config view
// cargo run -- config set
// cargo run -- "Tell me more about covid"
#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();
    let config = commands::config::Config::new();

    match cli.command {
        Some(Config::View) => {
            config.get(true);                
            Ok(())
        }
        Some(Config::Set) => {
            config.set();
            Ok(())
        }
        None => {
            config.get(false);
            if let Some(prompts) = cli.prompts.as_deref() {
                let exec = executor!()?;
                let response = prompt!(prompts)
                    .run(&parameters!(), &exec)
                    .await?;
                println!("{}", response);
                Ok(())
            } else {
                Ok(())
            }
        }
    }
 
}