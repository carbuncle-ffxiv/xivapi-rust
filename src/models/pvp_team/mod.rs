use serde_json::Value;

use crate::utils::error::XIVAPIError;

pub struct PvPTeam<'a> {
    pub client: &'a reqwest::Client,
    pub config: &'a crate::config::Config,
}

impl<'a> PvPTeam<'a> {
    pub async fn search(
        &self,
        name: &str,
        server: Option<&str>,
        page: Option<i32>,
    ) -> Result<Value, XIVAPIError> {
        let mut url = self.config.build_url("/pvpteam/search");

        url.push_str(&format!("&name={}", name));

        if let Some(server) = server {
            url.push_str(&format!("&server={}", server));
        }

        if let Some(page) = page {
            url.push_str(&format!("&page={}", page));
        }

        Ok(self.client.get(&url).send().await?.json::<Value>().await?)
    }

    pub async fn get(&self, id: &str) -> Result<Value, XIVAPIError> {
        let url = self.config.build_url(&format!("/pvpteam/{}", id));

        Ok(self.client.get(&url).send().await?.json::<Value>().await?)
    }
}
