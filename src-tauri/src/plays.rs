use eyre::{eyre, Result};
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::path;

/// A struct that represents one entry of an end_song.json file. This struct represents a single "play" of
/// a single song/podcast.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct PlayItem {
    pub conn_country: Option<String>,
    pub episode_name: Option<String>,
    pub episode_show_name: Option<String>,
    pub incognito_mode: Option<bool>,
    pub ip_addr_decrypted: Option<String>,
    pub master_metadata_album_album_name: Option<String>,
    pub master_metadata_album_artist_name: Option<String>,
    pub master_metadata_track_name: Option<String>,
    pub ms_played: Option<u64>,
    pub offline: Option<bool>,
    pub offline_timestamp: Option<u64>,
    pub platform: Option<String>,
    pub reason_end: Option<String>,
    pub reason_start: Option<String>,
    pub shuffle: Option<bool>,
    pub skipped: Option<bool>,
    pub spotify_episode_uri: Option<String>,
    pub spotify_track_uri: Option<String>,
    pub ts: Option<String>,
    pub user_agent_decrypted: Option<String>,
    pub username: Option<String>,
}

fn get_song_plays_from_file(file_path: &path::PathBuf) -> Result<Vec<PlayItem>> {
    let input_file = File::open(file_path)?;
    let mut buf_reader = BufReader::new(input_file);
    let mut contents = String::new();

    buf_reader.read_to_string(&mut contents)?;
    let song_play_data: Vec<PlayItem> = serde_json::from_str(&contents)?;

    Ok(song_play_data)
}

fn get_song_history_file_paths(base_path: &path::PathBuf) -> Result<Vec<path::PathBuf>> {
    let mut file_paths = vec![];

    let mut i = 0;

    loop {
        let path_str = format!("endsong_{}.json", i);
        let next_file_name = path::Path::new(&path_str);
        let mut next_data_file_path = base_path.clone();
        next_data_file_path.push(next_file_name);

        if next_data_file_path.exists() {
            file_paths.push(next_data_file_path);
        } else {
            break;
        }

        i += 1;
    }

    // If no files are read, return an error. Else, return the file paths
    if i == 0 {
        Err(eyre!(
            "Invalid folder. Could not find any streaming data files."
        ))
    } else {
        Ok(file_paths)
    }
}

pub fn extract_plays_from_path(base_path: &path::PathBuf) -> Result<Vec<PlayItem>> {
    // Get all of the song history file paths
    match get_song_history_file_paths(base_path) {
        Err(e) => Err(e),
        Ok(file_paths) => {
            // Vec to hold all of the song play instances from all JSON files combined
            let mut all_song_plays: Vec<PlayItem> = vec![];

            // Extract a Vec of SongPlay instances from all of the JSON files
            // TODO: Load data on separate thread?
            for path in file_paths.iter() {
                if let Ok(mut single_file_song_plays) = get_song_plays_from_file(path) {
                    all_song_plays.append(&mut single_file_song_plays);
                } else {
                    // If the file can't be opened, then all of the data is loaded
                    // TODO: Determine if this is the right behavior
                    break;
                }
            }

            // Return the song plays
            Ok(all_song_plays)
        }
    }
}
