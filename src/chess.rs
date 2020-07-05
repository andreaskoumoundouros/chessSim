//TODO: Find a way to implement the GameBoard functionality so that we can have a gameboard with both empty spaces and references to game pieces at any given "square".

static ASCII_LOWER: [char; 26] = [
    'a', 'b', 'c', 'd', 'e', 
    'f', 'g', 'h', 'i', 'j', 
    'k', 'l', 'm', 'n', 'o',
    'p', 'q', 'r', 's', 't', 
    'u', 'v', 'w', 'x', 'y', 
    'z',
];

static SYMBOL_MAP: [char; 12] = [
    '♔','♕','♘','♗','♖','♙',
    '♚','♛','♞','♝','♜','♟'
];

#[derive(Debug, Clone, Copy)]
pub enum PieceType {
    King,
    Queen,
    Knight,
    Bishop,
    Rook,
    Pawn,
}

impl std::fmt::Display for PieceType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Colors {
    Black,
    White
}

#[derive(Debug)]
pub struct ChessPiece {
    position:  (i32, i32),
    piece_type: PieceType,
    color: Colors,
    in_play: bool,
    has_moved: bool
}

impl ChessPiece {
    pub fn new(position: (i32, i32), piece_type: PieceType, color: Colors) -> ChessPiece{
        ChessPiece {
            position,
            piece_type,
            color,
            in_play: true,
            has_moved: false
        }
    }

    pub fn _get_null_piece() -> ChessPiece {
        return ChessPiece::new((-1, -1), PieceType::Pawn, Colors::Black)
    }

    pub fn update_position(&mut self, position: (i32, i32)) {
        self.position = position;
        if self.has_moved != true {self.has_moved = true;}
    }

    // TODO: When generating moves ensure that the path is clear
    // i.e. there isnt a same colour piece in the way etc..
    pub fn get_possible_moves(&self) -> Vec<(i32,i32)> {
        let mut moves = Vec::new();

        match self.piece_type {
            PieceType::King => {

                let mov1 = (self.position.0, self.position.1+1);
                moves.push(mov1);

                let mov2 = (self.position.0+1, self.position.1+1);
                moves.push(mov2);

                let mov3 = (self.position.0+1, self.position.1);
                moves.push(mov3);

                let mov4 = (self.position.0+1, self.position.1-1);
                moves.push(mov4);

                let mov5 = (self.position.0, self.position.1-1);
                moves.push(mov5);

                let mov6 = (self.position.0-1, self.position.1-1);
                moves.push(mov6);

                let mov7 = (self.position.0-1, self.position.1);
                moves.push(mov7);

                let mov8 = (self.position.0-1, self.position.1+1);
                moves.push(mov8);

            },
            PieceType::Queen => {

                // Up
                for i in 1..8 {
                    if self.position.1 + i < 8 {
                        let mov = (self.position.0, self.position.1 + i);
                        moves.push(mov);
                    } else {
                        break;
                    }
                }

                // Down
                for i in 1..8 {
                    if self.position.1 - i >= 0 {
                        let mov = (self.position.0, self.position.1 - i);
                        moves.push(mov);
                    } else {
                        break;
                    }
                }

                // Left
                for i in 1..8 {
                    if self.position.0 - i >= 0 {
                        let mov = (self.position.0 - i, self.position.1);
                        moves.push(mov);
                    } else {
                        break;
                    }
                }

                // Right
                for i in 1..8 {
                    if self.position.0 + i < 8 {
                        let mov = (self.position.0 + i, self.position.1);
                        moves.push(mov);
                    } else {
                        break;
                    }
                }

                // Top Right
                for i in 1..8 {
                    if self.position.1 + i < 8 {
                        let mov = (self.position.0 + i, self.position.1 + i);
                        moves.push(mov);
                    } else {
                        break;
                    }
                }

                // Bottom Right
                for i in 1..8 {
                    if self.position.1 - i >= 0 {
                        let mov = (self.position.0 + i, self.position.1 - i);
                        moves.push(mov);
                    } else {
                        break;
                    }
                }

                // Bottom Left
                for i in 1..8 {
                    if self.position.0 - i >= 0 {
                        let mov = (self.position.0 - i, self.position.1 - i);
                        moves.push(mov);
                    } else {
                        break;
                    }
                }

                // Top Left
                for i in 1..8 {
                    if self.position.0 + i < 8 {
                        let mov = (self.position.0 - i, self.position.1 + i);
                        moves.push(mov);
                    } else {
                        break;
                    }
                }

                println!("{},{}", self.position.0, self.position.1);
            },
            PieceType::Knight => {
                
                let mov1 = (self.position.0+1, self.position.1+2);
                moves.push(mov1);

                let mov2 = (self.position.0-1, self.position.1+2);
                moves.push(mov2);

                let mov3 = (self.position.0+2, self.position.1+1);
                moves.push(mov3);

                let mov4 = (self.position.0+2, self.position.1-1);
                moves.push(mov4);

                let mov5 = (self.position.0+1, self.position.1-2);
                moves.push(mov5);

                let mov6 = (self.position.0-1, self.position.1-2);
                moves.push(mov6);

                let mov7 = (self.position.0-2, self.position.1+1);
                moves.push(mov7);

                let mov8 = (self.position.0-2, self.position.1-1);
                moves.push(mov8);
            },
            PieceType::Bishop => {
                
                // Top Right
                for i in 1..8 {
                    if self.position.1 + i < 8 {
                        let mov = (self.position.0 + i, self.position.1 + i);
                        moves.push(mov);
                    } else {
                        break;
                    }
                }

                // Bottom Right
                for i in 1..8 {
                    if self.position.1 - i >= 0 {
                        let mov = (self.position.0 + i, self.position.1 - i);
                        moves.push(mov);
                    } else {
                        break;
                    }
                }

                // Bottom Left
                for i in 1..8 {
                    if self.position.0 - i >= 0 {
                        let mov = (self.position.0 - i, self.position.1 - i);
                        moves.push(mov);
                    } else {
                        break;
                    }
                }

                // Top Left
                for i in 1..8 {
                    if self.position.0 + i < 8 {
                        let mov = (self.position.0 - i, self.position.1 + i);
                        moves.push(mov);
                    } else {
                        break;
                    }
                }

            },
            PieceType::Rook => {

                // Up
                for i in 1..8 {
                    if self.position.1 + i < 8 {
                        let mov = (self.position.0, self.position.1 + i);
                        moves.push(mov);
                    } else {
                        break;
                    }
                }

                // Down
                for i in 1..8 {
                    if self.position.1 - i >= 0 {
                        let mov = (self.position.0, self.position.1 - i);
                        moves.push(mov);
                    } else {
                        break;
                    }
                }

                // Left
                for i in 1..8 {
                    if self.position.0 - i >= 0 {
                        let mov = (self.position.0 - i, self.position.1);
                        moves.push(mov);
                    } else {
                        break;
                    }
                }

                // Right
                for i in 1..8 {
                    if self.position.0 + i < 8 {
                        let mov = (self.position.0 + i, self.position.1);
                        moves.push(mov);
                    } else {
                        break;
                    }
                }
            },
            PieceType::Pawn => {
                let mut modifier = 1;
                if self.color == Colors::Black {modifier = -1;}

                println!("Checking pawn: {:?}", self);

                if !self.has_moved {
                    let mov = (self.position.0, self.position.1 + 2*modifier);
                    moves.push(mov);
                }

                let mov = (self.position.0, self.position.1 + 1*modifier);
                moves.push(mov);
            },
        }


        let mut valid_moves = Vec::new();
        for elem in moves {
            if coord_on_board(&elem) {
                valid_moves.push(elem)
            }
        }

        return valid_moves;
    }
}

impl std::fmt::Display for ChessPiece {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?} {:?}", self.color, self.piece_type)
    }
}

pub struct ChessGame {
    pub pieces: Vec<ChessPiece>,
    pub turns: i32,
}

/*
Create a function that creates a new chess game
1. create the board data structure
2. create the pieces, also place the pieces on the board as needed
3. set turns to 1 * White always goes first so White will always be odd number turns and Black even number
*/

impl ChessGame {
    pub fn new() -> ChessGame {
        let mut new_game = ChessGame {
           pieces: Vec::new(),
           turns: 1
        };

        // The coordinates start in the bottom left and increase with y vertically and x horizontally.
        Self::init_game_piece(&mut new_game, (0,0), PieceType::Rook, Colors::White);
        Self::init_game_piece(&mut new_game, (0,7), PieceType::Rook, Colors::Black);

        Self::init_game_piece(&mut new_game, (1,7), PieceType::Knight, Colors::Black);
        Self::init_game_piece(&mut new_game, (1,0), PieceType::Knight, Colors::White);

        Self::init_game_piece(&mut new_game, (2,0), PieceType::Bishop, Colors::White);
        Self::init_game_piece(&mut new_game, (2,7), PieceType::Bishop, Colors::Black);

        Self::init_game_piece(&mut new_game, (3,0), PieceType::Queen, Colors::White);
        Self::init_game_piece(&mut new_game, (3,7), PieceType::Queen, Colors::Black);

        Self::init_game_piece(&mut new_game, (4,0), PieceType::King, Colors::White);
        Self::init_game_piece(&mut new_game, (4,7), PieceType::King, Colors::Black);

        Self::init_game_piece(&mut new_game, (5,0), PieceType::Bishop, Colors::White);
        Self::init_game_piece(&mut new_game, (5,7), PieceType::Bishop, Colors::Black);

        Self::init_game_piece(&mut new_game, (6,0), PieceType::Knight, Colors::White);
        Self::init_game_piece(&mut new_game, (6,7), PieceType::Knight, Colors::Black);

        Self::init_game_piece(&mut new_game, (7,0), PieceType::Rook, Colors::White);
        Self::init_game_piece(&mut new_game, (7,7), PieceType::Rook, Colors::Black);

        for i in 0..8 {
            Self::init_game_piece(&mut new_game, (i, 1), PieceType::Pawn, Colors::White);
            Self::init_game_piece(&mut new_game, (i, 6), PieceType::Pawn, Colors::Black);
        }

        return new_game;
    }

    fn init_game_piece(game: &mut ChessGame, coord: (i32,i32), piece_type: PieceType, color: Colors) {
        
        let temp = ChessPiece::new(coord, piece_type, color);
        game.pieces.push(temp);
    }

    pub fn print_board(&self) {
        for i in 0..8 {
            print!("{} |", 8-i);
            for j in 0..8 {
                print!("[{}]", self.get_symbol_at_position((j,i)));
            }
            println!();
        }

        print!("  ");
        for i in 0..8 {
            print!("  {}", ASCII_LOWER[i]);
        }
        println!();
    }

    pub fn get_piece_at_position(&self, coords: (i32, i32)) -> Result<&ChessPiece, &'static str> {
        for elem in &self.pieces {
            if elem.position == coords {
                return Ok(elem);
            }
        }
        return Err("No Piece at given position.");
    }

    pub fn get_symbol_at_position(&self, coords: (i32, i32)) -> char {
        for elem in &self.pieces {
            if elem.position == coords {
                return convert_piece_to_symbol(elem);
            }
        }
        return ' '; // ■
    }

    // Check for piece at curren location and move it to the new location if it is not occupied
    pub fn move_piece(&mut self, curr: (char,char), mov: (char, char)) -> Result<bool, &'static str> {
        //Convert coords to internal system.
        let curr_internal = convert_user_coord(curr);
        let mov_internal = convert_user_coord(mov);

        // First check there is a piece at curr
        let ret = self.get_piece_at_position(curr_internal)?;

        // Second check that there is open space at mov
        let space = self.get_piece_at_position(mov_internal);
        match space {
            Err(_) => println!("({},{})", mov.0,mov.1),
            Ok(_) => {
                return Err("There is a piece at the selected move location.");
            }
        };

        // Check that the move is legal
        let moves = ret.get_possible_moves();
        if !moves.contains(&mov_internal) || !coord_on_board(&mov_internal) {
            return Err("Illegal move.");
        }

        let path = self.check_path(&curr_internal, &mov_internal);
        match path {
            Some(_) => return Err("Piece in the way."),
            None    => {}
        }

        println!("Moves: {:?}", moves);

        //Update the piece
        for i in 0..33 {
            if self.pieces[i].position == curr_internal {
                self.pieces[i].update_position(mov_internal);
                self.pieces[i].has_moved = true;
                println!("Moving Piece...");
                return Ok(true);
            }
        }

        return Err("Failed to find piece...");
    }

    // Check that the given path is clear and there is no piece of the same color in the way
    pub fn check_path(&self, start: &(i32,i32), end: &(i32,i32)) -> Option<(i32,i32)> {

        let x_modifier = if start.0 > end.0 {
            -1
        } else {
            1
        };

        let y_modifier = if start.1 > end.1 {
            -1
        } else {
            1
        };


        // If same x coord and different y, left or right direction
        // tfw you have to implement your own half-assed dynamic .rev(), smh
        if start.0 == end.0 && start.1 != end.1 {
            println!("Left or right movement. {:?} {:?}", start, end);

            for i in 1..(start.1-end.1).abs() {
                println!("Checking: {:?}", (end.0, start.1 + i*y_modifier));
                let res = self.get_piece_at_position((end.0, start.1 + i*y_modifier));
                match res {
                    Ok(p) => return Some(p.position),
                    Err(_) => {}
                }
            }
        }

        // If same y coord and different x, up or down direction.
        if start.0 != end.0 && start.1 == end.1 {
            println!("up or down movement. {:?} {:?}",start, end);

            for i in 1..(start.0-end.0).abs() {
                println!("Checking: {:?}", (start.0 + i*x_modifier, end.1));
                let res = self.get_piece_at_position((start.0 + i*x_modifier, end.1));
                match res {
                    Ok(p) => return Some(p.position),
                    Err(_) => {}
                }
            }
        }

        // If neither of previous then the movement is diagonal. That is, both coordinates should be different.
        if start.0 != end.0 && start.1 != end.1 {
            println!("diagonal movement. {:?} {:?}", start, end);
            for i in 1..(start.0-end.0).abs() {
                println!("Checking: {:?}", (start.0 + i*x_modifier, start.1 + i*y_modifier));
                let res = self.get_piece_at_position((start.0 + i*x_modifier, start.1 + i*y_modifier));
                match res {
                    Ok(p) => return Some(p.position),
                    Err(_) => {}
                }
            }
        }

        return None;
    }
}

impl std::fmt::Display for ChessGame {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?}", self.pieces)
    }
}

// Converts the given piece to a String representation.
pub fn _convert_piece_to_string (piece: ChessPiece) -> String {
    let piece_type = format!("{:?}", piece.piece_type);
    let (first, _) = piece_type.split_at(2);
    let piece_name = first.to_string();

    return piece_name;
}

// Returns the character representation of the given chesspiece.
pub fn convert_piece_to_symbol (piece: &ChessPiece) -> char {
    let raw_index = piece.piece_type as usize + 6*piece.color as usize;

    return SYMBOL_MAP[raw_index];
}

// Converts the coords as seen by the user into the coordinate system used by the program.
pub fn convert_user_coord (coord: (char,char)) -> (i32,i32){
    let mut x = -1;
    for (index,elem) in ASCII_LOWER.iter().enumerate() {
        if coord.0 == *elem {
            x = index as i32;
        }
    }
    let mut y: i32 = -1;
    match coord.1.to_digit(10) {
        Some(inner) => y = inner as i32,
        None => println!("Somehow got a coord that could not convert in \'convert_user_coord\'")
    };

    return (x, 8 - y);
}

// Check that the given coordinate is in the range of possible board coordinates.
pub fn coord_on_board (coord: &(i32, i32)) -> bool{
    if coord.0 < 0 || coord.1 < 0 || coord.0 > 8 || coord.1 > 8 {
        return false;
    }
    return true;
}