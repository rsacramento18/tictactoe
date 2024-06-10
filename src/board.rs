use std::fmt::Display;

#[derive(Debug)]
pub enum Play {
    Circle,
    Cross,
    Null,
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

pub struct Coordinate {
    x: u8,
    y: u8,
    play: Play,
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
        Game{board}
    }

    pub fn print_board(&self) {
        self.board
            .iter()
            .for_each(|f| {
                f.iter().for_each(|j| print!(" {} ", j.value()));
                println!();
            });
    }
}

impl Display for Game {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.board)
    }
}
