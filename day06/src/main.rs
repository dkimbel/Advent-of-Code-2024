use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Copy, Clone)]
enum Tile {
    Empty,
    Barrier,
}

impl Tile {
    fn from_char(c: char) -> Option<Self> {
        match c {
            '.' => Some(Tile::Empty),
            '#' => Some(Tile::Barrier),
            _ => None
        }
    }
}

#[derive(Copy, Clone)]
enum Direction {
    Up,
    Right,
    Down,
    Left
}

impl Direction {
    fn from_char(c: char) -> Option<Self> {
        match c {
            '^' => Some(Direction::Up),
            '>' => Some(Direction::Right),
            'v' => Some(Direction::Down),
            '<' => Some(Direction::Left),
            _ => None
        }
    }

    fn next(self) -> Self {
        match self {
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
        }
    }
}

#[derive(Copy, Clone)]
struct GuardState {
    x: usize,
    y: usize,
    direction: Direction,
}

impl GuardState {
    fn next(self, board: &Board) -> Option<Self> {
        // return None if off-board
        let max_y = board.len() - 1;
        let max_x = board[0].len() - 1;

        let intended_next_coords: Option<(usize, usize)> = match self.direction {
            Direction::Up => {
                if self.y > 0 {
                    Some((self.x, self.y - 1))
                } else {
                    None
                }
            }
            Direction::Right => {
                if self.x < max_x {
                    Some((self.x + 1, self.y))
                } else {
                    None
                }
            }
            Direction::Down => {
                if self.y < max_y {
                    Some((self.x, self.y + 1))
                } else {
                    None
                }
            }
            Direction::Left => {
                if self.x > 0 {
                    Some((self.x - 1, self.y))
                } else {
                    None
                }
            }
        };

        if intended_next_coords.is_none() {
            None
        } else {
            let (intended_x, intended_y) = intended_next_coords.unwrap();
            let intended_tile = &board[intended_y][intended_x];
            match intended_tile {
                Tile::Empty => Some(Self {
                    direction: self.direction,
                    x: intended_x,
                    y: intended_y,
                }),
                Tile::Barrier => Some(Self {
                    direction: self.direction.next(),
                    x: self.x,
                    y: self.y,
                })
            }
        }
    }
}

type Board = Vec<Vec<Tile>>;

struct BoardLayout {
    board: Board,
    initial_guard_state: GuardState,
}

impl BoardLayout {
    fn from_file_path(path: &str) -> Self {
        let file = File::open(path).unwrap();
        let reader = BufReader::new(file);

        let mut board: Board = Vec::new();
        let mut maybe_guard_state: Option<GuardState> = None;
        let mut y = 0;

        for line in reader.lines() {
            let board_row = line.unwrap().chars().enumerate().map(|(x, c)| {
                let maybe_tile = Tile::from_char(c);
                if maybe_tile.is_some() {
                    maybe_tile.unwrap()
                } else {
                    let direction = Direction::from_char(c).unwrap();
                    maybe_guard_state = Some(GuardState {
                        x,
                        y,
                        direction,
                    });
                    Tile::Empty // guard's tile has no barrier
                }
            }).collect::<Vec<Tile>>();
            board.push(board_row);
            y += 1;
        }

        if maybe_guard_state.is_none() {
            panic!("Failed to find guard while parsing board");
        }

        Self {
            board,
            initial_guard_state: maybe_guard_state.unwrap()
        }
    }
}

fn solve_part_1(board_layout: &BoardLayout) {
    let num_rows = board_layout.board.len();
    let num_cols = board_layout.board[0].len();
    let mut visited: Vec<Vec<bool>> = vec![vec![false; num_cols]; num_rows];

    let mut maybe_guard_state: Option<GuardState> = Some(board_layout.initial_guard_state);
    while let Some(guard_state) = maybe_guard_state  {
        visited[guard_state.y][guard_state.x] = true;
        maybe_guard_state = guard_state.next(&board_layout.board);
    }
    let num_tiles_visited = visited.iter().map(|row| row.iter().map(|bool| match bool {
        false => 0,
        true => 1,
    }).sum::<i32>()).sum::<i32>();

    println!("Part 1 solution: {num_tiles_visited}");
}

fn main() {
    let board_layout = BoardLayout::from_file_path("resources/input.txt");
    solve_part_1(&board_layout);
}
