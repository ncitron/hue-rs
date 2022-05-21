use std::collections::HashMap;
use serde::ser::Serialize;
use eyre::Result;
use dotenv::dotenv;
use std::env;

#[tokio::main]
async fn main() -> Result<()> {
    dotenv().ok();

    let ip = env::var("IP_ADDR")?;
    let user = env::var("USER_KEY")?;

    let client = HueClient {
        ip: &ip,
        user: &user
    };

    // client.set_group_color(1, 0.1418, 0.0986).await?;
    client.set_group_on(1, true).await?;

    Ok(())
}

struct HueClient<'a> {
    ip: &'a str,
    user: &'a str
}

impl<'a> HueClient<'a> {
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

