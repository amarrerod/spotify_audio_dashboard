use futures_util::StreamExt;

use rspotify::{
    model::{FullArtist, FullTrack, TimeRange},
    prelude::*,
    AuthCodeSpotify, ClientError,
};

pub async fn get_current_top_artist(
    spotify: &AuthCodeSpotify,
    period: rspotify::model::TimeRange,
) -> Result<Vec<FullArtist>, ClientError> {
    let artist_top = spotify
        .current_user_top_artists(Some(period))
        .map(|s| s.unwrap())
        .collect::<Vec<FullArtist>>()
        .await;

    Ok(artist_top)
}

pub async fn get_current_top_tracks(
    spotify: &AuthCodeSpotify,
    period: TimeRange,
) -> Result<Vec<FullTrack>, ClientError> {
    let top_songs = spotify
        .current_user_top_tracks(Some(period))
        .map(|s| s.unwrap())
        .collect::<Vec<FullTrack>>()
        .await;
    Ok(top_songs)
}
