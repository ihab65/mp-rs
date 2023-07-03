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

#[derive(Clone, Default)]
pub struct StatusBarPart {
    text: String,
    attributes: attr_t
}
impl StatusBarPart {
    pub fn get_text(&mut self) -> String {
        self.text.clone()
    }
    pub fn set_text(&mut self, text: String, attr: attr_t) {
        self.text = text;
        if attr != u32::MAX {
            self.attributes = attr
        }
    }
    pub fn get_attrs(&mut self) -> attr_t {
        self.attributes
    }
}


pub struct StatusBar{
    pub parent: WINDOW,
    pub parts: Vec<StatusBarPart>
}
impl StatusBar {
    pub fn status_bar(&mut self, part_len: usize, parent: WINDOW) {
        self.parent = parent;
        let part: StatusBarPart = StatusBarPart::default();
        self.parts = vec![part ;part_len];
    }

    pub fn set_text(&mut self, part_index: usize, msg: String, attributes: attr_t) {
        self.parts[part_index].set_text(msg, attributes)
    }

    pub fn draw(&mut self) {
        let output_row = getmaxy(self.parent) - 1;
        let mut i = 0;
        let parts_len = self.parts.len();

        wmove(self.parent, output_row, 0);

        while i < self.parts.len() {
            let part = &mut self.parts[i];

            if i == parts_len - 1 {
                wmove(self.parent, output_row, getmaxx(self.parent) - part.get_text().len() as i32);
            }

            attron(part.get_attrs());
            wprintw(self.parent, &part.get_text());
            attroff(part.get_attrs());
            i += 1;
        }
    }
}