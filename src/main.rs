extern crate num;
#[macro_use]
extern crate num_derive;
use analytics::to_dataframe;
use polars::prelude::*;
use rspotify::model::{TimeRange, TrackId};
use std::collections::{HashMap, HashSet};

mod analytics;
mod request;
mod user;
#[tokio::main]
async fn main() {
    env_logger::init();

    let spotify = user::authorise_user().await.unwrap();

    // let top_artists = request::get_current_top_artist(&spotify, TimeRange::LongTerm)
    //     .await
    //     .unwrap();

    // println!("Top artists are: {:#?}", top_artists);

    let top_artists = request::get_current_top_artist(&spotify, TimeRange::LongTerm)
        .await
        .unwrap()
        .iter()
        .map(|a| a.name.clone())
        .collect::<Vec<String>>();

    println!("Top artists are: {:#?}", top_artists);

    let top_songs = request::get_current_top_tracks(&spotify, TimeRange::ShortTerm)
        .await
        .unwrap();

    let top_songs_names: Vec<(String, String)> = top_songs
        .iter()
        .map(|t| (t.name.clone(), t.artists[0].name.clone()))
        .collect::<Vec<(String, String)>>();

    println!("Top songs are: {:#?}", top_songs_names);

    let music_info = analytics::analyse_tracks(&top_songs, &spotify)
        .await
        .unwrap();

    let mut occurrences: HashMap<analytics::Key, u32> = HashMap::new();
    music_info.iter().for_each(|f| {
        let count = occurrences
            .entry(analytics::Key {
                mode: analytics::mode_to_string(f.mode).unwrap(),
                pitch: num::FromPrimitive::from_i32(f.key).unwrap_or(analytics::Pitches::NotFound),
            })
            .or_insert(0);
        *count += 1;
    });

    let mut ordered: Vec<(&analytics::Key, &u32)> = occurrences.iter().collect();
    ordered.sort_by(|a, b| b.1.cmp(a.1));

    println!("The keys are: {:#?}", ordered);
}
