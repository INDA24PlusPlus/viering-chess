# viering-chess

Simple chess library written in rust.

## Features
- Basic rules: move validation for all piece types, checking, checkmate, stalemate, etc
- Getting all possible moves for a piece
- Promoting pawns
- Importing boards from fen strings
- En passant
- Fifty-move rule
- Threefold repetition (NOT IMPLEMENTED YET)
- Castling (NOT IMPLEMENTED YET)
- Maybe more might be forgetting stuff, check docs instead :)

## Installation
Put the following inside of your `Cargo.toml`:
```rs
[dependencies]
viering-chess = { git = "https://github.com/INDA24PlusPlus/viering-chess.git" }
```

To import the crate into your project, put the following inside of your rust file:
```rs
use viering_chess::*;
```

## Example usage

```rs
// creates a chess game with the starting board
let mut game = Game::new(); 

// moves the piece at 1,1 (B2) -> 1,3 (B4) (tries to)
let result: MoveResult = game.make_move(Position::new(1, 1), Position::new(1, 3)); 

// gets all possible moves for a specific position
let moves: Vec<Position> = game.get_possible_moves(Position::new(1, 3));

// loads a game from a fen string
game.load_fen("rnbqkbnr/pppppp1p/8/8/6p1/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1"); 

// clears all pieces from the board
game.clear_board(); 

// promotes a pawn to a queen (tries to)
let result: MoveResult = game.promote(PieceType::Queen); 

// get square at 3,4 (D5)
let square: Option<Piece> = game.get_square(Position::new(3, 4));

// set square at 3,4 (D5) to a black rook
game.set_square(
    Position::new(3, 4),
    Some(Piece { piece_type: PieceType::Rook, color: Color::Black }
));

// set square at 3,4 (D5) to be empty
game.set_square(Position::new(3, 4), None));

// printing some useful info:
// game state (normal, check(color), checkmate(color), draw, awaiting promotion)
// current turn (color)
// moves since last capture (u32)
println!(
    "{:?}, {:?}, {}",
    game.game_state,
    game.turn,
    game.moves_since_capture
);
```

## Docs
todo :( :( :(
