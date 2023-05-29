use crate::types::Key;
use rspotify::model::{FullArtist, TrackId};
use std::collections::HashMap;
use tabled::settings::locator::ByColumnName;
use tabled::settings::{object::Segment, style::BorderColor, Alignment, Color, Modify};
use tabled::{builder::Builder, settings::Style};

use crate::types::{mode_to_string, GeekTrack, Pitches};

pub fn display_top_tracks_as_table(data: &HashMap<TrackId, GeekTrack>) {
    // Creates the Table
    let mut builder = Builder::default();
    builder.set_header([
        "Artist",
        "Track Name",
        "Key",
        "Tempo",
        "Time Signature",
        "Liveness",
        "Popularity",
    ]);

    for (_, v) in data.iter() {
        let (mode, pitch): (String, Pitches) = (
            mode_to_string(v.features.mode).unwrap(),
            num::FromPrimitive::from_i32(v.features.key).unwrap_or(Pitches::NotFound),
        );

        builder.push_record([
            v.track.artists[0].name.clone(),
            v.track.name.clone(),
            String::from(pitch.to_string() + &mode),
            v.features.tempo.to_string(),
            v.features.time_signature.to_string(),
            v.features.liveness.to_string(),
            v.track.popularity.to_string(),
        ]);
    }
    // Shows the table
    let mut table = builder.build();
    table.with(Style::rounded());

    table.with(
        BorderColor::default()
            .top(Color::FG_BRIGHT_CYAN)
            .bottom(Color::FG_BRIGHT_CYAN)
            .right(Color::FG_BRIGHT_CYAN)
            .left(Color::FG_BRIGHT_CYAN)
            .corner_bottom_left(Color::FG_BRIGHT_CYAN)
            .corner_bottom_right(Color::FG_BRIGHT_CYAN)
            .corner_top_left(Color::FG_BRIGHT_CYAN)
            .corner_top_right(Color::FG_BRIGHT_CYAN),
    );
    table.with(
        Modify::new(Segment::all())
            .with(Alignment::right())
            .with(Alignment::top()),
    );
    table.with(
        Modify::new(ByColumnName::new("Artist"))
            .with(Alignment::center())
            .with(Alignment::top()),
    );
    table.with(
        Modify::new(ByColumnName::new("Track Name"))
            .with(Alignment::center())
            .with(Alignment::top()),
    );
    println!("{}", table);
}

fn capitalize_genres(genre: &str) -> String {
    let tokens = genre.split(" ");
    tokens
        .into_iter()
        .map(|t| {
            let mut c = t.chars();
            match c.next() {
                None => String::new(),
                Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
            }
        })
        .collect::<Vec<String>>()
        .join(" ")
}

pub fn display_top_artists_as_table(data: &Vec<FullArtist>) {
    // Creates the Table
    let mut builder = Builder::default();
    builder.set_header(["Artist", "Genres", "Popularity"]);

    for artist in data.iter() {
        let genres = artist
            .genres
            .iter()
            .map(|g| capitalize_genres(g))
            .collect::<Vec<String>>()
            .join(", ");
        builder.push_record([artist.name.clone(), genres, artist.popularity.to_string()]);
    }
    // Shows the table
    let mut table = builder.build();
    table.with(Style::rounded());

    table.with(
        BorderColor::default()
            .top(Color::FG_BRIGHT_CYAN)
            .bottom(Color::FG_BRIGHT_CYAN)
            .right(Color::FG_BRIGHT_CYAN)
            .left(Color::FG_BRIGHT_CYAN)
            .corner_bottom_left(Color::FG_BRIGHT_CYAN)
            .corner_bottom_right(Color::FG_BRIGHT_CYAN)
            .corner_top_left(Color::FG_BRIGHT_CYAN)
            .corner_top_right(Color::FG_BRIGHT_CYAN),
    );
    table.with(
        Modify::new(Segment::all())
            .with(Alignment::right())
            .with(Alignment::top()),
    );
    table.with(
        Modify::new(ByColumnName::new("Artist"))
            .with(Alignment::center())
            .with(Alignment::top()),
    );
    table.with(
        Modify::new(ByColumnName::new("Genres"))
            .with(Alignment::center())
            .with(Alignment::top()),
    );
    println!("{}", table);
}

pub fn display_top_keys(data: &HashMap<Key, u32>) {
    // Creates the Table
    let mut builder = Builder::default();
    builder.set_header(["Key", "Occurences"]);

    let mut ordered: Vec<(&Key, &u32)> = data.iter().collect();
    ordered.sort_by(|a, b| b.1.cmp(a.1));

    for (k, occ) in ordered.iter() {
        builder.push_record([k.to_string(), occ.to_string()]);
    }
    // Shows the table
    let mut table = builder.build();
    table.with(Style::rounded());

    table.with(
        BorderColor::default()
            .top(Color::FG_BRIGHT_CYAN)
            .bottom(Color::FG_BRIGHT_CYAN)
            .right(Color::FG_BRIGHT_CYAN)
            .left(Color::FG_BRIGHT_CYAN)
            .corner_bottom_left(Color::FG_BRIGHT_CYAN)
            .corner_bottom_right(Color::FG_BRIGHT_CYAN)
            .corner_top_left(Color::FG_BRIGHT_CYAN)
            .corner_top_right(Color::FG_BRIGHT_CYAN),
    );
    table.with(
        Modify::new(Segment::all())
            .with(Alignment::right())
            .with(Alignment::top()),
    );
    table.with(
        Modify::new(ByColumnName::new("Key"))
            .with(Alignment::center())
            .with(Alignment::top()),
    );
    table.with(
        Modify::new(ByColumnName::new("Occurences"))
            .with(Alignment::center())
            .with(Alignment::top()),
    );
    println!("{}", table);
}
