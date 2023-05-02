use serde::Deserialize;

#[derive(Deserialize, Debug, Default)]
pub struct AccessToken {
    pub access_token: String,
    pub token_type: String,
    pub expires_in: i32,
}
#[derive(Debug, Default)]
pub struct User {
    pub id: String,
    pub name: String,
    pub access_token: AccessToken,
}

impl User {
    pub fn new(id: &str, name: &str, access_token: AccessToken) -> User {
        User {
            id: id.to_string(),
            name: name.to_string(),
            access_token,
        }
    }
}

impl AccessToken {
    pub fn is_valid(&self) -> bool {
        todo!()
    }
}
