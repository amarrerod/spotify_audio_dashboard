extern crate num;
#[macro_use]
extern crate num_derive;
use std::{
    collections::{HashMap, HashSet},
    println,
};

use rspotify::model::{TimeRange, TrackId};

mod analytics;
mod request;
mod user;
#[tokio::main]
async fn main() {
    env_logger::init();

    let spotify = user::authorise_user().await.unwrap();

    let top_artists = request::get_current_top_artist(&spotify, TimeRange::LongTerm)
        .await
        .unwrap();

    println!("Top artists are: {:#?}", top_artists);

    let top_songs = request::get_current_top_tracks(&spotify, TimeRange::ShortTerm)
        .await
        .unwrap();
    let ids: Vec<TrackId> = top_songs
        .iter()
        .map(|t| t.id.as_ref().unwrap().clone())
        .collect();

    println!("Ids of songs are: {:?}", ids);
    println!("Top songs are: {:?}", top_songs);

    let music_info = analytics::analyse_tracks(&top_songs, &spotify)
        .await
        .unwrap();
    println!("Music information: {:#?}", music_info);

    let mut occurrences: HashMap<analytics::Pitches, u32> = HashMap::new();
    music_info.iter().for_each(|f| {
        let count = occurrences
            .entry(num::FromPrimitive::from_i32(f.key).unwrap_or(analytics::Pitches::NotFound))
            .or_insert(0);
        *count += 1;
    });

    println!("The keys are: {:#?}", occurrences);
}
