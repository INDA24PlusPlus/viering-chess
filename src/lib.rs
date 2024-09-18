pub mod moves;
use crate::moves::*;

#[derive(Copy, Clone, PartialEq, Debug)]
pub struct Position {
    pub x: u8,
    pub y: u8,
}

impl Position {
    pub fn new(x: u8, y: u8) -> Self {
        if x > 7 || y > 7 {
            panic!("Attempt to initialize Position with out of bounds coordinates. Valid range is 0-7.");
        }

        Self { x, y }
    }
}

#[derive(Clone, Copy)]
pub struct PositionBuilder {
    position: Option<Position>,
    color: Color,
}

impl PositionBuilder {
    fn set(position: Position) -> PositionBuilder {
        PositionBuilder {
            position: Some(position),
            color: Color::White,
        }
    }

    fn color(mut self, color: Color) -> Self {
        self.color = color;
        self
    }

    fn walk(mut self, amount: (i32, i32)) -> Self {
        if let Some(mut pos) = self.position {
            let new_pos: (i32, i32) = (pos.x as i32 + amount.0, pos.y as i32 + amount.1);
            if (0..=7).contains(&new_pos.0) && (0..=7).contains(&new_pos.1) {
                pos.x = new_pos.0 as u8;
                pos.y = new_pos.1 as u8;
                self.position = Some(pos)
            } else {
                self.position = None
            }
        }
        self
    }

    // Move forward in the direction the piece is facing
    fn forward(mut self, amount: i32) -> Self {
        let modifier: i32 = if self.color == Color::White { 1 } else { -1 };
        if let Some(mut pos) = self.position {
            let y_pos: i32 = (pos.y as i32) + (amount * modifier);
            if (0..=7).contains(&y_pos) {
                pos.y = y_pos as u8;
                self.position = Some(pos)
            } else {
                self.position = None
            }
        }
        self
    }

    fn build(self) -> Option<Position> {
        self.position
    }
}

#[derive(Debug)]
pub enum MoveResult {
    Allowed,
    Disallowed,
}

#[derive(Copy, Clone)]
pub enum GameState {
    Normal,
    Check(Color),
    Checkmate(Color),
    Stalemate,
}

#[derive(Copy, Clone, PartialEq)]
pub enum PieceType {
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King,
}

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum Color {
    Black,
    White,
}

#[derive(Copy, Clone)]
pub struct Piece {
    pub piece_type: PieceType,
    pub color: Color,
}

pub type Square = Option<Piece>;

#[derive(Clone)]
pub struct Game {
    pub squares: [Square; 8 * 8],
    pub turn: Color,
    pub game_state: GameState,
}

impl Game {
    pub fn new() -> Self {
        Self {
            squares: [None; 8 * 8],
            turn: Color::White,
            game_state: GameState::Normal,
        }
    }

    pub fn get_square(&self, position: Position) -> Square {
        self.squares[8 * 8 - 8 - position.y as usize * 8 + position.x as usize]
    }

    pub fn set_square(&mut self, position: Position, value: Square) {
        self.squares[8 * 8 - 8 - position.y as usize * 8 + position.x as usize] = value;
    }

    pub fn load_fen(&mut self, fen: &str) {
        // Clear board
        self.squares.iter_mut().for_each(|square| *square = None);

        let segments: Vec<&str> = fen.split(" ").collect();

        if segments.len() != 6 {
            return; // ERROR
        }

        let board_segments: Vec<&str> = segments[0].split("/").collect();
        if board_segments.len() != 8 {
            return; // ERROR
        }

        // Parse segment 1: Board
        for (seg_index, seg) in board_segments.iter().enumerate() {
            let mut filled_tiles = 0;

            for chr in seg.chars() {
                if chr.is_ascii_digit() {
                    filled_tiles += chr.to_digit(10).unwrap() as usize;
                    continue;
                }

                let color = if chr.is_uppercase() {
                    Color::White
                } else {
                    Color::Black
                };
                let piece: PieceType = match chr.to_ascii_lowercase() {
                    'p' => PieceType::Pawn,
                    'r' => PieceType::Rook,
                    'n' => PieceType::Knight,
                    'b' => PieceType::Bishop,
                    'q' => PieceType::Queen,
                    'k' => PieceType::King,
                    _ => return, // ERROR
                };

                self.squares[seg_index * 8 + filled_tiles] = Some(Piece {
                    piece_type: piece,
                    color,
                });
                filled_tiles += 1;
            }

            if filled_tiles != 8 {
                return; // ERROR
            }
        }

        // Parse segment 2: Turn
        self.turn = match segments[1] {
            "w" => Color::White,
            "b" => Color::Black,
            _ => return, // ERROR
        };

        // TODO: future segments
        // segment 3: castling ability
        // segment 4: en passant target square
        // segment 5: halfmove clock
        // segment 6: fullmove counter

        // TODO this should probably check game state as well, right?
    }

    fn pseudo_validate_move(&self, from: Position, to: Position) -> bool {
        let source_square = self.get_square(from);

        if source_square.is_none() {
            return false;
        }

        let source_square = source_square.unwrap();

        match source_square.piece_type {
            PieceType::Pawn => pseudo_validate_pawn_move(&self, from, to),
            PieceType::Knight => pseudo_validate_knight_move(&self, from, to),
            PieceType::Bishop => pseudo_validate_bishop_move(&self, from, to),
            PieceType::Rook => pseudo_validate_rook_move(&self, from, to),
            PieceType::Queen => pseudo_validate_queen_move(&self, from, to),
            PieceType::King => pseudo_validate_king_move(&self, from, to),
        }
    }

    fn validate_move(&self, from: Position, to: Position) -> bool {
        if !self.pseudo_validate_move(from, to) {
            return false;
        }

        // Clone the board and simulate the move
        let mut new_game = self.clone();
        new_game.set_square(to, new_game.get_square(from));
        new_game.set_square(from, None);

        let source_square = new_game.get_square(to).unwrap();

        let mut white_king_pos = Position::new(0, 0);
        let mut black_king_pos = Position::new(0, 0);
        for x in 0..=7 {
            for y in 0..=7 {
                let pos = Position::new(x, y);
                if let Some(square) = new_game.get_square(pos) {
                    if square.piece_type == PieceType::King {
                        if square.color == Color::White {
                            white_king_pos = pos;
                        } else {
                            black_king_pos = pos;
                        }
                    }
                }
            }
        }

        for x in 0..=7 {
            for y in 0..=7 {
                let pos = Position::new(x, y);

                let possible_moves = new_game.get_pseudo_possible_moves(pos);

                for possible_move in possible_moves {
                    if possible_move == white_king_pos && source_square.color == Color::White {
                        return false;
                    }

                    if possible_move == black_king_pos && source_square.color == Color::Black {
                        return false;
                    }
                }
            }
        }

        true
    }

    pub fn make_move(&mut self, from: Position, to: Position) -> MoveResult {
        let source_square = self.get_square(from);
        let target_square = self.get_square(to);

        // Move is invalid if the piece didn't move
        if from == to {
            return MoveResult::Disallowed;
        }

        // Move is invalid if the source tile is empty
        if source_square.is_none() {
            return MoveResult::Disallowed;
        }

        // Move is invalid if it's not the correct turn
        if let Some(source_square) = source_square {
            if source_square.color != self.turn {
                return MoveResult::Disallowed;
            }
        }

        // Prevent friendly fire
        if let Some(target_square) = target_square {
            if target_square.color == self.turn {
                return MoveResult::Disallowed;
            }
        }

        // TODO validate the move by checking the resulting game state
        if !self.validate_move(from, to) {
            return MoveResult::Disallowed;
        }

        // Make the move
        self.set_square(to, source_square);
        self.set_square(from, None);

        // Change the turn
        self.turn = if self.turn == Color::White {
            Color::Black
        } else {
            Color::White
        };

        MoveResult::Allowed
    }

    pub fn get_possible_moves(&self, from: Position) -> Vec<Position> {
        let pseudo_possible_moves = self.get_pseudo_possible_moves(from);

        let mut possible_moves: Vec<Position> = Vec::new();

        for pseudo_possible_move in pseudo_possible_moves {
            if self.validate_move(from, pseudo_possible_move) {
                possible_moves.push(pseudo_possible_move);
            }
        }

        possible_moves
    }

    fn get_pseudo_possible_moves(&self, from: Position) -> Vec<Position> {
        let mut possible_moves: Vec<Position> = Vec::new();

        let source_square = self.get_square(from);

        if source_square.is_none() {
            return possible_moves;
        }
        let source_square = source_square.unwrap();

        for x in 0..=7 {
            for y in 0..=7 {
                let pos = Position { x, y };

                // Skip all positions which contain pieces of the same team
                if let Some(target_tile) = self.get_square(pos) {
                    if target_tile.color == source_square.color {
                        continue;
                    }
                }

                if self.pseudo_validate_move(from, pos) {
                    possible_moves.push(pos)
                }
            }
        }

        possible_moves
    }
}
