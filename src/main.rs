mod chess;
use chess as ch;
use std::io;

fn main() {
    let mut game = ch::ChessGame::new();

    loop {
        game.print_board();
        println!("Enter your move in the format: \"x y x y\", where the first set are the current coords, and the second set are the desired coords.");
        let mut input = String::new();
        io::stdin().read_line(&mut input)
            .ok()
            .expect("Couldn't read line");
        let mut coords: Vec<char> = input.chars().collect();
        coords.retain(|&x| x != ' ' && x != '\r' && x != '\n');

        if coords[0] == 'q' {
            println!("Ye Boi");
            return;
        }
        if coords.len() < 4 {
            println!("Invalid coordinate format.");
            continue;
        }

        let ret = game.move_piece((coords[0],coords[1]), (coords[2],coords[3]));
        match ret {
            Ok(_) => {}
            Err(e) => {println!("{}", e);}
        }
    }
}
