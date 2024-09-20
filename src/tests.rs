#[cfg(test)]
mod chess_tests {
    use std::collections::HashSet;

    use crate::{Color, Game, GameState, MoveResult, PieceType, Position};

    #[test]
    fn checkmate_tests() {
        let mut game = Game::new();

        // scenario 1
        game.load_fen("8/4K3/8/2p5/8/8/1R6/R3k3 b KQkq - 0 1");
        assert_eq!(game.game_state, GameState::Checkmate(Color::Black));

        // scenario 2
        game.load_fen("7k/5N1p/8/8/8/8/8/2K3R1 b KQkq - 0 1");
        assert_eq!(game.game_state, GameState::Checkmate(Color::Black));

        // scenario 3
        game.load_fen("6k1/8/8/8/8/5pP1/5PqP/6K1 w KQkq - 0 1");
        assert_eq!(game.game_state, GameState::Checkmate(Color::White));
    }

    #[test]
    fn en_passant_tests(){
        let mut game = Game::new();

        // scenario 1
        game.load_fen("rnbqkbnr/pppppppp/8/3P4/8/8/PP2PPPP/RNPQKBNR b KQkq - 0 1");
        game.make_move(Position::new(2, 6), Position::new(2, 4)); 
        let res = game.make_move(Position::new(3, 4), Position::new(2, 5));

        assert_eq!(res, MoveResult::Allowed);
        assert!(game.get_square(Position::new(2, 4)).is_none());

        // scenario 2
        game.load_fen("rnbqkbnr/pppppp1p/8/8/6p1/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1");
        game.make_move(Position::new(5, 1), Position::new(5, 3));
        let res = game.make_move(Position::new(6, 3), Position::new(5, 2));
        
        assert_eq!(res, MoveResult::Allowed);
        assert!(game.get_square(Position::new(5, 3)).is_none());
    }

    #[test]
    fn stalemate_tests() {
        let mut game = Game::new();

        // scenario 1
        game.load_fen("k7/8/1Q6/8/8/8/8/K7 b KQkq - 0 1");
        assert_eq!(game.game_state, GameState::Draw);

        // scenario 2
        game.load_fen("k7/5b2/4r3/3K4/2r5/1b6/8/8 w KQkq - 0 1");
        assert_eq!(game.game_state, GameState::Draw);
        // scenario 3

        game.load_fen("k7/5b2/4r3/3K4/2r5/1b6/8/8 b KQkq - 0 1");
        assert_eq!(game.game_state, GameState::Normal);
    }

    #[test]
    fn check_possible_moves_test() {
        let mut game = Game::new();

        // scenario 1
        game.load_fen("1r6/8/4k3/8/2K5/2P5/8/8 w KQkq - 0 1");
        let correct_possible_moves = vec![
            Position::new(2, 4),
            Position::new(3, 3),
            Position::new(3, 2),
        ];
        let possible_moves = game.get_possible_moves(Position::new(2, 3));
        assert!(no_order_iters_eq(
            possible_moves.into_iter(),
            correct_possible_moves.into_iter()
        ));

        // scenario 2
        game.load_fen("8/8/8/4p1b1/5P2/8/8/2K5 w KQkq - 0 1");
        let correct_possible_moves = vec![Position::new(6, 4)];
        let possible_moves = game.get_possible_moves(Position::new(5, 3));
        assert!(no_order_iters_eq(
            possible_moves.into_iter(),
            correct_possible_moves.into_iter()
        ));
    }

    #[test]
    fn castling_tests(){
        let mut game = Game::new();
        
        // scenario 1
        game.load_fen("rn1qkbnr/pppppppp/8/8/b7/8/PP1PPPPP/R3KBNR w KQkq - 0 1");
        assert!(game.make_move(Position::from_string("e1"), Position::from_string("c1")) == MoveResult::Disallowed);

        // scenario 2
        game.load_fen("rn1qkbnr/pppppppp/8/8/8/8/PPPPPPPP/R3KBNR w KQkq - 0 1");
        assert!(game.make_move(Position::from_string("e1"), Position::from_string("c1")) == MoveResult::Allowed);
        
        let square = game.get_square(Position::from_string("d1"));
        assert!(square.is_some());
        if let Some(square) = square {
            assert!(square.piece_type == PieceType::Rook);
        }

        // scenario 3
        game.load_fen("rn1qkbn1/ppppppp1/6r1/8/8/8/PPPPP2P/RNBQK2R w KQq - 0 1");
        assert!(game.make_move(Position::from_string("e1"), Position::from_string("g1")) == MoveResult::Disallowed);
    }

    // Checks if two vectors contain the exact same elements (order doesn't matter)
    fn no_order_iters_eq(
        mut first: impl Iterator<Item = Position>,
        second: impl Iterator<Item = Position>,
    ) -> bool {
        let set: HashSet<Position> = second.collect();

        first.all(|el| set.contains(&el))
    }
}
