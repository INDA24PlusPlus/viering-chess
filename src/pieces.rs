use std::cmp::max;

use crate::{calc_max_move_len, unpack_tile, ChessGame, Color, Position, PositionBuilder};

pub fn validate_knight_move(game: &ChessGame, from: Position, to: Position) -> bool {
    let base_builder = PositionBuilder::set(from).color(game.turn);
    let valid_positions = [
        base_builder.walk((-1, 2)).build(),
        base_builder.walk((1, 2)).build(),
        base_builder.walk((2, 1)).build(),
        base_builder.walk((2, -1)).build(),
        base_builder.walk((1, -2)).build(),
        base_builder.walk((-1, -2)).build(),
        base_builder.walk((-2, -1)).build(),
        base_builder.walk((-2, 1)).build()
    ];

    return valid_positions.iter().flatten().any(|pos| *pos == to)
}


pub fn validate_pawn_move(game: &ChessGame, from: Position, to: Position) -> bool {
    let target_tile = unpack_tile(game.get_tile(to));

    // Forward movement
    if let Some(one_forward) = PositionBuilder::set(from).color(game.turn).forward(1).build() {
        if to == one_forward && !target_tile.has_piece {
            return true
        }

        // Double move when standing on initial position
        if let Some(two_forward) = PositionBuilder::set(from).color(game.turn).forward(2).build() {
            let initial_row = if game.turn == Color::White { 1 } else { 6 };
            if to == two_forward && !unpack_tile(game.get_tile(one_forward)).has_piece && !target_tile.has_piece && from.y == initial_row {
                return true
            }
        }
    }

    // Capture left
    if let Some(diagonal_left) = PositionBuilder::set(from).color(game.turn).forward(1).walk((-1, 0)).build() {
        if to == diagonal_left && target_tile.has_piece && target_tile.color != game.turn {
            return true
        }
    }

    // Capture right
    if let Some(diagonal_right) = PositionBuilder::set(from).color(game.turn).forward(1).walk((1, 0)).build() {
        if to == diagonal_right && target_tile.has_piece && target_tile.color != game.turn {
            return true
        }
    }

    false
}


pub fn validate_rook_move(game: &ChessGame, from: Position, to: Position) -> bool {
    let base_builder = PositionBuilder::set(from).color(game.turn);

    let x_diff = to.x as i32 - from.x as i32;
    let y_diff = to.y as i32 - from.y as i32;
    
    if x_diff != 0 && y_diff != 0 {
        return false
    }

    let diff = max(x_diff.abs(), y_diff.abs());
    
    let max_move_len = if x_diff > 0 {
        calc_max_move_len(game, base_builder, (1, 0), true)
    }else if x_diff < 0 {
        calc_max_move_len(game, base_builder, (-1, 0), true)
    }else if y_diff > 0 {
        calc_max_move_len(game, base_builder, (0, 1), true)
    }else if y_diff < 0 {
        calc_max_move_len(game, base_builder, (0, -1), true)
    }else {
        0
    };

    diff <= max_move_len
}


pub fn validate_bishop_move(game: &ChessGame, from: Position, to: Position) -> bool {
    let base_builder = PositionBuilder::set(from).color(game.turn);

    let x_diff = to.x as i32 - from.x as i32;
    let y_diff = to.y as i32 - from.y as i32;
    
    if x_diff.abs() != y_diff.abs() {
        return false
    }

    let x_mov = if x_diff > 0 { 1 } else { -1 };
    let y_mov = if y_diff > 0 { 1 } else { -1 };
    let max_move_len = calc_max_move_len(game, base_builder, (x_mov, y_mov), true);

    x_diff.abs() <= max_move_len
}


pub fn validate_king_move(game: &ChessGame, from: Position, to: Position) -> bool {
    let base_builder = PositionBuilder::set(from).color(game.turn);
    let valid_positions = [
        base_builder.walk((-1, 1)).build(),
        base_builder.walk((0, 1)).build(),
        base_builder.walk((1, 1)).build(),
        base_builder.walk((-1, 0)).build(),
        base_builder.walk((1, 0)).build(),
        base_builder.walk((-1, -1)).build(),
        base_builder.walk((0, -1)).build(),
        base_builder.walk((1, -1)).build(),
    ];

    return valid_positions.iter().flatten().any(|pos| *pos == to)
}

pub fn validate_queen_move(game: &ChessGame, from: Position, to: Position) -> bool {
    validate_bishop_move(game, from, to) || validate_rook_move(game, from, to)
}
