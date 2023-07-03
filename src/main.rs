use ncurses::*;
use ui::{StatusBar, StatusBarPart};
use walkdir::{DirEntry, WalkDir};
use std::env;
use std::{path::Path, fs::File, process};
use std::io::BufReader;
use rodio::{Decoder, OutputStream, Sink};

mod ui;
// mod lib;

const MUSIC_FILES: [&str; 4] = ["mp3", "wav", "ogg", "flac"];
const REGULAR_PAIR: i16 = 0;
const HIGHLIGHTED_PAIR: i16 = 1;
const COLORED_PAIR: i16 = 2;

#[derive(PartialEq)]
enum Status {
    Paused,
    Playing,
    Stoped
}

fn is_music_file(entry: &DirEntry) -> bool {
    entry
        .path()
        .extension()
        .and_then(std::ffi::OsStr::to_str)
        .map_or(false, |extension| MUSIC_FILES.contains(&extension))
}

fn get_file(entry: DirEntry) -> Result<String, ()> {
    match entry.path().to_str() {
        Some(file) => {
            let file = file.to_string();
            Ok(file)
        },
        None => {
            Err(println!("ERROR: failed to convert `DirEntry` to var of type String"))
        }
    }
}

fn main() {

    // argument parsing

    // let args = lib::Cli::parse();

    // process::exit(1);

    // temporary arg parsing
    
    let mut args = env::args();
    args.next().unwrap();

    let file_path = match args.next() {
        Some(file_path) => file_path,
        None => {
            eprintln!("Usage: mp-rs <path>");
            eprintln!("ERROR: Dir or file path is not provided");
            process::exit(1);
        }
    };

    
// Logic for reading Dir's

    let mut songs = Vec::<String>::new();
    songs.push(file_path.clone());

    for entry in WalkDir::new(file_path.clone()) {
        match entry {
            Ok(entry) => {
                if is_music_file(&entry) {
                    songs.push(get_file(entry).unwrap());
                }
            }
            Err(_) => {
                eprintln!("ERROR: specifed path doesn't exist");
                process::exit(0);
            }
        }
    }

    if songs.len() == 1 && songs[0] == file_path {
        eprintln!(
            "ERROR: there is no audio files in this dir : {}", songs[0]
        );
        process::exit(0);
    }

    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    let sink = Sink::try_new(&stream_handle).unwrap();
    let curr_dir = songs.remove(0);
    
    initscr();
    noecho();
    curs_set(CURSOR_VISIBILITY::CURSOR_INVISIBLE);
    
    start_color();
    init_pair(REGULAR_PAIR, COLOR_WHITE, COLOR_BLACK);
    init_pair(HIGHLIGHTED_PAIR, COLOR_BLACK, COLOR_WHITE);
    init_pair(COLORED_PAIR, COLOR_GREEN, COLOR_YELLOW);
    init_pair(1, COLOR_BLACK, COLOR_BLUE);
    init_pair(3, COLOR_WHITE, COLOR_RED);
    init_pair(2, COLOR_WHITE, COLOR_BLACK);


    // Event loop setup :

    let mut quit = false;
    let mut ui = ui::Ui::default();
    let mut status = Status::Stoped;
    let mut index: usize = 0;

    while !quit {
        erase();
        // Ui Block
        { 
            ui.begin(0, 0);
            ui.label(&("Currently playing music from : ".to_string() + &curr_dir), REGULAR_PAIR);

            ui.begin_list(index);
            for (i, song) in songs.iter().enumerate() {
                let song = song.trim_start_matches(&(curr_dir.clone() + "/"));
                ui.list_element(&format!("{} - {}" , i + 1, song), i);
            }
            ui.end_list();

            // let path = Path::new(songs.get(index).unwrap());
            // let total_secs: f32 = ((mp3_duration::from_path(path).unwrap().as_secs()) as f32) / 60f32;
            // let mins = total_secs.floor() as u32;
            // let secs = (
            //     (total_secs - total_secs.floor()) * 60f32 
            // ).round();
            // let duration = if secs >= 10f32 {
            //     format!("  {}:{}  ", mins, secs)
            // } else {
            //     format!("  {}:0{}  ", mins, secs)
            // };

            let song_name = " ".to_owned() + songs.get(index)
                .unwrap()
                .trim_start_matches(&curr_dir)
                .trim_start_matches('/') + " ";

            let mut state = "Paused".to_string();
            match status {
                Status::Paused => {
                    state = state.clone();
                },
                Status::Playing => {
                    state = " Playing ".to_string();
                },
                Status::Stoped => {
                    state = " Stopped ".to_string();
                }
            }

            let mut statusbar = StatusBar {parent: stdscr(), parts: Vec::<StatusBarPart>::new()};
            statusbar.status_bar(3, stdscr());
            statusbar.set_text(
                0,
                state,
                if status == Status::Stoped { COLOR_PAIR(3) } else { COLOR_PAIR(1) }
            );
            statusbar.set_text(1, song_name, COLOR_PAIR(2));
            statusbar.set_text(2, " duration ".to_string(), COLOR_PAIR(3));
            statusbar.draw();
        }

        let key = getch();

        match key as u8 as char {
            '\n' => {
                let file = BufReader::new(File::open(songs.get(index).unwrap())
                    .expect("ERROR: No such file or directory"));
                let source = Decoder::new(file).unwrap();
                sink.append(source); 
                

                match status {
                    Status::Paused => {}
                    Status::Playing => {
                        sink.stop();
                        status = Status::Stoped
        
                    }
                    Status::Stoped => {
                        sink.play();
                        status = Status::Playing
                    }
                }
            },
            'w' => {
                if index > 0 {
                    index = index.saturating_sub(1);
                }
            },
            's' => {
                if index + 1 < songs.len() {
                    index += 1
                }
            },

            'q' => {
                sink.stop();
                quit = true;
                mvprintw(songs.len() as i32 + 1 , 1, "\n");
                mvprintw(songs.len() as i32 + 2 , 1, "exiting ...");
                refresh();
                std::thread::sleep(std::time::Duration::from_secs_f32(0.5));
            },

            ' ' => match status {
                Status::Paused => {
                    sink.play();
                    status = Status::Playing;
                }
                Status::Playing => {
                    sink.pause();
                    status = Status::Paused;
                },
                Status::Stoped => {}
            }

            _ => {}
        }
        refresh();
    }
    endwin();
}
