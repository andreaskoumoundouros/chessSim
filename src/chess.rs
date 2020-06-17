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
    in_play: bool
}

impl ChessPiece {
    pub fn new(position: (i32, i32), piece_type: PieceType, color: Colors) -> ChessPiece{
        ChessPiece {
            position,
            piece_type,
            color,
            in_play: true,
        }
    }
}

impl std::fmt::Display for ChessPiece {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?} [{},{}] {:?} {}", self.piece_type, self.position.0, self.position.1, self.color, self.in_play  )
    }
}

#[derive(Clone)]
pub struct Location {
    current_piece: *const ChessPiece,
}

impl Location{
    pub fn new (current_piece: *const ChessPiece) -> Location {
        Location {
            current_piece
        }
    }
}

impl std::fmt::Display for Location {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "")
    }
}

pub struct ChessBoard {
    locations: Vec<Location>,
}

impl ChessBoard {
    pub fn new () -> ChessBoard {
        ChessBoard {
            locations: Vec::new(),
        }
    } 
}

impl std::fmt::Display for ChessBoard {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "")
    }
}

pub struct ChessGame {
    pub board: ChessBoard,
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
    pub fn new () -> ChessGame {
        let mut new_game = ChessGame {
           board: ChessBoard::new(),
           pieces: Vec::new(),
           turns: 1
        };

        // The coordinates start in the bottom left and increase with y vertically and x horizontally.

        let mut temp = Self::init_game_piece(&mut new_game, (0,0), PieceType::Rook, Colors::White);
        new_game.pieces.push(temp);
        temp = Self::init_game_piece(&mut new_game, (0,7), PieceType::Rook, Colors::Black);
        new_game.pieces.push(temp);

        temp = Self::init_game_piece(&mut new_game, (1,7), PieceType::Knight, Colors::Black);
        new_game.pieces.push(temp);
        temp = Self::init_game_piece(&mut new_game, (1,0), PieceType::Knight, Colors::White);
        new_game.pieces.push(temp);

        temp = Self::init_game_piece(&mut new_game, (2,0), PieceType::Bishop, Colors::White);
        new_game.pieces.push(temp);
        temp = Self::init_game_piece(&mut new_game, (2,7), PieceType::Bishop, Colors::Black);
        new_game.pieces.push(temp);

        temp = Self::init_game_piece(&mut new_game, (3,0), PieceType::Queen, Colors::White);
        new_game.pieces.push(temp);
        temp = Self::init_game_piece(&mut new_game, (3,7), PieceType::Queen, Colors::Black);
        new_game.pieces.push(temp);

        temp = Self::init_game_piece(&mut new_game, (4,0), PieceType::King, Colors::White);
        new_game.pieces.push(temp);
        temp = Self::init_game_piece(&mut new_game, (4,7), PieceType::King, Colors::Black);
        new_game.pieces.push(temp);

        temp = Self::init_game_piece(&mut new_game, (5,0), PieceType::Bishop, Colors::White);
        new_game.pieces.push(temp);
        temp = Self::init_game_piece(&mut new_game, (5,7), PieceType::Bishop, Colors::Black);
        new_game.pieces.push(temp);

        temp = Self::init_game_piece(&mut new_game, (6,0), PieceType::Knight, Colors::White);
        new_game.pieces.push(temp);
        temp = Self::init_game_piece(&mut new_game, (6,7), PieceType::Knight, Colors::Black);
        new_game.pieces.push(temp);

        temp = Self::init_game_piece(&mut new_game, (7,0), PieceType::Rook, Colors::White);
        new_game.pieces.push(temp);
        temp = Self::init_game_piece(&mut new_game, (7,7), PieceType::Rook, Colors::Black);
        new_game.pieces.push(temp);

        for i in 0..8 {
            temp = Self::init_game_piece(&mut new_game, (i, 1), PieceType::Pawn, Colors::White);
            new_game.pieces.push(temp);
            temp = Self::init_game_piece(&mut new_game, (i, 6), PieceType::Pawn, Colors::Black);
            new_game.pieces.push(temp);
        }

        return new_game;
    }

    fn init_game_piece(_game: &mut ChessGame, coord: (i32,i32), piece_type: PieceType, color: Colors) -> ChessPiece {
        
        let temp = ChessPiece::new(coord, piece_type, color);
        return temp;
    }

    pub fn print_board(&self) {
        for i in 0..8 {
            print!("{} ", 8-i);
            for j in 0..8 {
                // if Self::get_piece_at_position(self, (j,i)) {
                //     print!("X ");
                // } else {
                //     print!("O ");
                // }
                print!("{} ", Self::get_symbol_at_position(self, (j,i)));
            }
            println!();
        }

        print!("  ");
        for i in 0..8 {
            print!("{} ", ASCII_LOWER[i]);
        }
    }

    pub fn get_piece_at_position(&self, coords: (i32, i32)) -> bool {
        for elem in &self.pieces {
            if elem.position == coords {
                return true;
            }
        }
        return false;
    }

    pub fn get_symbol_at_position(&self, coords: (i32, i32)) -> char {
        for elem in &self.pieces {
            if elem.position == coords {
                return convert_piece_to_symbol(elem);
            }
        }
        return 'O';
    }
}

impl std::fmt::Display for ChessGame {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?}", self.pieces)
    }
}

pub fn convert_piece_to_string (piece: ChessPiece, ) -> String {
    let piece_type = format!("{:?}", piece.piece_type);
    let (first, _) = piece_type.split_at(2);
    let piece_name = first.to_string();

    return piece_name;
}

pub fn convert_piece_to_symbol (piece: &ChessPiece) -> char {
    let raw_index = piece.piece_type as usize + 6*piece.color as usize;

    return SYMBOL_MAP[raw_index];
}