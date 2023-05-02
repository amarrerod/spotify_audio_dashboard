use dotenv::dotenv;
mod models;
mod request;
mod user;
#[tokio::main]
async fn main() {
    dotenv().ok();

    let token: user::AccessToken = request::get_secret().await.unwrap();
    let info = request::get_artist_info("4Z8W4fKeB5YxbusRsdQVPb", &token)
        .await
        .unwrap();
    println!("The information is the following: {:#?}", info);
}
