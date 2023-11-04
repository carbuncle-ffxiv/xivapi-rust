use serde_json::Value;

use crate::utils::error::XIVAPIError;

pub enum FreeCompanyOption {
    FCM, // Free Company Members
}

impl FreeCompanyOption {
    fn to_str(&self) -> &str {
        match self {
            FreeCompanyOption::FCM => "FCM",
        }
    }
}

pub struct FreeCompany<'a> {
    pub client: &'a reqwest::Client,
    pub config: &'a crate::config::Config,
}

impl<'a> FreeCompany<'a> {
    pub async fn search(
        &self,
        name: &str,
        server: Option<&str>,
        page: Option<i32>,
    ) -> Result<Value, XIVAPIError> {
        let mut url = self.config.build_url("/freecompany/search");

        url.push_str(&format!("&name={}", name));

        if let Some(server) = server {
            url.push_str(&format!("&server={}", server));
        }

        if let Some(page) = page {
            url.push_str(&format!("&page={}", page));
        }

        Ok(self.client.get(&url).send().await?.json::<Value>().await?)
    }

    pub async fn get(
        &self,
        id: u64,
        data: Option<Vec<FreeCompanyOption>>,
    ) -> Result<Value, XIVAPIError> {
        let mut url = self.config.build_url(&format!("/freecompany/{}", id));

        if let Some(data) = data {
            url.push_str(&format!(
                "&data={}",
                data.iter()
                    .map(|option: &FreeCompanyOption| option.to_str())
                    .collect::<Vec<&str>>()
                    .join(",")
            ));
        }

        Ok(self.client.get(&url).send().await?.json::<Value>().await?)
    }
}
