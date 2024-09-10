#[derive(Clone, Copy, PartialEq)]
pub enum PieceType {
    Pawn = 0b000,
    Knight = 0b001,
    Bishop = 0b010,
    Rook = 0b011,
    Queen = 0b100,
    King = 0b101
}

impl PieceType {
    fn from_u8(value: u8) -> Option<PieceType> {
        match value {
            0b000 => Some(PieceType::Pawn),
            0b001 => Some(PieceType::Knight),
            0b010 => Some(PieceType::Bishop),
            0b011 => Some(PieceType::Rook),
            0b100 => Some(PieceType::Queen),
            0b101 => Some(PieceType::King),
            _ => None
        }
    }
}

#[derive(Clone, Copy, PartialEq)]
pub enum Color {
    White = 0b0,
    Black = 0b1
}

impl Color {
    fn from_u8(value: u8) -> Option<Color> {
        match value {
            0b0 => Some(Color::White),
            0b1 => Some(Color::Black),
            _ => None
        }
    }
}

pub struct ChessGame {
    pub board: [u8; 64],
    pub turn: Color
}

impl ChessGame {
    fn get_tile(&self, pos: Position) -> u8 {
        self.board[((8 - pos.y) * 8 + pos.x) as usize]
    }


    fn set_tile(&mut self, pos: Position, value: u8) {
        self.board[((8 - pos.y) * 8 + pos.x) as usize] = value;
    }
}

pub struct ChessTile {
    pub piece: PieceType,
    pub color: Color,
    pub has_piece: bool
}

#[derive(Clone, Copy, PartialEq, Debug)]
struct Position {
    x: u8,
    y: u8,
}

#[derive(Clone, Copy)]
struct PositionBuilder {
    position: Option<Position>,
    color: Color
}

impl PositionBuilder {
    fn set(position: Position) -> PositionBuilder {
        PositionBuilder { position: Some(position), color: Color::White }
    }

    fn color(mut self, color: Color) -> Self {
        self.color = color;
        self
    }

    fn forward(mut self, amount: i32) -> Self {
        let modifier: i32 = if self.color == Color::White { 1 } else { -1 };
        if let Some(mut pos) = self.position {
            let y_pos: i32 = (pos.y as i32) + (amount * modifier);
            if (0..=7).contains(&y_pos){
                pos.y = y_pos as u8;
                self.position = Some(pos) 
            }
        }
        self
    }

    fn backward(mut self, amount: i32) -> Self {
        let modifier: i32 = if self.color == Color::White { 1 } else { -1 };
        if let Some(mut pos) = self.position {
            let y_pos: i32 = (pos.y as i32) - (amount * modifier);
            if (0..=7).contains(&y_pos){
                pos.y = y_pos as u8;
                self.position = Some(pos) 
            }
        }
        self
    }

    fn left(mut self, amount: i32) -> Self {
        if let Some(mut pos) = self.position {
            let x_pos: i32 = (pos.x as i32) - amount;
            if (0..=7).contains(&x_pos){
                pos.x = x_pos as u8;
                self.position = Some(pos) 
            }
        }
        self
    }

    fn right(mut self, amount: i32) -> Self {
        if let Some(mut pos) = self.position {
            let x_pos: i32 = (pos.x as i32) + amount;
            if (0..=7).contains(&x_pos){
                pos.x = x_pos as u8;
                self.position = Some(pos) 
            }
        }
        self
    }

    fn build(self) -> Option<Position> {
        self.position
    }
}

#[derive(PartialEq)]
pub enum MoveResult {
    Allowed,
    Disallowed
}

pub fn new_game() -> ChessGame {
    from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1").unwrap()
}

pub fn from_fen(fen: &str) -> Option<ChessGame> {
    let mut game = ChessGame{board: [0; 64], turn: Color::White};

    let segments: Vec<&str> = fen.split(" ").collect();

    if segments.len() != 6 {
        return None
    }

    // segment 1: board
    let board_segments: Vec<&str> = segments[0].split("/").collect();
    if board_segments.len() != 8 {
        return None
    }

    for (seg_index, seg) in board_segments.iter().enumerate() {
        let mut filled_tiles = 0;
        
        for chr in seg.chars() {
            if chr.is_ascii_digit() {
                filled_tiles += chr.to_digit(10).unwrap() as usize;
                continue
            }

            let color = if chr.is_uppercase() { Color::White } else { Color::Black }; 
            let piece: PieceType = match chr.to_ascii_lowercase() {
                'p' => PieceType::Pawn,
                'r' => PieceType::Rook,
                'n' => PieceType::Knight,
                'b' => PieceType::Bishop,
                'q' => PieceType::Queen,
                'k' => PieceType::King,
                _ => return None
            };

            game.board[seg_index * 8 + filled_tiles] = pack_tile(piece, color, true);
            filled_tiles += 1;
        }

        if filled_tiles != 8 {
            return None
        }
    }

    // segment 2: next turn
    game.turn = match segments[1] {
        "w" => Color::White,
        "b" => Color::Black,
        _ => return None
    };

    // TODO future segments
    // segment 3: castling ability
    // segment 4: en passant target square
    // segment 5: halfmove clock
    // segment 6: fullmove counter

    Some(game)
}

fn parse_pos(pos: &str) -> Option<Position> {
    if pos.len() != 2 {
        return None
    }

    let mut chars = pos.chars();
    let col = chars.next().unwrap();
    let row = chars.next().unwrap();
    
    let mut position = Position{x: 0, y: 0};

    if "abcdefgh".contains(col){
        position.x = col as u8 - 'a' as u8;
    }
    if let Some(digit) = row.to_digit(10) {
        if !(1..=8).contains(&digit) {
            return None
        }
        
        position.y = digit as u8;
    } else {
        return None
    }

    Some(position)
}

pub fn make_move(game: &mut ChessGame, team: Color, from: &str, to: &str) -> MoveResult {
    // Convert from and to to usize instead of str refs
    let from = match parse_pos(from){
        Some(val) => val,
        None => return MoveResult::Disallowed
    };
    let to = match parse_pos(to){
        Some(val) => val,
        None => return MoveResult::Disallowed
    };

    // Make sure it's the players turn to move
    if game.turn != team {
        return MoveResult::Disallowed
    }

    let source_tile = unpack_tile(game.get_tile(from));

    // Make sure tile isn't empty
    if !source_tile.has_piece {
        return MoveResult::Disallowed
    }

    // Make sure the piece is the correct color
    if source_tile.color != team {
        return MoveResult::Disallowed
    }

    // Prevent friendly fire
    let target_tile = unpack_tile(game.get_tile(to));
    if target_tile.color == team && target_tile.has_piece {
        return MoveResult::Disallowed
    }

    // TODO further validation of piece movement patterns, depending on the piece
    if !validate_move(game, from, to){
        return MoveResult::Disallowed
    }

    // Make the move
    game.set_tile(to, game.get_tile(from));
    game.set_tile(from, 0);

    game.turn = if team == Color::White { Color::Black } else { Color::White };
    MoveResult::Allowed
}

// TODO make sure this works for black as well, just tested for white atm
fn validate_move(game: &ChessGame, from: Position, to: Position) -> bool {
    let source_tile = unpack_tile(game.get_tile(from));
    let target_tile = unpack_tile(game.get_tile(to));

    if source_tile.piece == PieceType::Pawn { 
        if let Some(one_forward) = PositionBuilder::set(from).color(game.turn).forward(1).build() {
            if to == one_forward && !target_tile.has_piece {
                return true
            }

            if let Some(two_forward) = PositionBuilder::set(from).color(game.turn).forward(2).build() {
                let initial_row = if game.turn == Color::White { 2 } else { 7 };
                println!("{}", from.y);
                if to == two_forward && !unpack_tile(game.get_tile(one_forward)).has_piece && !target_tile.has_piece && from.y == initial_row {
                    return true
                }
            }
        }

        if let Some(diagonal_left) = PositionBuilder::set(from).color(game.turn).forward(1).left(1).build() {
            if to == diagonal_left && target_tile.has_piece && target_tile.color != game.turn {
                return true
            }
        }

        if let Some(diagonal_right) = PositionBuilder::set(from).color(game.turn).forward(1).right(1).build() {
            if to == diagonal_right && target_tile.has_piece && target_tile.color != game.turn {
                return true
            }
        }

        return false
    } else if source_tile.piece == PieceType::Knight {
        let base_builder = PositionBuilder::set(from).color(game.turn);
        let valid_positions = [
            base_builder.forward(2).left(1).build(),
            base_builder.forward(2).right(1).build(),
            base_builder.forward(1).left(2).build(),
            base_builder.forward(1).right(2).build(),
            base_builder.backward(2).left(1).build(),
            base_builder.backward(2).right(1).build(),
            base_builder.backward(1).left(2).build(),
            base_builder.backward(1).right(2).build()
        ];

        if valid_positions.iter().flatten().any(|pos| *pos == to){
            return true
        }
    } else if source_tile.piece == PieceType::King {
        let base_builder = PositionBuilder::set(from).color(game.turn);
        let valid_positions = [
            base_builder.left(1).build(),
            base_builder.forward(1).left(1).build(),
            base_builder.forward(1).build(),
            base_builder.forward(1).right(1).build(),
            base_builder.right(1).build(),
            base_builder.backward(1).right(1).build(),
            base_builder.backward(1).build(),
            base_builder.backward(1).left(1).build(),
        ];

        if valid_positions.iter().flatten().any(|pos| *pos == to){
            return true
        }
    }

    false
}

pub fn pack_tile(piece: PieceType, color: Color, has_piece: bool) -> u8 {
    let piece = piece as u8;  
    let color = color as u8;

    (piece & 0b111) | (color << 3) | ((has_piece as u8) << 4)
}

pub fn unpack_tile(packed: u8) -> ChessTile {
    let piece = packed & 0b111;
    let color = (packed >> 3) & 0b1;
    let has_piece = (packed >> 4) & 0b1 == 1;

    let piece = PieceType::from_u8(piece).unwrap();
    let color = Color::from_u8(color).unwrap();

    ChessTile{piece, color, has_piece}
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {

    }
}
