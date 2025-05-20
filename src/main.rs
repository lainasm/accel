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
    noecho();

    let mut cursor_x = 0;
    let mut cursor_y = 0;

    let cell_count_x: usize = 6;
    let cell_count_y: usize = 5;

    let mut entering = String::new();
    let mut is_entering = false;
    let mut cells = Vec::new();

    let coord_to_index = |x: usize, y: usize| {
        y * cell_count_x + x
    };

    for y in 0..cell_count_y {
        for x in 0..cell_count_x {
            cells.push(Cell::new(x as i32, y as i32));
        }
    }

    for cell in &cells {
        cell.draw();
    }
    refresh();

    loop {
        mv(cursor_y * 3 + 1, cursor_x * CELL_WIDTH + 1);

        let c = getch();

        if is_entering {
            if c == 10 {
                is_entering = false;
                cells[coord_to_index(cursor_x as usize, cursor_y as usize)].expression = entering.clone();
                cells[coord_to_index(cursor_x as usize, cursor_y as usize)].evaluate();
                entering.clear();
            } else {
                if c == 127 {
                    entering.pop();
                } else {
                    entering.push(char::from_u32(c as u32).unwrap());
                }
                cells[coord_to_index(cursor_x as usize, cursor_y as usize)].display = entering.clone();
            }
        } else {
            if c == 'q' as i32 {
                break;
            }

            if c == 'j' as i32 {
                cursor_y += 1;
            }
            if c == 'k' as i32 {
                cursor_y -= 1;
            }
            if c == 'h' as i32 {
                cursor_x -= 1;
            }
            if c == 'l' as i32 {
                cursor_x += 1;
            }

            if c == 10 as i32 {
                is_entering = true;
            }
        }

        for cell in &cells {
            cell.draw();
        }
        refresh();
    }
    endwin();
}
