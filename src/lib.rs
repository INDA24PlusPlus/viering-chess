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

pub struct ChessTile {
    pub piece: PieceType,
    pub color: Color,
    pub has_piece: bool
}

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

pub fn make_move(game: &mut ChessGame, team: Color, from: usize, to: usize) -> MoveResult {
    if game.turn != team {
        return MoveResult::Disallowed
    }

    // Make sure tiles are in bounds
    if from >= game.board.len() || to >= game.board.len() {
        return MoveResult::Disallowed
    }

    let tile = unpack_tile(game.board[from]);
    // Make sure tile isn't empty
    if !tile.has_piece {
        return MoveResult::Disallowed 
    }
    // Make sure the piece is the correct color
    if tile.color != team {
        return MoveResult::Disallowed
    }
    // Prevent friendly fire
    let target_tile = unpack_tile(game.board[to]);
    if target_tile.color == team && target_tile.has_piece {
        return MoveResult::Disallowed
    }

    // TODO further validation of piece movement patterns, depending on the piece
    if !validate_move(game, from, to){
        return MoveResult::Disallowed
    }

    // Make the move
    game.board[to] = game.board[from];
    game.board[from] = 0;

    game.turn = if team == Color::White { Color::Black } else { Color::White };
    MoveResult::Allowed
}

// TODO make sure this works for black as well, just tested for white atm
fn validate_move(game: &ChessGame, from: usize, to: usize) -> bool {
    let source_tile = unpack_tile(game.board[from]);
    let target_tile = unpack_tile(game.board[to]);

    if source_tile.piece == PieceType::Pawn {
        let modifier: i32 = if source_tile.color == Color::White { 1 } else { -1 };
        

        let pos_diff = (from as i32 - to as i32) * modifier;
        let source_row = from / 8;
        let source_col = from % 8;

        // Regular 1 tile movement
        if pos_diff == 8 && !target_tile.has_piece {
            return true
        }

        // 2 tile movement
        if pos_diff == 16 && !target_tile.has_piece && ((source_tile.color == Color::White && source_row == 6) || source_tile.color == Color::Black && source_row == 1) {
            return true
        }

        // Attacking other pieces
        if ((pos_diff == 7 && source_col != 7) || (pos_diff == 9 && source_col != 0)) && target_tile.has_piece {
            return true
        }

        return false
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
