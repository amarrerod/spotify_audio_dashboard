use log::info;
use rspotify::{prelude::*, scopes, AuthCodeSpotify, Credentials, OAuth, Token};
use std::path::Path;

fn load_token(path: &Path) -> Result<Token, rspotify::model::error::ModelError> {
    info!("Loading Token from cache");
    let token: Token = Token::from_cache(&path)?;
    info!("Token loaded");
    Ok(token)
}

async fn log_user(cache_file: &Path) -> Result<Token, rspotify::model::error::ModelError> {
    info!("Login user");

    let credentials = Credentials::from_env().unwrap();
    info!("Credentials obtained!");
    // Using every possible scope
    let scopes = scopes!(
        "user-read-email",
        "user-read-private",
        "user-top-read",
        "user-read-recently-played",
        "user-follow-read",
        "user-library-read",
        "user-read-currently-playing",
        "user-read-playback-state",
        "user-read-playback-position",
        "playlist-read-collaborative",
        "playlist-read-private"
    );
    info!("Scopes selected are: {:?}", scopes);

    let oauth = OAuth::from_env(scopes).unwrap();
    let spotify = AuthCodeSpotify::new(credentials.clone(), oauth.clone());
    let url = spotify.get_authorize_url(false).unwrap();
    // This function requires the `cli` feature enabled.
    spotify
        .prompt_for_token(&url)
        .await
        .expect("couldn't authenticate successfully");

    // Antes de terminar guardamos el nuevo token
    info!("Saving new Access Token into cache file");

    let token: Token = spotify.get_token().lock().await.unwrap().clone().unwrap();
    token
        .write_cache(&cache_file)
        .expect("Couldn't write token into cache file:");
    info!("New Access Token saved into cache file");

    Ok(token)
}

pub async fn authorise_user() -> Result<AuthCodeSpotify, rspotify::model::error::ModelError> {
    let cache_file: &Path = Path::new(rspotify::DEFAULT_CACHE_PATH);

    let token: Token = match load_token(&cache_file) {
        Ok(token) => token,
        Err(_) => log_user(&cache_file).await.unwrap(),
    };

    let spotify: AuthCodeSpotify = AuthCodeSpotify::from_token(token);
    if spotify
        .get_token()
        .lock()
        .await
        .unwrap()
        .clone()
        .unwrap()
        .is_expired()
    {
        spotify.refresh_token().await.unwrap();
    }

    info!("AuthCodeSpotify created");
    Ok(spotify)
}
