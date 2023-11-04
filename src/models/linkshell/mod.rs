use serde_json::Value;

use crate::utils::error::XIVAPIError;

pub struct Linkshell<'a> {
    pub client: &'a reqwest::Client,
    pub config: &'a crate::config::Config,
}

impl<'a> Linkshell<'a> {
    pub async fn search(
        &self,
        name: &str,
        server: Option<&str>,
        page: Option<i32>,
    ) -> Result<Value, XIVAPIError> {
        let mut url = self.config.build_url("/linkshell/search");

        url.push_str(&format!("&name={}", name));

        if let Some(server) = server {
            url.push_str(&format!("&server={}", server));
        }

        if let Some(page) = page {
            url.push_str(&format!("&page={}", page));
        }

        Ok(self.client.get(&url).send().await?.json::<Value>().await?)
    }

    pub async fn cross_world_search(
        &self,
        name: &str,
        page: Option<i32>,
    ) -> Result<Value, XIVAPIError> {
        let mut url = self.config.build_url("/linkshell/crossworld/search");

        url.push_str(&format!("&name={}", name));

        if let Some(page) = page {
            url.push_str(&format!("&page={}", page));
        }

        Ok(self.client.get(&url).send().await?.json::<Value>().await?)
    }

    pub async fn get(&self, id: u64) -> Result<Value, XIVAPIError> {
        let url = self.config.build_url(&format!("/linkshell/{}", id));

        Ok(self.client.get(&url).send().await?.json::<Value>().await?)
    }

    pub async fn cross_world_get(&self, id: &str) -> Result<Value, XIVAPIError> {
        let url = self
            .config
            .build_url(&format!("/linkshell/crossworld/{}", id));

        Ok(self.client.get(&url).send().await?.json::<Value>().await?)
    }
}
