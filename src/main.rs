use rspotify::model::TimeRange;

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

    println!("Top songs are: {:#?}", top_songs);
}
