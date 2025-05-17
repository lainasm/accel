use evalexpr::*;
use ncurses::*;

const CELL_WIDTH: i32 = 16;

struct Cell {
    display: String,
    expression: String,
    x: i32,
    y: i32,
}

impl Cell {
    fn new(x: i32, y: i32) -> Cell {
        Cell {
            display: String::new(),
            expression: String::new(),
            x,
            y,
        }
    }

    fn draw(&self) {
        let middle_bars = "═".repeat(CELL_WIDTH as usize - 2);
        mvaddstr(self.y * 3, self.x * CELL_WIDTH, &format!("╔{middle_bars}╗")).unwrap();
        mvaddstr(self.y * 3 + 1, self.x * CELL_WIDTH, &format!("║{}║", fix_length(&self.display, CELL_WIDTH as usize - 2))).unwrap();
        mvaddstr(self.y * 3 + 2, self.x * CELL_WIDTH, &format!("╚{middle_bars}╝")).unwrap();
    }

    fn evaluate(&mut self) {
        self.display = eval_int(&self.expression).unwrap().to_string();
    }
}

fn fix_length(s: &str, l: usize) -> String {
    let mut buffer = s.to_string();
    while buffer.len() < l {
        buffer.push(' ');
    }

    if buffer.len() > l {
        buffer = buffer[0..l].to_string();
    }

    buffer
}

fn main() {
    setlocale(LcCategory::all, "").unwrap();
    initscr();

    let mut cells = Vec::new();

    for y in 0..3 {
        for x in 0..4 {
            cells.push(Cell::new(x, y));
        }
    }

    for cell in cells {
        cell.draw();
    }
    refresh();
    getch();
    endwin();
}
