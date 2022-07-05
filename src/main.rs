use std::thread::sleep;
use std::time::Duration;

#[derive(Clone)]
enum State {
    Live,
    Dead,
}

#[derive(Clone)]
struct Cell {
    x: usize,
    y: usize,
    state: State,
    neighbors: usize,
}

impl Cell {
    fn switch(&mut self) {
        match self.state {
            State::Dead => self.state = State::Live,
            State::Live => self.state = State::Dead,
        }
    }
}

// MAIN
fn main() {
    let mut board: Vec<Vec<Cell>> = init(50);
    gun_glider(&mut board);

    loop {
        sleep(Duration::from_millis(1000));
        printer(&mut board);
        generate(&mut board);
    }
}

// AUX
fn init(n: usize) -> Vec<Vec<Cell>> {
    let mut board: Vec<Vec<Cell>> = vec![
        vec![
            Cell {
                x: 0,
                y: 0,
                state: State::Dead,
                neighbors: 0
            };
            n
        ];
        n
    ];
    for i in 0..board.len() {
        for j in 0..board[i].len() {
            board[i][j].x = j;
            board[i][j].y = i;
        }
    }

    board
}

fn printer(board: &Vec<Vec<Cell>>) {
    let mut line: String = String::new();
    for y in board {
        for x in y {
            match x.state {
                State::Dead => line += ". ",
                State::Live => line += "+ ",
            }
        }

        line += "\n";
    }

    println!("{}", line);
}

fn generate(board: &mut Vec<Vec<Cell>>) {
    // Any live cell with two or three live neighbours survives.
    // Any dead cell with three live neighbours becomes a live cell.
    // All other live cells die in the next generation. Similarly, all other dead cells stay dead.

    fn neighbours((x, y): (usize, usize), size: i32) -> Vec<(usize, usize)> {
        let mut neigh: Vec<(usize, usize)> = vec![(x, y)];
        for i in [-1, 0, 1] {
            for j in [-1, 0, 1] {
                let e = x as i32 + i;
                let w = y as i32 + j;
                if e >= 0 && e < size && w >= 0 && w < size {
                    neigh.push((e as usize, w as usize));
                }
            }
        }

        neigh
    }

    for i in 0..(board.len()) {
        for j in 0..(board[0].len()) {
            let mut live_num = 0;
            for (a, b) in neighbours((j, i), board.len() as i32) {
                if (j, i) != (a, b) {
                    match board[b][a].state {
                        State::Dead => (),
                        State::Live => live_num += 1,
                    }
                }
            }
            board[i][j].neighbors = live_num;
        }
    }

    for l in board {
        for c in l {
            match c.state {
                State::Dead => {
                    if c.neighbors == 3 {
                        c.switch()
                    }
                }
                State::Live => {
                    if c.neighbors == 2 || c.neighbors == 3 {
                        ()
                    } else {
                        c.switch()
                    }
                }
            }
        }
    }
}

fn gun_glider(board: &mut Vec<Vec<Cell>>) {
    board[10][10].state = State::Live;
    board[10][11].state = State::Live;
    board[11][11].state = State::Live;
    board[11][10].state = State::Live;

    board[10][20].state = State::Live;
    board[11][20].state = State::Live;
    board[12][20].state = State::Live;

    board[9][21].state = State::Live;
    board[8][22].state = State::Live;
    board[8][23].state = State::Live;

    board[13][21].state = State::Live;
    board[14][22].state = State::Live;
    board[14][23].state = State::Live;

    board[11][24].state = State::Live;

    board[9][25].state = State::Live;
    board[13][25].state = State::Live;
    
    board[10][26].state = State::Live;
    board[11][26].state = State::Live;
    board[12][26].state = State::Live;

    board[11][27].state = State::Live;

    board[8][30].state = State::Live;
    board[9][30].state = State::Live;
    board[10][30].state = State::Live;
    board[8][31].state = State::Live;
    board[9][31].state = State::Live;
    board[10][31].state = State::Live;
    
    board[7][32].state = State::Live;
    board[11][32].state = State::Live;

    board[6][34].state = State::Live;
    board[7][34].state = State::Live;
    board[11][34].state = State::Live;
    board[12][34].state = State::Live;

    board[8][44].state = State::Live;
    board[9][44].state = State::Live;
    board[8][45].state = State::Live;
    board[9][45].state = State::Live;
}