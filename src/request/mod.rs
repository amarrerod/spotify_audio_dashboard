use std::fmt::format;

use reqwest::header::HeaderMap;

use crate::models::artist::{self, Artist};
use crate::user::*;

pub async fn get_secret() -> Result<AccessToken, reqwest::Error> {
    let client = reqwest::Client::new();
    let spotify_client_token =
        std::env::var("SPOTIFY_CLIENT_ID").expect("SPOTIFY_CLIENT_ID must be set in .env");
    let spotify_client_secret =
        std::env::var("SPOTIFY_CLIENT_SECRET").expect("SPOTIFY_CLIENT_SECRET must be set in .env");
    let body : String = format!("grant_type=client_credentials&client_id={spotify_client_token}&client_secret={spotify_client_secret}").to_string();
    let response: reqwest::Response = client
        .post("https://accounts.spotify.com/api/token")
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(body)
        .send()
        .await?;
    let info: AccessToken = response.json().await?;
    Ok(info)
}

pub async fn get_artist_info(
    artist_id: &str,
    access: &AccessToken,
) -> Result<Artist, reqwest::Error> {
    let client = reqwest::Client::new();
    let query: String = format!("https://api.spotify.com/v1/artists/{artist_id}").to_string();
    println!("The access token is the following: {:#?}", access);

    let response = client
        .get(query)
        .bearer_auth(&access.access_token)
        .send()
        .await?
        .json::<serde_json::Value>()
        .await?;
    let artist = Artist::new(
        response["id"].as_str().unwrap(),
        response["name"].as_str().unwrap(),
        response["uri"].as_str().unwrap(),
        response["followers"].as_i64().unwrap_or(0) as i32,
        response["external_urls"]["spotify"].as_str().unwrap(),
        response["popularity"].as_i64().unwrap_or(0) as i32,
    );

    Ok(artist)
}
