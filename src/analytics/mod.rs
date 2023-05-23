use polars::prelude::*;
use rspotify::{
    model::{AudioFeatures, FullTrack},
    model::{Modality, TrackId},
    prelude::BaseClient,
    AuthCodeSpotify, ClientError,
};
use std::io::Cursor;

#[derive(FromPrimitive, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Pitches {
    NotFound = -1,
    C = 0,
    Db = 1,
    D = 2,
    Eb = 3,
    E = 4,
    F = 5,
    Gb = 6,
    G = 7,
    Ab = 8,
    A = 9,
    Bb = 10,
    B = 11,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Key {
    pub mode: String,
    pub pitch: Pitches,
}

pub fn mode_to_string(m: Modality) -> Option<String> {
    match m {
        Modality::Major => Some(String::from("Major")),
        Modality::Minor => Some(String::from("Minor")),
        _ => Some(String::from("Not Found")),
    }
}

pub fn to_dataframe(tracks: &[AudioFeatures]) -> Result<DataFrame, PolarsError> {
    let json = serde_json::to_string(&tracks).unwrap();
    let cursor = Cursor::new(json);
    let df = JsonReader::new(cursor).finish()?;
    Ok(df)
}

pub async fn analyse_tracks(
    tracks: &[FullTrack],
    spotify: &AuthCodeSpotify,
) -> Result<Vec<AudioFeatures>, ClientError> {
    let ids: Vec<TrackId> = tracks
        .iter()
        .map(|t| t.id.as_ref().unwrap().clone())
        .collect();
    let track_features = spotify.tracks_features(ids.clone()).await.unwrap().unwrap();
    Ok(track_features)
}
