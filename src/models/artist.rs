use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Artist {
    pub id: String,
    pub name: String,
    pub uri: String,
    pub followers: i32,
    pub external_urls: String,
    pub popularity: i32,
    pub genres: Vec<String>,
}

impl Artist {
    pub fn new(
        id: &str,
        name: &str,
        genres: Vec<String>,
        uri: &str,
        following: i32,
        urls: &str,
        popularity: i32,
    ) -> Artist {
        Artist {
            id: id.to_string(),
            name: name.to_string(),
            genres: genres,
            uri: uri.to_string(),
            followers: following,
            external_urls: urls.to_string(),
            popularity: popularity,
        }
    }
}
