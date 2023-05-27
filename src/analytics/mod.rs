use std::collections::HashMap;

use rspotify::{
    model::{AudioFeatures, FullTrack},
    model::{Modality, TrackId},
    prelude::BaseClient,
    AuthCodeSpotify, ClientError,
};

use crate::types::{mode_to_string, Key, Pitches};

pub fn key_occurences(music_info: &[AudioFeatures]) -> HashMap<Key, u32> {
    let mut occurrences: HashMap<Key, u32> = HashMap::new();
    music_info.iter().for_each(|f| {
        let count = occurrences
            .entry(Key {
                mode: mode_to_string(f.mode).unwrap(),
                pitch: num::FromPrimitive::from_i32(f.key).unwrap_or(Pitches::NotFound),
            })
            .or_insert(0);
        *count += 1;
    });

    // let mut ordered: Vec<(&Key, &u32)> = occurrences.iter().collect();
    // ordered.sort_by(|a, b| b.1.cmp(a.1));
    occurrences
}
