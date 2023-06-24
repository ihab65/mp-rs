use ncurses::*;


type Id = usize;

const REGULAR_PAIR: i16 = 0;
const HIGHLIGHTED_PAIR: i16 = 1;

#[derive(Default)]
pub struct Ui {
    list_curr: Option<Id>,
    row: usize,
    col: usize,
}

impl Ui {
    pub fn begin(&mut self, row: usize, col: usize) {
        self.row = row;
        self.col = col;
    }

    pub fn begin_list(&mut self, id: Id) {
        assert!(
            self.list_curr.is_none(),
            "Nested lists are not allowed!"
        );
        
        self.list_curr = Some(id);
    }

    pub fn list_element(&mut self, label: &str, id: Id) {
        let id_curr = self.list_curr
            .expect("Not allowed to create list elements outside of lists");
        self.label(label , 
            match id_curr == id {
                true => HIGHLIGHTED_PAIR,
                false => REGULAR_PAIR,
            }
        )
    } 

    pub fn label(&mut self, text: &str, pair: i16) {
        mv(self.row as i32, self.col as i32);
        attron(COLOR_PAIR(pair)); 
        addstr(text);
        attroff(COLOR_PAIR(pair));
        self.row += 1;
    }

    pub fn end_list(&mut self) {
        self.list_curr = None;
    }

    // pub fn end(&mut self) {
    //     ()
    // }
}