use std::cmp::max;

use crate::{Color, Game, PieceType, Position, PositionBuilder};

pub(crate) fn calc_max_move_len(
    game: &Game,
    moving_team: Color,
    base: PositionBuilder,
    direction: (i32, i32),
    can_capture: bool,
) -> i32 {
    let mut move_len = 0;
    let mut builder = base;
    loop {
        builder = builder.walk(direction);
        if let Some(pos) = builder.position {
            let piece = game.get_square(pos);
            if let Some(piece) = piece {
                return if piece.color == moving_team {
                    move_len
                } else if can_capture {
                    move_len + 1
                } else {
                    move_len
                };
            }
            move_len += 1;
            continue;
        }
        return move_len;
    }
}

pub(crate) fn pseudo_validate_knight_move(game: &Game, from: Position, to: Position) -> bool {
    let piece = game.get_square(from).unwrap();

    let base_builder = PositionBuilder::set(from).color(piece.color);
    let valid_positions = [
        base_builder.walk((-1, 2)).build(),
        base_builder.walk((1, 2)).build(),
        base_builder.walk((2, 1)).build(),
        base_builder.walk((2, -1)).build(),
        base_builder.walk((1, -2)).build(),
        base_builder.walk((-1, -2)).build(),
        base_builder.walk((-2, -1)).build(),
        base_builder.walk((-2, 1)).build(),
    ];

    return valid_positions.iter().flatten().any(|pos| *pos == to);
}

pub(crate) fn pseudo_validate_pawn_move(game: &Game, from: Position, to: Position) -> bool {
    let piece = game.get_square(from).unwrap();
    let target_square = game.get_square(to);

    // Forward movement
    if let Some(one_forward) = PositionBuilder::set(from)
        .color(piece.color)
        .forward(1)
        .build()
    {
        if to == one_forward && target_square.is_none() {
            return true;
        }

        // Double move when standing on initial position
        if let Some(two_forward) = PositionBuilder::set(from)
            .color(piece.color)
            .forward(2)
            .build()
        {
            let initial_row = if piece.color == Color::White { 1 } else { 6 };
            if to == two_forward
                && game.get_square(one_forward).is_none()
                && target_square.is_none()
                && from.y == initial_row
            {
                return true;
            }
        }
    }

    // TODO below can be compacted

    // Capture left
    if let Some(diagonal_left) = PositionBuilder::set(from)
        .color(piece.color)
        .forward(1)
        .walk((-1, 0))
        .build()
    {
        if let Some(target_square) = target_square {
            if to == diagonal_left && target_square.color != piece.color {
                return true;
            }
        } else if to == diagonal_left {
            // en passant left
            if let Some(target_square) = PositionBuilder::set(from).walk((-1, 0)).build() {
                if let Some(en_passant_susceptible_pawn) = game.en_passant_susceptible_pawn {
                    if let Some(target_piece) = game.get_square(target_square) {
                        if target_piece.color != piece.color
                            && target_square == en_passant_susceptible_pawn
                            && target_piece.piece_type == PieceType::Pawn
                        {
                            return true;
                        }
                    }
                }
            }
        }
    }

    // Capture right
    if let Some(diagonal_right) = PositionBuilder::set(from)
        .color(piece.color)
        .forward(1)
        .walk((1, 0))
        .build()
    {
        if let Some(target_square) = target_square {
            if to == diagonal_right && target_square.color != piece.color {
                return true;
            }
        } else if to == diagonal_right {
            // en passant right
            if let Some(target_square) = PositionBuilder::set(from).walk((1, 0)).build() {
                if let Some(en_passant_susceptible_pawn) = game.en_passant_susceptible_pawn {
                    if let Some(target_piece) = game.get_square(target_square) {
                        if target_piece.color != piece.color
                            && target_square == en_passant_susceptible_pawn
                            && target_piece.piece_type == PieceType::Pawn
                        {
                            return true;
                        }
                    }
                }
            }
        }
    }

    false
}

pub(crate) fn pseudo_validate_rook_move(game: &Game, from: Position, to: Position) -> bool {
    let piece = game.get_square(from).unwrap();
    let base_builder = PositionBuilder::set(from).color(piece.color);

    let x_diff = to.x as i32 - from.x as i32;
    let y_diff = to.y as i32 - from.y as i32;

    if x_diff != 0 && y_diff != 0 {
        return false;
    }

    let diff = max(x_diff.abs(), y_diff.abs());

    let max_move_len = if x_diff != 0 {
        calc_max_move_len(
            game,
            piece.color,
            base_builder,
            (if x_diff > 0 { 1 } else { -1 }, 0),
            true,
        )
    } else {
        calc_max_move_len(
            game,
            piece.color,
            base_builder,
            (0, if y_diff > 0 { 1 } else { -1 }),
            true,
        )
    };

    diff <= max_move_len
}

pub(crate) fn pseudo_validate_bishop_move(game: &Game, from: Position, to: Position) -> bool {
    let piece = game.get_square(from).unwrap();
    let base_builder = PositionBuilder::set(from).color(piece.color);

    let x_diff = to.x as i32 - from.x as i32;
    let y_diff = to.y as i32 - from.y as i32;

    if x_diff.abs() != y_diff.abs() {
        return false;
    }

    let x_mov = if x_diff > 0 { 1 } else { -1 };
    let y_mov = if y_diff > 0 { 1 } else { -1 };
    let max_move_len = calc_max_move_len(game, piece.color, base_builder, (x_mov, y_mov), true);

    x_diff.abs() <= max_move_len
}

pub(crate) fn pseudo_validate_king_move(game: &Game, from: Position, to: Position) -> bool {
    let piece = game.get_square(from).unwrap();
    let base_builder = PositionBuilder::set(from).color(piece.color);
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

    return valid_positions.iter().flatten().any(|pos| *pos == to);
}

pub(crate) fn pseudo_validate_queen_move(game: &Game, from: Position, to: Position) -> bool {
    pseudo_validate_bishop_move(game, from, to) || pseudo_validate_rook_move(game, from, to)
}
