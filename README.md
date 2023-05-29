# Spotify Audio Trends 

[![Rust](https://github.com/amarrerod/spotify_audio_dashboard/actions/workflows/rust.yml/badge.svg?branch=master)](https://github.com/amarrerod/spotify_audio_dashboard/actions/workflows/rust.yml)

Get the audio information from your top tracks and artists.

![](img/top_tracks_medium_term.png)


## Dependencies

- [RSpotify](https://github.com/ramsayleung/rspotify/)
- [Tabled](https://github.com/zhiburt/tabled)
- [Clap](https://github.com/clap-rs/clap)


## How to use

1. Create an application in [Spotify for Developers](https://developer.spotify.com/)
2. Create an .env file with the following items
    - RSPOTIFY_CLIENT_ID
    - RSPOTIFY_CLIENT_SECRET
    - RSPOTIFY_REDIRECT_URI
3. Follow the AuthCodeSpotify [tutorial](https://docs.rs/rspotify/latest/rspotify/struct.AuthCodeSpotify.html).
4. Run the application
