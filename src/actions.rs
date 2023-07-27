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

pub fn check_path(PathToPlay { path }: &PathToPlay) {
    let path = PathBuf::new().join(path);

    if path.exists() { // path exists
        println!("{:?}", path.canonicalize().unwrap());

        if path.is_dir() { // path is a Dir
            println!("the path you spicified is a dir");
            let list = dir_case(path.clone());
            
            if list.len() == 1 && list[0] == path.clone().canonicalize().unwrap() {
                eprintln!("ERROR: there is no audio files in : {}", list[0].display());
                process::exit(0);
            }

        } else if path.is_file() { 
            println!("this is a path to a file");
            if is_music_file(&path) {
                println!("this is a supported file type")
            } else {
                println!("ERROR: this is not a supported file")
            }
        }
    } else {
        println!("ERROR: the specified path doesn't exist")
    }
}

pub fn dir_case(path: PathBuf) -> Vec<PathBuf> {
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