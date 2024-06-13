use std::{fmt::Display, str::FromStr};

#[derive(Debug, PartialEq)]
pub enum Play {
    Circle,
    Cross,
    Null,
}

enum Direction {
    North,
    East,
    South,
    West,
}

impl Play {
    pub fn value(&self) -> String {
        match self {
            Play::Circle => return "o".to_string(),
            Play::Cross => return "x".to_string(),
            Play::Null => return "_".to_string(),
        }
    }
}

#[derive(Debug)]
pub struct Coordinate {
    x: usize,
    y: usize,
    play: Play,
}

impl FromStr for Coordinate {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts = s.chars().collect::<Vec<_>>();
        println!("{:?}", parts);
        if parts.len() != 3 {
            return Err(anyhow::anyhow!("bad play!"));
        }
        return Ok(Coordinate {
            x: parts[0].to_string().parse()?,
            y: parts[1].to_string().parse()?,
            play: match parts[2].to_string().to_lowercase().as_str() {
                "x" => Play::Cross,
                "o" => Play::Circle,
                _ => Play::Null,
            },
        });
    }
}

pub struct Game {
    board: [[Play; 3]; 3],
}

impl Game {
    pub fn new() -> Game {
        let board = [
            [Play::Null, Play::Null, Play::Null],
            [Play::Null, Play::Null, Play::Null],
            [Play::Null, Play::Null, Play::Null],
        ];
        Game { board }
    }

    pub fn print_board(&self) {
        self.board.iter().for_each(|f| {
            f.iter().for_each(|j| print!(" {} ", j.value()));
            println!();
        });
    }
    pub fn insert_coordinate(&mut self, c: Coordinate) {
        self.board[c.y][c.x] = c.play;
    }

    pub fn is_win(&self, c: Coordinate, count: usize) -> usize {
        todo!()
    }

    fn check_win_direction(
        board: [[Play; 3]; 3],
        mut x: usize,
        mut y: usize,
        play: Play,
        mut win: usize,
        direction: Direction,
    ) -> bool {
        let mut is_win = false;
        if board[y][x] == play {
            win = win + 1;
            let is_win = if win == 3 {
                true
            } else { false };
        } else {
            match direction {
                Direction::North => y = y - 1,
                Direction::East => x = x + 1,
                Direction::South => y = y + 1,
                Direction::West => x = x - 1,
            }
            if x < 0 || x > 2 || y < 0 || y > 2 {
                return false;
            } else {
                return Game::check_win_direction(board, x, y, play, win, direction);
            }
        }
        is_win
    }
}

impl Display for Game {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.board)
    }
}
