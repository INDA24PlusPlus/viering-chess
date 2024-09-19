# viering-chess

Simple chess library written in rust.

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
let mut game = Game::new(); // creates a chess game with the starting board

let result: MoveResult = game.make_move(Position::new(1, 1), Position::new(1, 3)); // moves the piece at 1,1 (B2) -> 1,3 (B4)

let moves: Vec<Position> = game.get_possible_moves(Position::new(1, 3)); // gets all possible moves for a specific position

game.load_fen("rnbqkbnr/pppppp1p/8/8/6p1/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1") // loads a game from a fen string

game.clear_board(); // clears all pieces from the board
```

## Docs
todo :( :( :(
