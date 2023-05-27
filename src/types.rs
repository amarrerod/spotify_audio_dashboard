use rspotify::model::{AudioFeatures, FullTrack, Modality};
use std::fmt;
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

impl fmt::Display for Pitches {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Pitches::C => write!(f, "C"),
            Pitches::Db => write!(f, "C#"),
            Pitches::D => write!(f, "D"),
            Pitches::Eb => write!(f, "Eb"),
            Pitches::E => write!(f, "E"),
            Pitches::F => write!(f, "F"),
            Pitches::Gb => write!(f, "F#"),
            Pitches::G => write!(f, "G"),
            Pitches::Ab => write!(f, "G#"),
            Pitches::A => write!(f, "A"),
            Pitches::Bb => write!(f, "Bb"),
            Pitches::B => write!(f, "B"),
            Pitches::NotFound => write!(f, "Not Found"),
        }
    }
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

pub struct GeekTrack {
    pub track: FullTrack,
    pub features: AudioFeatures,
}
