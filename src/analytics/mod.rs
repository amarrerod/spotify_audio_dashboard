use rspotify::{
    model::{AudioAnalysis, TrackId},
    model::{AudioFeatures, FullTrack},
    prelude::BaseClient,
    AuthCodeSpotify, ClientError,
};

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

// #[derive(Debug, Clone)]
// pub struct TrackMusicInfo {
//     pub features: Vec<AudioFeatures>,
//     pub analysis: AudioAnalysis,
// }

pub async fn analyse_tracks(
    tracks: &[FullTrack],
    spotify: &AuthCodeSpotify,
) -> Result<Vec<AudioFeatures>, ClientError> {
    let ids: Vec<TrackId> = tracks
        .iter()
        .map(|t| t.id.as_ref().unwrap().clone())
        .collect();
    let track_features = spotify.tracks_features(ids.clone()).await.unwrap().unwrap();

    // let track_analysis: Vec<AudioAnalysis> =
    //     futures::future::join_all(ids.iter().map(|id| spotify.track_analysis(id.clone())))
    //         .await
    //         .into_iter()
    //         .collect::<Result<_, _>>()
    //         .unwrap();

    // let all_data: Vec<TrackMusicInfo> = track_features
    //     .iter()
    //     .zip(track_analysis)
    //     .map(|(f, a)| TrackMusicInfo {
    //         features: f.clone(),
    //         analysis: a.clone(),
    //     })
    //     .collect::<Vec<_>>();
    Ok(track_features)
}
