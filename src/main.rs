#[macro_use]
extern crate clap;
extern crate reqwest;
extern crate serde_derive;
use clap::App;
use reqwest::Error;
use serde::Deserialize;
use std::process::Command;

#[derive(Deserialize, Debug, Clone)]
struct Channels {
    channels: Vec<Channel>,
}

#[derive(Deserialize, Debug, Clone)]
struct Channel {
    id: String,
    title: String,
    description: String,
    dj: String,
    genre: String,
    twitter: String,
    listeners: String,
    playlists: Vec<Playlist>,
    lastPlaying: String
}

#[derive(Deserialize, Debug, Clone)]
struct Playlist {
    url: String,
    format: String,
    quality: String
}

fn main() {
    let channels: Vec<Channel> = get_channels_from_api().unwrap();
    let yaml = load_yaml!("cli.yml");
    let matches = App::from_yaml(yaml).get_matches();

    if matches.is_present("channels") {
        for channel in &channels {
            println!("{}, listeners: {}", channel.title, channel.listeners);
        }
    }

    if matches.is_present("last") {
        let last_played_channel = get_channel_by_title(&channels, matches.value_of("last").unwrap().to_string());
        println!("{} last played: {}", last_played_channel.title, last_played_channel.lastPlaying);
    }

    if matches.is_present("play") {
        let play_channel = get_channel_by_title(&channels, matches.value_of("play").unwrap().to_string());
        println!("{:?}", play_channel);
        Command::new("mpv")
            .arg("--no-config")
            .arg(&play_channel.playlists[0].url).spawn();
    }
}

fn get_channels_from_api() -> Result<Vec<Channel>, Error> {
    let url = format!("https://somafm.com/channels.json");
    let mut response = reqwest::get(&url)?;
    let channels: Channels = response.json()?;
    Ok(channels.channels)
}

fn get_channel_by_title(channels: &[Channel], title: String) -> &Channel {
    for channel in channels {
        if channel.title == title {
            return channel;
        }
    }
    return &channels[0];
}
