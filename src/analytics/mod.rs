use rspotify::{
    model::{AudioAnalysis, TrackId},
    model::{AudioFeatures, FullTrack},
    prelude::BaseClient,
    AuthCodeSpotify, ClientError,
};

#[derive(Debug, Clone)]
pub struct TrackMusicInfo {
    pub features: Vec<AudioFeatures>,
    pub analysis: AudioAnalysis,
}

pub async fn analyse_tracks(
    tracks: &[FullTrack],
    spotify: &AuthCodeSpotify,
) -> Result<Vec<TrackMusicInfo>, ClientError> {
    let ids: Vec<TrackId> = tracks
        .iter()
        .map(|t| t.id.as_ref().unwrap().clone())
        .collect();
    let track_features = spotify.tracks_features(ids.clone()).await.unwrap();

    let track_analysis: Vec<AudioAnalysis> =
        futures::future::join_all(ids.iter().map(|id| spotify.track_analysis(id.clone())))
            .await
            .into_iter()
            .collect::<Result<_, _>>()
            .unwrap();

    let all_data: Vec<TrackMusicInfo> = track_features
        .iter()
        .zip(track_analysis)
        .map(|(f, a)| TrackMusicInfo {
            features: f.clone(),
            analysis: a.clone(),
        })
        .collect::<Vec<_>>();
    Ok(all_data)
}
