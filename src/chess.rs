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

#[derive(Debug, Clone, Copy)]
pub enum Colors {
    Black,
    White
}

#[derive(Debug)]
pub struct ChessPiece {
    position: (i32, i32),
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
                print!("[{}]", Self::get_symbol_at_position(self, (j,i)));
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
        return ' ';
    }

    // Check for piece at curren location and move it to the new location if it is not occupied.
    pub fn move_piece(&self, curr: (char,char), mov: (char, char)) {
        //Convert coords to internal system.
        let curr_internal = convert_user_coord(curr);
        let mov_internal = convert_user_coord(mov);

        // First check there is a piece at curr.
        let ret = Self::get_piece_at_position(self, curr_internal);
        let _piece: ChessPiece = ChessPiece::new((-1,-1), PieceType::Pawn, Colors::Black);
        match ret {
            Ok(p) => {
                print!("{} ({},{}) To ", p, curr.0, curr.1);
            },
            Err(_) => {
                println!("There is no piece at ({},{})", curr.0,curr.1);
                return;
            }
        };

        // Second check that there is open space at mov
        let space = Self::get_piece_at_position(self, mov_internal);
        match space {
            Err(_) => println!("({},{})", mov.0,mov.1),
            Ok(p) => {
                println!("There is a piece at ({},{}) {}", mov.0, mov.1, p);
                return;
            }
        };

        //Update the piece
        //piece.position = mov_internal;
    }
}

impl std::fmt::Display for ChessGame {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?}", self.pieces)
    }
}

pub fn _convert_piece_to_string (piece: ChessPiece, ) -> String {
    let piece_type = format!("{:?}", piece.piece_type);
    let (first, _) = piece_type.split_at(2);
    let piece_name = first.to_string();

    return piece_name;
}

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

    return (x, y - 1);
}