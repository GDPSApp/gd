use bon::Builder;
use reqwest::Url;

pub const DEFAULT_BASE_URL: &str = "https://www.boomlings.com/database";

#[derive(Debug, Clone, Builder)]
pub struct Client {
    pub client: reqwest::Client,
    pub base_url: Url,
    // pub game_version: GameVersion,
    // pub binary_version: BinaryVersion,
}

impl Default for Client {
    fn default() -> Self {
        let client = Default::default();

        let base_url = Url::parse(DEFAULT_BASE_URL).unwrap();

        Self { client, base_url }
    }
}
