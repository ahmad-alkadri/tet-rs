use pancurses::{endwin, initscr, noecho, Input};
use rand::seq::SliceRandom;
use std::time::{Duration, Instant};

const BOARD_WIDTH: usize = 10;
const BOARD_HEIGHT: usize = 20;
const TICK_RATE: Duration = Duration::from_millis(500);

#[derive(Clone)]
struct Tetromino {
    shape: Vec<Vec<u8>>,
    x: usize,
    y: usize,
}

impl Tetromino {
    fn new(shape: Vec<Vec<u8>>) -> Tetromino {
        Tetromino {
            shape,
            x: BOARD_WIDTH / 2 - 1,
            y: 0,
        }
    }

    fn rotate(&self) -> Tetromino {
        let mut shape = vec![vec![0; self.shape.len()]; self.shape[0].len()];

        for (i, row) in self.shape.iter().enumerate() {
            for (j, &cell) in row.iter().enumerate() {
                shape[j][self.shape.len() - 1 - i] = cell;
            }
        }

        Tetromino {
            shape,
            x: self.x,
            y: self.y,
        }
    }
}

struct Tetris {
    board: Vec<Vec<u8>>,
    tetromino: Tetromino,
    score: usize,
}

impl Tetris {
    fn new() -> Tetris {
        let mut rng = rand::thread_rng();
        let tetrominos = vec![
            vec![vec![1, 1, 1, 1]],
            vec![vec![1, 1], vec![1, 1]],
            vec![vec![0, 1, 0], vec![1, 1, 1]],
            vec![vec![0, 1, 1], vec![1, 1, 0]],
            vec![vec![1, 1, 0], vec![0, 1, 1]],
            vec![vec![1, 0, 0], vec![1, 1, 1]],
            vec![vec![0, 0, 1], vec![1, 1, 1]],
        ];
        let shape = tetrominos.choose(&mut rng).unwrap().clone();
        let tetromino = Tetromino::new(shape);
        let board = vec![vec![0; BOARD_WIDTH]; BOARD_HEIGHT];
        Tetris {
            board,
            tetromino,
            score: 0,
        }
    }

    fn can_move(&self, dx: isize, dy: isize, tetromino: &Tetromino) -> bool {
        for (y, row) in tetromino.shape.iter().enumerate() {
            for (x, &cell) in row.iter().enumerate() {
                if cell == 0 {
                    continue;
                }
                let new_x = tetromino.x as isize + x as isize + dx;
                let new_y = tetromino.y as isize + y as isize + dy;
                if new_x < 0
                    || new_x >= BOARD_WIDTH as isize
                    || new_y < 0
                    || new_y >= BOARD_HEIGHT as isize
                    || self.board[new_y as usize][new_x as usize] != 0
                {
                    return false;
                }
            }
        }
        true
    }

    fn place_tetromino(&mut self) {
        for (y, row) in self.tetromino.shape.iter().enumerate() {
            for (x, &cell) in row.iter().enumerate() {
                if cell != 0 {
                    self.board[self.tetromino.y + y][self.tetromino.x + x] = cell;
                }
            }
        }
        self.clear_lines();
        let mut rng = rand::thread_rng();
        let tetrominos = vec![
            vec![vec![1, 1, 1, 1]],
            vec![vec![1, 1], vec![1, 1]],
            vec![vec![0, 1, 0], vec![1, 1, 1]],
            vec![vec![0, 1, 1], vec![1, 1, 0]],
            vec![vec![1, 1, 0], vec![0, 1, 1]],
            vec![vec![1, 0, 0], vec![1, 1, 1]],
            vec![vec![0, 0, 1], vec![1, 1, 1]],
        ];
        let shape = tetrominos.choose(&mut rng).unwrap().clone();
        self.tetromino = Tetromino::new(shape);
    }

    fn clear_lines(&mut self) {
        self.board.retain(|row| row.iter().any(|&cell| cell == 0));
        let cleared_lines = BOARD_HEIGHT - self.board.len();
        self.score += cleared_lines;
        self.board = vec![vec![0; BOARD_WIDTH]; cleared_lines]
            .into_iter()
            .chain(self.board.clone().into_iter())
            .collect();
    }

    fn draw_board(&self, window: &pancurses::Window) {
        window.clear();
        for (y, row) in self.board.iter().enumerate() {
            for (x, &cell) in row.iter().enumerate() {
                if cell != 0 {
                    window.mvaddstr(y as i32, (x * 2) as i32, "[]");
                }
            }
        }
        for (y, row) in self.tetromino.shape.iter().enumerate() {
            for (x, &cell) in row.iter().enumerate() {
                if cell != 0 {
                    window.mvaddstr(
                        (self.tetromino.y + y) as i32,
                        ((self.tetromino.x + x) * 2) as i32,
                        "[]",
                    );
                }
            }
        }
        window.mvaddstr(
            0,
            (BOARD_WIDTH * 2 + 2) as i32,
            &format!("Score: {}", self.score),
        );
        window.refresh();
    }
}

fn main() {
    let window = initscr();
    window.keypad(true);
    noecho();
    window.timeout(0);
    pancurses::curs_set(0);

    let mut tetris = Tetris::new();
    let mut last_tick = Instant::now();

    loop {
        let elapsed = last_tick.elapsed();
        if elapsed >= TICK_RATE {
            last_tick = Instant::now();
            if tetris.can_move(0, 1, &tetris.tetromino) {
                tetris.tetromino.y += 1;
            } else {
                tetris.place_tetromino();
                if !tetris.can_move(0, 0, &tetris.tetromino) {
                    break; // Game over
                }
            }
            tetris.draw_board(&window);
        }

        match window.getch() {
            Some(Input::KeyLeft) => {
                if tetris.can_move(-1, 0, &tetris.tetromino) {
                    tetris.tetromino.x = (tetris.tetromino.x as isize - 1) as usize;
                }
            }
            Some(Input::KeyRight) => {
                if tetris.can_move(1, 0, &tetris.tetromino) {
                    tetris.tetromino.x = (tetris.tetromino.x as isize + 1) as usize;
                }
            }
            Some(Input::KeyDown) => {
                if tetris.can_move(0, 1, &tetris.tetromino) {
                    tetris.tetromino.y += 1;
                }
            }
            Some(Input::Character('q')) => break,
            Some(Input::Character(' ')) | Some(Input::KeyUp) => {
                let rotated_tetromino = tetris.tetromino.rotate();
                if tetris.can_move(0, 0, &rotated_tetromino) {
                    tetris.tetromino = rotated_tetromino;
                }
            }
            _ => (),
        }
    }

    endwin();
}
