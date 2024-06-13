use std::io;

use board::Game;

use crate::board::Coordinate;

mod board;

fn main() -> Result<(), anyhow::Error> {
    let game = Game::new();
    game.print_board();

    let mut play = String::new();
    println!("Insert a line and a column to play.");
    io::stdin()
        .read_line(&mut play)
        .expect("Failed to read play");

    println!("play string --> {}", play);
    let play: Coordinate = play.trim().parse()?;

    println!("{:?}", play);

    Ok(())
}
