mod config;

mod client;
pub use client::*;

mod utils;
pub use utils::enums::Language;

mod models;
pub use models::{
    character::Character, free_company::FreeCompany, game_data::GameData, linkshell::Linkshell,
    pvp_team::PvPTeam,
};

pub struct XIVClient {
    client: client::Client,
    config: config::Config,
}

impl XIVClient {
    pub fn new(
        key: String,
        language: Language,
        snake_case: Option<bool>,
        pretty: Option<bool>,
        staging: Option<bool>,
    ) -> Self {
        Self {
            client: reqwest::Client::new(),
            config: config::Config::new(key, language, snake_case, pretty, staging),
        }
    }

    pub fn character(&self) -> Character {
        Character {
            client: &self.client,
            config: &self.config,
        }
    }

    pub fn free_company(&self) -> FreeCompany {
        FreeCompany {
            client: &self.client,
            config: &self.config,
        }
    }

    pub fn game_data(&self) -> GameData {
        GameData {
            client: &self.client,
            config: &self.config,
        }
    }

    pub fn linkshell(&self) -> Linkshell {
        Linkshell {
            client: &self.client,
            config: &self.config,
        }
    }

    pub fn pvp_team(&self) -> PvPTeam {
        PvPTeam {
            client: &self.client,
            config: &self.config,
        }
    }
}
