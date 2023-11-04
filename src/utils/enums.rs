#[derive(Debug, Clone)]
pub enum Language {
    English,
    Japanese,
    German,
    French,
    Chinese,
    Korean,
}

impl Language {
    pub fn to_str(&self) -> &str {
        match self {
            Language::English => "en",
            Language::Japanese => "ja",
            Language::German => "de",
            Language::French => "fr",
            Language::Chinese => "cn",
            Language::Korean => "ko",
        }
    }
}
