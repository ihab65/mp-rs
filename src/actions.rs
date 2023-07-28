use std::{path::PathBuf, process};
use walkdir::WalkDir;
use clap::Args;

#[derive(Args, Debug)]
pub struct PathToPlay {
    path: String,
}

const MUSIC_FILES: [&str; 1] = ["mp3"];

fn is_music_file(entry: &PathBuf) -> bool {
    entry
        .extension()
        .and_then(std::ffi::OsStr::to_str)
        .map_or(false, |extension| MUSIC_FILES.contains(&extension))
}

pub fn check_path(PathToPlay { path }: &PathToPlay) -> Vec<PathBuf> {
    let path = PathBuf::new().join(path).canonicalize().unwrap();
    let mut list = Vec::<PathBuf>::new();

    if path.exists() {
        if path.is_dir() {
            list = dir_case(path);
        } else if path.is_file() {
            list = file_case(path)
        }
    } else {
        println!("ERROR: the path doesn't exist")
    };

    list
}

pub fn dir_case(path: PathBuf) -> Vec<PathBuf> {
    let list = get_songs_from_dir(path.clone());
    if list.len() == 1 && list[0] == path.clone() {
        eprintln!("ERROR: there is no audio files in : {}", list[0].display());
        process::exit(0);
    }
    list
}

pub fn file_case(path: PathBuf) -> Vec<PathBuf> {
    let mut list = Vec::<PathBuf>::new();

    if is_music_file(&path) {
        list.push(path);
    } else {
        println!("ERROR: this path isn't a audio file");
    }
    list
}

pub fn get_songs_from_dir(path: PathBuf) -> Vec<PathBuf> {
    let mut songs = Vec::<PathBuf>::new();
    songs.push(path.clone().canonicalize().unwrap());

    for entry in WalkDir::new(path) {
        match entry {
            Ok(entry) => if is_music_file(&entry.path().to_path_buf()) {
                let song = entry.path().canonicalize().unwrap();
                songs.push(song.clone());
            },
            Err(_) => println!("ERROR: Something went wrong when reading the directory")
        }
    };
    songs
}