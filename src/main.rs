use board::Game;

mod board;

fn main() {
    let game = Game::new();
    game.print_board();
}
