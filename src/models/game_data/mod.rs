use serde_json::Value;

use crate::utils::error::XIVAPIError;

pub struct GameData<'a> {
    pub client: &'a reqwest::Client,
    pub config: &'a crate::config::Config,
}

impl<'a> GameData<'a> {
    pub async fn content(&self) -> Result<Value, XIVAPIError> {
        let url = self.config.build_url("/content");

        Ok(self.client.get(&url).send().await?.json::<Value>().await?)
    }

    // TODO Handle params
    pub async fn list(&self, name: &str) -> Result<Value, XIVAPIError> {
        let url = self.config.build_url(&format!("/{}", name));

        Ok(self.client.get(&url).send().await?.json::<Value>().await?)
    }

    // TODO Handle params
    pub async fn get(&self, name: &str, id: &str) -> Result<Value, XIVAPIError> {
        let url = self.config.build_url(&format!("/{}/{}", name, id));

        Ok(self.client.get(&url).send().await?.json::<Value>().await?)
    }

    pub async fn servers(&self) -> Result<Value, XIVAPIError> {
        let url = self.config.build_url("/servers");

        Ok(self.client.get(&url).send().await?.json::<Value>().await?)
    }

    pub async fn data_centers(&self) -> Result<Value, XIVAPIError> {
        let url = self.config.build_url("/servers/dc");

        Ok(self.client.get(&url).send().await?.json::<Value>().await?)
    }
}
