extern crate num;
#[macro_use]
extern crate num_derive;

mod analytics;
mod cli;
mod request;
mod types;
mod user;

use clap::Parser;
use rspotify::model::TimeRange;
use std::collections::HashMap;
use types::{Key, Pitches};

#[tokio::main]
async fn main() {
    env_logger::init();
    let args = cli::Args::parse();
    let spotify = user::authorise_user().await.unwrap();

    let data: Option<Vec<String>> = match args.scope {
        cli::Scope::TopArtists => Some(
            request::get_current_top_artist(&spotify, args.period)
                .await
                .unwrap()
                .iter()
                .map(|a| a.name.clone())
                .collect(),
        ),
        cli::Scope::TopTracks => Some(
            request::get_current_top_tracks(&spotify, args.period)
                .await
                .unwrap()
                .iter()
                .map(|t| t.name.clone())
                .collect(),
        ),
        _ => None,
    };

    println!("{:#?}", data);
}
