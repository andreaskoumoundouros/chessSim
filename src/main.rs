mod chess;
use chess as ch;

use tetra::graphics::{self, Color, DrawParams, Texture, Rectangle};
use tetra::input::{self, MouseButton};
use tetra::math::Vec2;
use tetra::{Context, ContextBuilder, State};

const WINDOW_WIDTH: f32 = 1280.0;
const WINDOW_HEIGHT: f32 = 720.0;
const BOARD_WIDTH: f32 = 1166.0;
const BOARD_HEIGHT: f32 = 1145.0;
const BOARD_PARAM: f32 = 1024.0;
// Board texture offsets
// Mouse coords are from the top left so subtract the top and left offsets to get the position on the actual board
const LEFT_OFFSET: f32 = 74.0;
// const RIGHT_OFFSET: f32 = 70.0;
const TOP_OFFSET: f32 = 61.0;
// const BOTTOM_OFFSET: f32 = 62.0;

struct GameState {
    game: ch::ChessGame,
    board: Texture,
    selection: Texture,
    piece_sheet: Texture,
    mouse_position: Vec2<f32>,
    board_position: Vec2<f32>,
    _max_board_position: f32,
    position_multiple: f32,
    scale: Vec2<f32>,
    highlight_positions: Vec<Vec2<f32>>,
    selected_position: Vec2<i32>
}

impl GameState {
    fn new(ctx: &mut Context) -> tetra::Result<GameState> {
        let scaling_param = (WINDOW_WIDTH/BOARD_WIDTH).min(WINDOW_HEIGHT/BOARD_HEIGHT);
        Ok(GameState {
            game: ch::ChessGame::new(),
            board: Texture::new(ctx, "./resources/board.png")?,
            selection: Texture::new(ctx, "./resources/selection.png")?,
            piece_sheet: Texture::new(ctx, "./resources/ChessPiecesArray.png")?,
            mouse_position: Vec2::new(0.0, 0.0),
            board_position: Vec2::new(0.0, 0.0),
            _max_board_position: (BOARD_PARAM*scaling_param).round(),
            position_multiple:(BOARD_PARAM*scaling_param).round()/8.0,
            scale: Vec2::new(scaling_param, scaling_param),
            highlight_positions: Vec::new(),
            selected_position: Vec2::new(-1,-1),
        })
    }
}

impl State for GameState {
    fn update(&mut self, ctx: &mut Context) -> tetra::Result {
        self.mouse_position = input::get_mouse_position(ctx).round();
        self.board_position = Vec2::new(
            self.mouse_position[0]-LEFT_OFFSET*self.scale[0],
            self.mouse_position[1]-TOP_OFFSET*self.scale[0]
        );
        
        if input::is_mouse_button_pressed(ctx, MouseButton::Left) {
            let convert_pos = self.board_position.map(|x| (x as i32/self.position_multiple as i32));


            if convert_pos[0] > 7 || convert_pos[1] < 0 || convert_pos[1] > 7 || convert_pos[0] < 0 {
                
            } else {
                let square_center = Vec2::new(
                    (convert_pos[0] as f32 +0.5)*self.position_multiple as f32 + LEFT_OFFSET*self.scale[0],
                    (convert_pos[1] as f32 +0.5)*self.position_multiple as f32 + TOP_OFFSET*self.scale[0]
                );
                self.highlight_positions.push(square_center);

                // TODO: instead of just attempting to move to this spot regardless allow for selection of different
                // pieces without having to remove the current selection.
                if self.selected_position != Vec2::new(-1,-1) {
                    match self.game.move_piece(self.selected_position.into_tuple(), convert_pos.into_tuple()) {
                        Ok(_) => {
                            self.selected_position[0] = -1;
                            self.selected_position[1] = -1;
                            self.highlight_positions.clear();
                        },
                        Err(e) => println!("Failed to move piece. {}", e),
                    }
                } else {
                    self.selected_position = convert_pos;
                    let piece = self.game.get_piece_at_position(&(convert_pos[0], convert_pos[1]));
                    match piece {
                        Ok(p) => {
                            println!("Selected {}", p);
                        },
                        Err(e) => {println!("{}", e)},
                    }
                }  
            }
        }

        if input::is_mouse_button_released(ctx, MouseButton::Right) {
            // let position_multiple = (self.max_board_position/8.0) as i32;
            // let convert_pos = self.board_position.map(|x| (x as i32/position_multiple));

            self.highlight_positions.clear();
            self.selected_position[0] = -1;
            self.selected_position[1] = -1;
        }

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> tetra::Result {
        graphics::clear(ctx, Color::rgb(0.769, 0.812, 0.631));

        graphics::draw(
            ctx,
            &self.board,
            DrawParams::new()
                .position(Vec2::new(0.0, 0.0))
                .origin(Vec2::new(0.0, 0.0))
                .scale(self.scale)
        );

        let sprites: Vec<Rectangle> = Rectangle::row(0.0, 0.0, 128.0, 128.0).take(12).collect();
        for elem in &self.game.pieces {
            let square_center = Vec2::new(
                (elem.get_position().0 as f32 +0.5)*self.position_multiple as f32 + LEFT_OFFSET*self.scale[0],
                (elem.get_position().1 as f32 +0.5)*self.position_multiple as f32 + TOP_OFFSET*self.scale[0]
            );
            
            let mut index = 0;
            match elem.piece_type {
                ch::PieceType::King => {
                    match elem.color {
                        ch::Colors::White => {},
                        ch::Colors::Black => {index = 6},
                    }
                },
                ch::PieceType::Queen => {
                    match elem.color {
                        ch::Colors::White => {index = 1},
                        ch::Colors::Black => {index = 7},
                    }
                },
                ch::PieceType::Knight => {
                    match elem.color {
                        ch::Colors::White => {index = 3},
                        ch::Colors::Black => {index = 9},
                    }
                },
                ch::PieceType::Bishop => {
                    match elem.color {
                        ch::Colors::White => {index = 4},
                        ch::Colors::Black => {index = 10},
                    }
                },
                ch::PieceType::Rook => {
                    match elem.color {
                        ch::Colors::White => {index = 2},
                        ch::Colors::Black => {index = 8},
                    }
                },
                ch::PieceType::Pawn => {
                    match elem.color {
                        ch::Colors::White => {index = 5},
                        ch::Colors::Black => {index = 11},
                    }
                },
            }

            graphics::draw(
                ctx,
                &self.piece_sheet,
                DrawParams::new()
                    .position(square_center)
                    .origin(Vec2::new(64.0, 64.0))
                    .scale(self.scale)
                    .clip(sprites[index]),
            );
        }

        for elem in &self.highlight_positions {
            graphics::draw(
                ctx,
                &self.selection,
                DrawParams::new()
                    .position(*elem)
                    .origin(Vec2::new(64.0, 64.0))
                    .scale(self.scale)
                    .color(Color::rgba(1.0, 1.0, 1.0, 0.4))
            );
        }

        Ok(())
    }
}

fn main() -> tetra::Result {
    ContextBuilder::new("Mouse Input", WINDOW_WIDTH as i32, WINDOW_HEIGHT as i32)
        .quit_on_escape(true)
        .show_mouse(true)
        .build()?
        .run(GameState::new)
}
