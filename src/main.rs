use dotenv::dotenv;
mod models;
mod request;
mod user;
#[tokio::main]
async fn main() {
    dotenv().ok();

    let token: user::AccessToken = request::get_secret().await.unwrap();
    let info = request::get_artist_info("36QJpDe2go2KgaRleHCDTp?si=SNOllQDrRXmUrP5OSNG0zw", &token)
        .await
        .unwrap();
    println!("The information is the following: {:#?}", info);
}
