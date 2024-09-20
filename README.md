# viering-chess

Simple chess library written in rust.

## Features
- Basic rules: move validation for all piece types, checking, checkmate, stalemate, etc
- All advanced rules: promoting, castling, en passant, fifty-move rule, threefold repetition (NOT IMPLEMENTED YET), etc
- Getting all possible moves for a piece
- Importing boards from fen strings
- Maybe more might be forgetting stuff, check docs instead :)

## Installation
Put the following inside of your `Cargo.toml`:
```toml
[dependencies]
viering-chess = { git = "https://github.com/INDA24PlusPlus/viering-chess.git" }
```

To import the crate into your project, put the following inside of your rust file:
```rs
use viering_chess::*;
```

## Example usage
Below is some examples of how to use the API. See [this link](https://gist.github.com/freeeranger/e88f6834ebb156c28da3d5aa0f04e9c6) for a fully functional terminal chess client. 
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

// set square at 3,4 (D5) to be empty using algebraic notation
game.set_square(Position::from_string("D5"), None);

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
### Position
`Position` is a struct with the fields `x: u8` and `y: u8`. The position is counted with `0, 0` being the bottom left corner (queenside white) and `7, 7` being the top right corner (kingside black).

Constructing a position can be done through:
-  `Position::new(x: u8, y: u8)` which constructs a position through integer coordinates
- `Position::from_string(string: &str)` which constructs a position through algebraic notation


### GameState
`GameState` is an enum with 5 possible states:
- `Normal`: When nothing special is happening in the game
- `Check(Color)`: When the specified color is in check
- `Checkmate(Color)`: When the specified color has been checkmated
- `Draw`: When the game has ended as a draw
- `AwaitingPromotion(Position)`: When the piece at specified position is awaiting promotion

**Note:** While in `AwaitingPromotion`, no moves can be made until the piece has been promoted.


### Square
A `Square` is an individual square on the board. In code, it is represented by an `Option<Piece>`.

### Piece
A `Piece` is a struct with two fields: `piece_type: PieceType` and `color: Color`.

### PieceType
`PieceType` is an enum consisting of all possible types of pieces: `Pawn`, `Rook`, `Knight`, `Bishop`, `Queen` and `King`.

### Color
`Color` is an enum for the two colors in chess: `White` and `Black`.

### MoveResult
`MoveResult` is an enum returned when making a move, promoting, etc. It can either be `Allowed` or `Disallowed`.

### Game
A `Game` is the struct that holds all of the useful methods, state etc for the chess game. Its methods are probably best explained by the example usage section above, but in case you need more in-depth information, here's a full run-down:

The `Game` struct has the following fields:
- `squares: [Square; 8 * 8]`: The internal representation of the board.
- `turn: Color`: The color who's turn it is.
- `game_state: GameState`: Holds the state of the game.
- `moves_since_capture: u32`: The number of moves since the last capture was made.
- `en_passant_susceptible_pawn: Option<Position>`: Holds the position of the pawn susceptible to en passant (if there is one).
- `white_castling_kingside_available: bool`: If castling is possible on white's kingside.
- `white_castling_queenside_available: bool`: If castling is possible on white's queenside.
- `black_castling_kingside_available: bool`: If castling is possible on black's kingside.
- `black_castling_queenside_available: bool`: If castling is possible on black's queenside.

The `Game` struct has the following methods:
- `new() -> Self`: A static method returning an instance of the board with the default board setup. 
- `clear_board()`: Clears the board
- `load_fen(fen: &str)`: Loads a game from the fen string
- `get_square(position: Position) -> Square`: Returns the square at the given position
- `set_square(position: Position, value: Square)`: Sets the square at the given position to the given value
-  `make_move(from: Position, to: Position) -> MoveResult`: Tries to move a piece from one position to the other (taking chess rules into account)
- `promote(new_type: PieceType) -> MoveResult`: Promotes a piece to the given piece type if there is one to promote
- `get_possible_moves(from: Position) -> Vec<Position>`: Returns all possible moves for the piece at the given position