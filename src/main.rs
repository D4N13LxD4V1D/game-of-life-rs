use std::thread;
use std::time::Duration;

#[derive(Clone)]
struct Cell {
    alive: bool,
    neighbors: usize,
}

impl Cell {
    fn new(alive: bool) -> Cell {
        Cell {
            alive,
            neighbors: 0,
        }
    }

    fn update(&mut self) {
        match (self.alive, self.neighbors) {
            (true, x) if x < 2 => self.alive = false,
            (true, x) if x == 2 || x == 3 => self.alive = true,
            (true, x) if x > 3 => self.alive = false,
            (false, 3) => self.alive = true,
            _ => (),
        }
    }
}

fn print(c: &Vec<Vec<Cell>>) {
    for row in c {
        for c in row {
            print!("{}", if c.alive { "⬛️" } else { "⬜️" });
        }
        println!();
    }

    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
}

fn update(c: &mut Vec<Vec<Cell>>) {
    let old = c.clone();
    for i in 0..c.len() {
        for j in 0..c[i as usize].len() {
            c[i][j].neighbors = 0;

            // get x and y as isize to avoid overflow
            for x in (i as isize - 1)..(i as isize + 2) {
                for y in (j as isize - 1)..(j as isize + 2) {

                    // skip self
                    if x == i as isize && y == j as isize {
                        continue;
                    }

                    // wrap around the edges
                    if old[(x + c.len() as isize) as usize % c.len()][(y + c[i].len() as isize) as usize % c[i].len()].alive {
                        c[i as usize][j as usize].neighbors += 1;
                    }
                }
            }
            c[i as usize][j as usize].update();
        }
    }
}

fn main() {
    let mut c: Vec<Vec<Cell>> = Vec::new();

    for _ in 0..30 {
        let mut row: Vec<Cell> = Vec::new();
        for _ in 0..30 {
            row.push(Cell::new(false));
        }
        c.push(row);
    }

    // glider
    c[1][2].alive = true;
    c[2][3].alive = true;
    c[3][1].alive = true;
    c[3][2].alive = true;
    c[3][3].alive = true;


    loop {
        print(&c);
        update(&mut c);
        thread::sleep(Duration::from_millis(250));
    }
}
