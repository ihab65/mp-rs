use ncurses::*;
use std::fs::File;
use std::io::BufReader;
use rodio::{Decoder, OutputStream, Sink};

const REGULAR_PAIR: i16 = 0;
const HIGHLIGHTED_PAIR: i16 = 1;

enum Status {
    Paused,
    Playing
}
impl Status {
    fn toggle(&self) -> Self {
        match self {
            Status::Playing => Status::Paused,
            Status::Paused => Status::Playing,
        }
    }
}

#[derive(Default)]
struct Ui {
    row: usize,
    col: usize,
}
impl Ui {
    fn begin(&mut self, row: usize, col: usize) {
        self.row = row;
        self.col = col;
    }
    fn label(&mut self, text: &str, pair: i16) {
        mv(self.row as i32, self.col as i32);
        attron(COLOR_PAIR(pair)); 
        addstr(text);
        attroff(COLOR_PAIR(pair));
        self.row += 1;
    }
}

fn main() {
    
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    let sink = Sink::try_new(&stream_handle).unwrap();
    

    initscr();
    start_color();
    noecho();
    curs_set(CURSOR_VISIBILITY::CURSOR_INVISIBLE);

    init_pair(REGULAR_PAIR, COLOR_WHITE, COLOR_BLACK);
    init_pair(HIGHLIGHTED_PAIR, COLOR_BLACK, COLOR_WHITE);

    let mut quit = false;
    let mut ui = Ui::default();
    let mut status = Status::Playing;

    while !quit {
        erase();
        ui.begin(0, 0);
        match status {
            Status::Paused  => ui.label(" pause [play]", REGULAR_PAIR),
            Status::Playing => ui.label("[pause] play ", REGULAR_PAIR)
        }

        let key = getch();

        match key as u8 as char {
            '\n' => {
                let file = BufReader::new(File::open("test.mp3")
                    .expect("ERROR: No such file or directory"));
                let source = Decoder::new(file).unwrap();
                sink.append(source); 
            },

            'q' => {
                sink.stop();
                quit = true
            },

            ' ' => match status {
                Status::Paused => {
                    sink.play();
                    status = status.toggle();
                }
                Status::Playing => {
                    sink.pause();
                    status = status.toggle();
                }
            }

            _ => {}
        }
        refresh();
    }
    endwin();
}
