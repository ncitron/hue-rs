use std::collections::HashMap;
use serde::ser::Serialize;
use serde::Deserialize;
use eyre::Result;
use clap::{Parser, Subcommand};
use std::path::PathBuf;
use std::fs::read_to_string;

#[tokio::main]
async fn main() -> Result<()> {
   
    let home = dirs::home_dir().unwrap();
    let client = HueClient::from_config(home.join("hueconfig.json"))?;

    let cli = Cli::parse();

    match &cli.command {
        Commands::Toggle { group, status } => {
            match status.as_str() {
                "on" => {
                    client.set_group_on(*group, true).await
                },
                "off" => {
                    client.set_group_on(*group, false).await
                },
                _ => {
                    Err(eyre::eyre!("status must be on or off"))
                }
            }
        },
        Commands::Color { group, xy } => {
            client.set_group_color(*group, xy[0], xy[1]).await
        }
    }
}

#[derive(Parser)]
#[clap(author, version, about)]
struct Cli {
    #[clap(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Toggle { group: usize, status: String },
    Color {
        group: usize,
        #[clap(long, min_values=2, max_values=2)]
        xy: Vec<f64>,
    },
}

struct HueClient {
    ip: String,
    user: String,
}

#[derive(Deserialize)]
struct Config {
    ip: String,
    user: String,
}

impl HueClient {
    pub fn from_config(config: PathBuf) -> Result<HueClient> {
        let config_str = read_to_string(config)?;
        let config: Config = serde_json::from_str(&config_str)?;
        
        Ok(HueClient { ip: config.ip, user: config.user })
    }

    async fn set_light_state<V: Serialize>(&self, light: usize, state: HashMap<&str, V>) -> Result<()> {
        let client = reqwest::Client::new();
        let url = format!("http://{}/api/{}/lights/{}/state", self.ip, self.user, light); 
        client.put(url).json(&state).send().await?;
        Ok(())
    }

    async fn set_group_state<V: Serialize>(&self, group: usize, state: HashMap<&str, V>) -> Result<()> {
        let client = reqwest::Client::new();
        let url = format!("http://{}/api/{}/groups/{}/action", self.ip, self.user, group);
        client.put(url).json(&state).send().await?;

        Ok(())
    }

    pub async fn set_light_on(&self, light: usize, on: bool) -> Result<()> {
        let mut state = HashMap::new();
        state.insert("on", on);
        self.set_light_state(light, state).await?;
    
        Ok(())
    }

    pub async fn set_group_on(&self, group: usize, on: bool) -> Result<()> {
        let mut state = HashMap::new();
        state.insert("on", on);
        self.set_group_state(group, state).await?;

        Ok(())
    }

    pub async fn set_light_color(&self, light: usize, x: f64, y: f64) -> Result<()> {
        let mut state = HashMap::new();
        state.insert("xy", vec![x, y]);
        self.set_light_state(light, state).await?;
    
        Ok(())
    }
    
    pub async fn set_group_color(&self, group: usize, x: f64, y: f64) -> Result<()> {
        let mut state = HashMap::new();
        state.insert("xy", vec![x, y]);
        self.set_group_state(group, state).await?;

        Ok(())
    }
}

