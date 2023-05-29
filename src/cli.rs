use clap::Parser;
use rspotify::model::TimeRange;

#[derive(Debug, Clone, PartialEq)]
pub enum Scope {
    TopArtists,
    TopTracks,
    Both,
}

fn parse_scope(input: &str) -> Result<Scope, std::io::Error> {
    match input.to_uppercase().as_str() {
        "ARTISTS" => Ok(Scope::TopArtists),
        "TRACKS" => Ok(Scope::TopTracks),
        "BOTH" => Ok(Scope::Both),
        _ => Err(std::io::Error::new(
            std::io::ErrorKind::InvalidInput,
            "Invalid scope given. \n Expected <artists> <tracks> <both>",
        )),
    }
}

fn parse_time_range(input: &str) -> Result<TimeRange, std::io::Error> {
    match input {
        "long-term" => Ok(TimeRange::LongTerm),
        "medium-term" => Ok(TimeRange::MediumTerm),
        "short-term" => Ok(TimeRange::ShortTerm),
        _ => Err(std::io::Error::new(
            std::io::ErrorKind::InvalidInput,
            "Invalid time range given. \n Expected <long-term> <medium-term> <short-term>",
        )),
    }
}

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    // Type of the information to retrieve
    #[arg(short, long, value_parser = parse_scope )]
    pub scope: Scope,
    // Period of type considered
    #[arg(short, long, value_parser = parse_time_range )]
    pub period: TimeRange,
    // Key summary
    #[arg(short, long)]
    pub key_summary: Option<bool>,
}
