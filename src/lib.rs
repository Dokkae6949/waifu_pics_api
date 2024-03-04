use std::fmt;
use std::fmt::Display;
use serde::{Deserialize, Serialize};
use thiserror::Error;


#[derive(Error, Debug)]
pub enum Error {
    #[error("Network error: {0}")]
    NetworkError(#[from] reqwest::Error),

    #[error("Parse error: {0}")]
    ParseError(#[from] serde_json::Error),
}

#[derive(Clone, Debug)]
pub enum SfwCategory {
    Waifu,
    Neko,
    Shinobu,
    Megumin,
    Bully,
    Cuddle,
    Cry,
    Hug,
    Awoo,
    Kiss,
    Lick,
    Pat,
    Smug,
    Bonk,
    Yeet,
    Blush,
    Smile,
    Wave,
    Highfive,
    Handhold,
    Nom,
    Bite,
    Glomp,
    Slap,
    Kill,
    Kick,
    Happy,
    Wink,
    Poke,
    Dance,
    Cringe,
}

#[derive(Clone, Debug)]
pub enum NsfwCategory {
    Waifu,
    Neko,
    Trap,
    Blowjob,
}

#[derive(Clone, Debug)]
pub enum Type {
    Sfw(SfwCategory),
    Nsfw(NsfwCategory),
}


#[derive(Debug, Deserialize, Serialize)]
struct Response {
    url: String,
}

#[derive(Debug, Deserialize, Serialize)]
struct ResponseMany {
    files: Vec<String>,
}

#[derive(Debug, Deserialize, Serialize)]
struct RequestMany {
    exclude: Vec<String>,
}


impl Display for SfwCategory {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl Display for NsfwCategory {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}


pub async fn get_image_url(config: &Type) -> Result<String, Error> {
    let url = match config {
        Type::Sfw(category) =>
            format!("https://api.waifu.pics/sfw/{}", category.to_string().to_lowercase()),
        Type::Nsfw(category) =>
            format!("https://api.waifu.pics/nsfw/{}", category.to_string().to_lowercase()),
    };

    let response = reqwest::get(url).await?;
    let body = response.text().await?;
    let json: Response = serde_json::from_str(&body)?;

    Ok(json.url)
}

pub async fn get_image_urls(config: &Type, excluded_urls: Vec<String>) -> Result<Vec<String>, Error> {
    let url = match config {
        Type::Sfw(category) =>
            format!("https://api.waifu.pics/many/sfw/{}", category.to_string().to_lowercase()),
        Type::Nsfw(category) =>
            format!("https://api.waifu.pics/many/nsfw/{}", category.to_string().to_lowercase()),
    };

    let client = reqwest::Client::new();
    let response = client.post(url)
        .json(&RequestMany {exclude: excluded_urls})
        .send().await?;
    let response_body = response.text().await?;
    let json: ResponseMany = serde_json::from_str(&response_body)?;

    Ok(json.files)
}


#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_sfw_neko() {
        let result = get_image_url(&Type::Sfw(SfwCategory::Neko)).await.unwrap();
        assert!(result.starts_with("https://"));
    }

    #[tokio::test]
    async fn test_nsfw_neko() {
        let result = get_image_url(&Type::Nsfw(NsfwCategory::Neko)).await.unwrap();
        assert!(result.starts_with("https://"));
    }

    #[tokio::test]
    async fn test_sfw_neko_many_no_exclude() {
        let result = get_image_urls(
            &Type::Sfw(SfwCategory::Neko),
            vec![]
        ).await.unwrap();

        assert!(result.len() > 0);
        assert!(result.first().expect("Empty result").starts_with("https://"));
    }

    #[tokio::test]
    async fn test_sfw_neko_many_one_exclude() {
        let result = get_image_urls(
            &Type::Sfw(SfwCategory::Neko),
            vec!["https://i.waifu.pics/GLGqCnV.gif".to_string()]
        ).await.unwrap();

        assert!(result.len() > 0);
        assert!(result.first().expect("Empty result").starts_with("https://"));
    }
}
