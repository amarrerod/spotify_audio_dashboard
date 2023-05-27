use rspotify::model::Modality;

#[derive(FromPrimitive, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Pitches {
    NotFound = -1,
    C = 0,
    Db = 1,
    D = 2,
    Eb = 3,
    E = 4,
    F = 5,
    Gb = 6,
    G = 7,
    Ab = 8,
    A = 9,
    Bb = 10,
    B = 11,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Key {
    pub mode: String,
    pub pitch: Pitches,
}

pub fn mode_to_string(m: Modality) -> Option<String> {
    match m {
        Modality::Major => Some(String::from("M")),
        Modality::Minor => Some(String::from("m")),
        _ => Some(String::from("Not Found")),
    }
}
