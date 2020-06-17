mod chess;
use chess as ch;

fn main() {
    let game = ch::ChessGame::new();
    ch::ChessGame::print_board(&game);
}
