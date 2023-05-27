use std::{collections::hash_map::Entry, collections::HashMap};

use futures::{executor::Enter, future};
use futures_util::StreamExt;
use rspotify::{
    model::{AudioFeatures, FullArtist, FullTrack, TimeRange, TrackId},
    prelude::*,
    AuthCodeSpotify, ClientError,
};

use crate::types::GeekTrack;

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
) -> Result<HashMap<TrackId, GeekTrack>, ClientError> {
    let mut tracks: HashMap<TrackId, FullTrack> = spotify
        .current_user_top_tracks(Some(period))
        .map(|s| {
            let track = s.unwrap();
            (track.id.as_ref().unwrap().clone(), track.clone())
        })
        .collect::<HashMap<TrackId, FullTrack>>()
        .await;

    // Ids of the top songs
    let ids: Vec<TrackId<'_>> = tracks.keys().into_iter().map(|k| k.clone()).collect();

    let mut features: HashMap<TrackId, AudioFeatures> = HashMap::new();
    let r = spotify.tracks_features(ids.clone()).await.unwrap().unwrap();

    r.iter().for_each(|f| {
        features.entry(f.id.as_ref()).or_insert(f.clone());
    });

    let mut top_geek: HashMap<TrackId, GeekTrack> = HashMap::new();
    ids.iter().for_each(|id| {
        let ft = match tracks.entry(id.clone()) {
            Entry::Occupied(o) => o.get().clone(),
            Entry::Vacant(v) => panic!("Full track info not found"),
        };

        let af = match features.entry(id.clone()) {
            Entry::Occupied(o) => o.get().clone(),
            Entry::Vacant(v) => panic!("Audio features not found"),
        };

        top_geek.entry(id.clone()).or_insert(GeekTrack {
            track: ft.clone(),
            features: af.clone(),
        });
    });

    Ok(top_geek)
}

pub async fn get_track_features(
    id: TrackId<'_>,
    spotify: &AuthCodeSpotify,
) -> Result<AudioFeatures, ClientError> {
    let result = spotify.track_features(id).await.unwrap();
    Ok(result)
}

pub async fn get_tracks_features(
    ids: Vec<TrackId<'_>>,
    spotify: &AuthCodeSpotify,
) -> Result<Vec<AudioFeatures>, ClientError> {
    let track_features = spotify.tracks_features(ids.clone()).await.unwrap().unwrap();
    Ok(track_features)
}
