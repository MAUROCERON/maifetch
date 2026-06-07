use reqwest::blocking::{Client, Response};
use serde::Deserialize;
use std::error::Error;
use std::time::Duration;

const BASE_URL: &str = "https://maitea.app";

#[derive(Debug, Deserialize)]
pub struct Image {
    pub png: String,
}

#[derive(Debug, Deserialize)]
pub struct Profile {
    pub id: i64,
    pub name: String,
    pub rating: i64,
    pub rating_highest: i64,
    pub level: i64,
    pub play_stats: PlayStats,
    pub options: ProfileOptions,
}

#[derive(Debug, Deserialize)]
pub struct PlayStats {
    pub total: i64,
}

#[derive(Debug, Deserialize)]
pub struct ProfileOptions {
    pub icon: Image,
}

#[derive(Debug, Deserialize)]
pub struct TrackInfo {
    pub name: LocalizedName,
}

#[derive(Debug, Deserialize)]
pub struct LocalizedName {
    pub en: String,
}

#[derive(Debug, Deserialize)]
pub struct DifficultyLevel {
    pub value: String,
}

#[derive(Debug, Deserialize)]
pub struct Play {
    pub achievement_formatted: String,
    pub score_formatted: String,
    pub rank: String,
    pub full_combo_label: Option<String>,
    pub difficulty_level: DifficultyLevel,
    pub song: TrackInfo,
}

#[derive(Debug, Deserialize)]
struct DataEnvelope<T> {
    data: T,
}

pub struct ApiClient {
    access_token: String,
    client: Client,
}

impl ApiClient {
    pub fn new(access_token: String) -> Result<Self, Box<dyn Error>> {
        let client = Client::builder().timeout(Duration::from_secs(30)).build()?;

        Ok(Self {
            access_token,
            client,
        })
    }

    pub fn get_profiles(&self) -> Result<Vec<Profile>, Box<dyn Error>> {
        let response: DataEnvelope<Vec<Profile>> = self.get("/api/v1/profiles")?.json()?;
        Ok(response.data)
    }

    pub fn get_plays(&self) -> Result<Vec<Play>, Box<dyn Error>> {
        let response: DataEnvelope<Vec<Play>> = self.get("/api/v1/plays")?.json()?;
        Ok(response.data)
    }

    pub fn get_bytes(&self, url: &str) -> Result<Vec<u8>, Box<dyn Error>> {
        let response = self.client.get(url).send()?.error_for_status()?;
        Ok(response.bytes()?.to_vec())
    }

    fn get(&self, path: &str) -> Result<Response, Box<dyn Error>> {
        let url = format!("{}{}", BASE_URL, path);
        let response = self
            .client
            .get(url)
            .bearer_auth(&self.access_token)
            .header("Content-Type", "application/json")
            .header("Accept", "application/json")
            .send()?
            .error_for_status()?;

        Ok(response)
    }
}

pub fn difficulty_string(diff: &str) -> String {
    match diff {
        "easy" => bg("Easy", 69, 174, 255),
        "basic" => bg("Basic", 111, 212, 61),
        "advanced" => bg("Advanced", 248, 183, 9),
        "expert" => bg("Expert", 255, 46, 66),
        "master" => bg("Master", 171, 140, 233),
        "remaster" | "re:master" => bg("Re:Master", 207, 114, 237),
        "utage" => bg("Utage", 255, 68, 1),
        _ => diff.to_string(),
    }
}

pub fn rank_string(rank: &str) -> String {
    match rank {
        "SSS+" => format!(
            "{}{}{}{}",
            fg("S", 255, 200, 54),
            fg("S", 225, 38, 165),
            fg("S", 73, 64, 233),
            fg("+", 21, 203, 148)
        ),
        "SSS" => format!(
            "{}{}{}",
            fg("S", 255, 200, 54),
            fg("S", 232, 39, 148),
            fg("S", 18, 195, 144)
        ),
        "SS+" => color("SS+", (248, 200, 75), (143, 71, 33)),
        "SS" => color("SS", (248, 200, 75), (143, 71, 33)),
        "S+" => color("S+", (248, 200, 75), (75, 82, 82)),
        "S" => color("S", (248, 200, 75), (75, 82, 82)),
        "AAA" => fg("AAA", 23, 163, 255),
        "AA" => fg("AA", 23, 163, 255),
        "A" => fg("A", 23, 163, 255),
        _ => rank.to_string(),
    }
}

pub fn accent(text: &str) -> String {
    fg(text, 72, 184, 200)
}

fn fg(text: &str, r: u8, g: u8, b: u8) -> String {
    format!("\x1b[38;2;{};{};{}m{}\x1b[0m", r, g, b, text)
}

fn bg(text: &str, r: u8, g: u8, b: u8) -> String {
    color(text, (255, 255, 255), (r, g, b))
}

fn color(text: &str, foreground: (u8, u8, u8), background: (u8, u8, u8)) -> String {
    let (fr, fg, fb) = foreground;
    let (br, bg, bb) = background;
    format!(
        "\x1b[38;2;{};{};{}m\x1b[48;2;{};{};{}m{}\x1b[0m",
        fr, fg, fb, br, bg, bb, text
    )
}
