use crate::Language;

#[derive(Debug, Clone)]
pub struct Config {
    pub url: String,
    pub key: String,
    pub language: Language,
    pub snake_case: Option<bool>,
    pub pretty: Option<bool>,
    pub staging: Option<bool>,
}

impl Config {
    pub fn new(
        key: String,
        language: Language,
        snake_case: Option<bool>,
        pretty: Option<bool>,
        staging: Option<bool>,
    ) -> Self {
        let url = if let Some(staging) = staging {
            if staging {
                "https://staging.xivapi.com".to_string()
            } else {
                "https://xivapi.com".to_string()
            }
        } else {
            "https://xivapi.com".to_string()
        };
        Self {
            url,
            key,
            language,
            snake_case,
            pretty,
            staging,
        }
    }

    pub fn build_url(&self, endpoint: &str) -> String {
        format!(
            "{}{}?private_key={}&language={}",
            self.url,
            endpoint,
            self.key,
            self.language.to_str()
        )
    }
}
