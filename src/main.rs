extern crate num;
#[macro_use]
extern crate num_derive;

mod analytics;
mod cli;
mod display;
mod request;
mod types;
mod user;

use clap::Parser;
use rspotify::model::FullArtist;

#[tokio::main]
async fn main() {
    env_logger::init();
    let args = cli::Args::parse();
    let spotify = user::authorise_user().await.unwrap();

    match args.scope {
        cli::Scope::TopArtists => {
            let data: Vec<FullArtist> = request::get_current_top_artist(&spotify, args.period)
                .await
                .unwrap();
            display::display_top_artists_as_table(&data);
        }
        cli::Scope::TopTracks => {
            let data = request::get_current_top_tracks(&spotify, args.period)
                .await
                .unwrap();

            display::display_top_tracks_as_table(&data);
        }
        _ => (),
    };
}
