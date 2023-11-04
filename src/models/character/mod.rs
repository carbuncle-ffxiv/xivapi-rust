use serde_json::Value;

use crate::utils::error::XIVAPIError;

#[derive(Debug)]
pub enum CharacterOption {
    AC,   // Achievements
    FR,   // Friends
    FC,   // Free Company
    FCM,  // Free Company Members
    MIMO, // Minions and Mounts
    PVP,  // PVP Team
}

impl CharacterOption {
    fn to_str(&self) -> &str {
        match self {
            CharacterOption::AC => "AC",
            CharacterOption::FR => "FR",
            CharacterOption::FC => "FC",
            CharacterOption::FCM => "FCM",
            CharacterOption::MIMO => "MIMO",
            CharacterOption::PVP => "PVP",
        }
    }
}

pub struct Character<'a> {
    pub client: &'a reqwest::Client,
    pub config: &'a crate::config::Config,
}

impl<'a> Character<'a> {
    pub async fn search(
        &self,
        name: &str,
        server: Option<&str>,
        page: Option<i32>,
    ) -> Result<Value, XIVAPIError> {
        let mut url = self.config.build_url("/character/search");

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
        id: i64,
        extended: bool,
        data: Option<Vec<CharacterOption>>,
    ) -> Result<Value, XIVAPIError> {
        let mut url = self.config.build_url(&format!("/character/{}", id));

        if extended {
            url.push_str("&extended=1");
        }

        if let Some(data) = data {
            let data_str = data
                .iter()
                .map(|option: &CharacterOption| option.to_str())
                .collect::<Vec<&str>>()
                .join(",");

            url.push_str(&format!("&data={}", data_str));
        }

        Ok(self.client.get(&url).send().await?.json::<Value>().await?)
    }
}
