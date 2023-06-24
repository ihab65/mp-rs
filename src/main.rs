use ncurses::*;
use walkdir::{DirEntry, WalkDir};
use std::env;
use std::{fs::File, process};
use std::io::BufReader;
use rodio::{Decoder, OutputStream, Sink};

mod ui;
const REGULAR_PAIR: i16 = 0;
const HIGHLIGHTED_PAIR: i16 = 1;

// use walkdir::DirEntry;


const COLORED_PAIR: i16 = 2;

enum Status {
    Paused,
    Playing,
    Stoped
}
// Logic for reading Dir's
// --- --- --- --- --- --- --- --- --- --- --- ---
// fn get_path(paths_lsit: Vec<String>) {
//     todo!()
// }

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
    
    let mut args = env::args();
    args.next().unwrap();

    let file_path = match args.next() {
        Some(file_path) => file_path,
        None => {
            eprintln!("Usage: mp-rs <file-path>");
            eprintln!("ERROR: Dir or file path is not provided");
            process::exit(1);
        }
    };

    // Logic for reading Dir's
    // --- --- --- --- --- --- --- --- --- --- --- ---
    let mut songs = Vec::<String>::new();

    for entry in WalkDir::new(file_path.clone()) {
        match entry {
            Ok(entry) => {
                songs.push(
                    get_file(entry).unwrap()
                );
            },
            Err(_) => eprintln!("ERROR: specifed path doesn't exist")
        }
    }

    // process::exit(1);

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

    let mut quit = false;
    let mut ui = ui::Ui::default();
    let mut status = Status::Stoped;
    let mut index: usize = 0;

    while !quit {
        erase();
        // Ui Block
        { 
            ui.begin(0, 0);
            match status {
                Status::Paused => {
                    ui.label(" Stoped  Playing  [Paused]", REGULAR_PAIR);
                },
                Status::Playing => {
                    ui.label(" Stoped [Playing] Paused ", REGULAR_PAIR);
                },
                Status::Stoped => {
                    ui.label("[Stoped] Playing  Paused ", REGULAR_PAIR)
                }
            }
            
            ui.label(&curr_dir, REGULAR_PAIR);

            ui.begin_list(index);
            for (i, song) in songs.iter().enumerate() {
                let song = song.trim_start_matches(&(curr_dir.clone() + "/"));
                ui.list_element(&format!("{} - {}" , i + 1, song), i);
            }
            ui.end_list();

            attron(COLOR_PAIR(HIGHLIGHTED_PAIR));
            mvprintw(LINES() - 1 ,0 ," press Enter to launch ");
            attroff(COLOR_PAIR(HIGHLIGHTED_PAIR));

            attron(COLOR_PAIR(HIGHLIGHTED_PAIR)); 
            mvprintw(LINES() - 1 ," press Enter to launch ".len() as i32 + 1 ," press q to quit ");
            attroff(COLOR_PAIR(HIGHLIGHTED_PAIR));

            attron(COLOR_PAIR(HIGHLIGHTED_PAIR)); 
            mvprintw(LINES() - 1," press q to quit ".len() as i32 + " press Enter to launch ".len() as i32 + 2 ," press Space to pause/play ");
            attroff(COLOR_PAIR(HIGHLIGHTED_PAIR));
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

            // Movement :
            'w' => {
                if index > 0 {
                    index -= 1
                }
            },
            's' => {
                if index + 1 < songs.len() {
                    index += 1
                }
            },

            'q' => {
                sink.stop();
                quit = true
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
