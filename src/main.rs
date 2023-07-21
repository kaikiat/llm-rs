use clap::{Parser, Subcommand};
use llm_chain::{executor, parameters, prompt};

mod commands;

use llm_chain::chains::sequential::Chain;
use llm_chain::serialization::StorableEntity;
use llm_chain::step::Step;
use llm_chain::traits::Executor as ExecutorTrait;
use llm_chain_openai::chatgpt::Executor;

#[derive(Parser)]
#[command(name = "llm-rs cli tool")]
#[command(author = "Kai Kiat Poh")]
#[command(author, version, about, long_about = None)]
struct Cli {
    prompts: Option<String>,

    #[command(subcommand)]
    configure: Option<Config>,

    #[arg(short,long)]
    #[clap(default_value="false")]
    chain: bool
}

#[derive(Subcommand,Debug)] 
enum Config {
    #[clap(about = "View OpenAI API key")]
    View,
    #[clap(about = "Set OpenAI API key")]
    Set,
}

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();
    let config = commands::config::Config::new();

    println!("Value for chain: {}", cli.chain);
    
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
        if prompts.len() > 0 && cli.chain == false {
            match &config.get(false) as &str {
                "" => {
                    println!("unknown api_key");
                },
                _ => {
                }
            }
            let exec = executor!()?;
            let response = prompt!(prompts)
                    .run(&parameters!(), &exec)
                    .await?;
            println!("{}", response);
            return Ok(())
        } 
        else if prompts.len() > 0 && cli.chain == true { 
            println!("chain")
            // let chatgpt = Executor::new()?;
            // let mut path = std::env::temp_dir();
            // path.push("chain-from-json.yaml");
            // let path = path.to_str().unwrap();
        
            // let chain_to_write: Chain = Step::for_prompt_template(prompt!(
            //     "You are a bot for making personalized greetings",
            //     "Make a personalized greet for Joe"
            // ))
            // .to_chain();
            // chain_to_write.write_file_sync(path)?;
            // println!("Wrote chain to {}", path);
        
            // let chain = Chain::read_file_sync(path).unwrap();
            // let res = chain.run(parameters!(), &chatgpt).await.unwrap();
            // println!("{}", res);
            // Ok(())
        }
    }

    Ok(())
   
    // match cli.configure {
 
 
}