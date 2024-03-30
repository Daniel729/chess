#[derive(Clone, Copy, Debug)]
enum Piece {
    WhitePawn,
    WhiteRook,
    WhiteKnight,
    WhiteBishop,
    WhiteQueen,
    WhiteKing,
    BlackPawn,
    BlackRook,
    BlackKnight,
    BlackBishop,
    BlackQueen,
    BlackKing,
}

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
enum Players {
    White,
    Black,
}

mod mod_position {
    #[derive(Clone, Copy)]
    pub struct Position(i8, i8); // rand, coloana : row, col
    impl Position {
        pub fn new(row: i8, col: i8) -> Self {
            Position(row, col)
        }

        pub fn row(&self) -> i8 {
            self.0
        }

        pub fn col(&self) -> i8 {
            self.1
        }
    }

    impl std::ops::Add for Position {
        type Output = Self;

        fn add(self, other: Self) -> Self {
            Self(self.0 + other.0, self.1 + other.1)
        }
    }
}

use mod_position::Position;

#[derive(Clone, Copy)]
pub enum Move {
    Normal {
        piece: Piece,
        start: Position,
        end: Position,
        captured_piece: Option<Piece>,
    },
    CastlingShort,
    CastlingLong,
    Promovation,
}

#[derive(Debug)]
pub struct ChessGame {
    board: [[Option<Piece>; 8]; 8],
    pub move_stack: Vec<Move>,
    current_player: Players,
}

impl ChessGame {
    pub fn new() -> Self {
        #[rustfmt::skip]
        let game = ChessGame {
            board: [
                [
                    Some(Piece {piece_type: PieceTypes::Rook, owner: Players::White}),
                    Some(Piece {piece_type: PieceTypes::Knight, owner: Players::White}),
                    Some(Piece {piece_type: PieceTypes::Bishop, owner: Players::White}),
                    Some(Piece {piece_type: PieceTypes::Queen, owner: Players::White}),
                    Some(Piece {piece_type: PieceTypes::King, owner: Players::White}),
                    Some(Piece {piece_type: PieceTypes::Bishop, owner: Players::White}),
                    Some(Piece {piece_type: PieceTypes::Knight, owner: Players::White}),
                    Some(Piece {piece_type: PieceTypes::Rook, owner: Players::White}),
                ],
                [Some(Piece {piece_type: PieceTypes::Pawn, owner: Players::White}); 8],
                [None; 8],
                [None; 8],
                [None; 8],
                [None; 8],
                [Some(Piece {piece_type: PieceTypes::Pawn, owner: Players::Black}); 8],
                [
                    Some(Piece {piece_type: PieceTypes::Rook, owner: Players::Black}),
                    Some(Piece {piece_type: PieceTypes::Knight, owner: Players::Black}),
                    Some(Piece {piece_type: PieceTypes::Bishop, owner: Players::Black}),
                    Some(Piece {piece_type: PieceTypes::Queen, owner: Players::Black}),
                    Some(Piece {piece_type: PieceTypes::King, owner: Players::Black}),
                    Some(Piece {piece_type: PieceTypes::Bishop, owner: Players::Black}),
                    Some(Piece {piece_type: PieceTypes::Knight, owner: Players::Black}),
                    Some(Piece {piece_type: PieceTypes::Rook, owner: Players::Black}),
                ],
            ],
            move_stack: Vec::with_capacity(100),
            current_player: Players::White, 
        };

        game
    }

    pub fn get_position(&self, position: Position) -> Option<&Option<Piece>> {
        self.board
            .get(position.row() as usize)
            .and_then(|row| row.get(position.col() as usize))
    }

    fn set_position(&mut self, position: Position, new_place: Option<Piece>) {
        self.board.get_mut(position.row() as usize).and_then(|row| {
            row.get_mut(position.col() as usize)
                .map(|place| *place = new_place)
        });
    }

    pub fn push(&mut self, _move: Move) {
        self.move_stack.push(_move);
        match _move {
            Move::Normal {
                piece, start, end, ..
            } => {
                self.set_position(start, None);
                self.set_position(end, Some(piece));
            }
            // TODO: other moves
            _ => (),
        };
    }

    pub fn pop(&mut self) -> Move {
        let _move = self.move_stack.pop().expect("Tried to pop a new game");

        match _move {
            Move::Normal {
                piece,
                start,
                end,
                captured_piece,
            } => {
                self.set_position(start, Some(piece));
                self.set_position(end, captured_piece);
            }
            // TODO: other moves
            _ => (),
        };

        _move
    }

    pub fn get_moves(&self) -> Vec<Move> {
        self.board
            .iter()
            .enumerate()
            .flat_map(|(r, v)| {
                v.iter()
                    .enumerate()
                    .map(move |(c, v)| (Position::new(r as i8, c as i8), v))
            })
            .filter_map(|(position, place)| {
                place
                    .filter(|piece| piece.owner == self.current_player)
                    .map(|_| (position, place))
            })
            .flat_map(|(position, place)| {
                place
                    .map(|piece| piece.get_moves(self, position))
                    .unwrap_or_default()
            })
            .collect()
    }
}

macro_rules! find_moves_loops {
    ( $moves:ident, $pos:ident, $game:ident, $piece_type:ident, $( $x:expr ),* ) => {
        {
            $(
            for delta in $x {
                let end = $pos + delta;
                if let Some(place) = $game.get_position(end) {
                    if place.is_some_and(|piece| piece.owner == $game.current_player) {
                        break;
                    } else {
                        $moves.push(Move::Normal {
                            piece: *$piece_type,
                            start: $pos,
                            end,
                            captured_piece: *place,
                        });
                        if place.is_some()
                        {
                            break;
                        }
                    }
                } else {
                    break;
                }
            }
            )*
        }
    };
}

impl Piece {
    fn owner(&self) -> Players {
        match self {
            WhitePawn => Players::White,
            WhiteRook => Players::White,
            WhiteKnight => Players::White,
            WhiteBishop => Players::White,
            WhiteQueen => Players::White,
            WhiteKing => Players::White,
            BlackPawn => Players::Black,
            BlackRook => Players::Black,
            BlackKnight => Players::Black,
            BlackBishop => Players::Black,
            BlackQueen => Players::Black,
            BlackKing => Players::Black,
        }
    }

    fn get_moves(&self, game: &ChessGame, pos: Position) -> Vec<Move> {
        let mut moves = Vec::new();
        match self {
            Piece::WhitePawn | Piece::BlackPawn => {
                let first_row = match self.owner() {
                    Players::White => 1,
                    Players::Black => 6,
                };

                let normal_delta = match self.owner() {
                    Players::White => Position::new(1, 0),
                    Players::Black => Position::new(-1, 0),
                };

                let first_row_delta = match self.owner() {
                    Players::White => Position::new(2, 0),
                    Players::Black => Position::new(-2, 0),
                };

                if pos.row() == first_row
                    && game.get_position(pos + normal_delta).unwrap().is_none()
                    && game.get_position(pos + first_row_delta).unwrap().is_none()
                {
                    moves.push(Move::Normal {
                        piece: *self,
                        start: pos,
                        end: pos + first_row_delta,
                        captured_piece: None,
                    });
                }
                let side_deltas = match self.owner() {
                    Players::White => [Position::new(1, 1), Position::new(1, -1)],
                    Players::Black => [Position::new(-1, 1), Position::new(-1, -1)],
                };

                if game
                    .get_position(pos + normal_delta)
                    .is_some_and(|place| place.is_none())
                {
                    moves.push(Move::Normal {
                        piece: *self,
                        start: pos,
                        end: pos + normal_delta,
                        captured_piece: None,
                    });
                }
                for delta in side_deltas {
                    if let Some(place) = game.get_position(pos + delta) {
                        if place.is_some_and(|piece| piece.owner != self.owner()) {
                            moves.push(Move::Normal {
                                piece: *self,
                                start: pos,
                                end: pos + delta,
                                captured_piece: *place,
                            });
                        }
                    }
                }
                // TODO: En Passant

                // TODO: Promotion
            }
            PieceTypes::King => {}
            //     for (a, b) in [
            //         (0, 1),
            //         (0, -1),
            //         (1, 0),
            //         (-1, 0),
            //         (1, 1),
            //         (1, -1),
            //         (-1, 1),
            //         (-1, -1),
            //     ] {
            //         let new_pos = (pos.0 + a, pos.1 + b);
            //         game.push(Move::Normal {
            //             piece: self,
            //             start: pos,
            //             end: new_pos,
            //             is_capture: game.get_position(new_pos).is_some(),
            //         });
            //         game.pop();
            //         if let Some(place) = game.get_position(new_pos) {
            //             if game.get_moves().iter().any(|mv| match mv {
            //                 Move::Normal { end, .. } => *end == new_pos,
            //                 _ => false,
            //             }) {
            //                 moves.push(Move::Normal {
            //                     piece: *self,
            //                     start: pos,
            //                     end: (pos.0 + a, pos.1 + b),
            //                     is_capture: place.is_some(),
            //                 });
            //                 if place.is_some_and(|piece| piece.owner != game.current_player) {
            //                     break;
            //                 }
            //             }
            //         } else {
            //             break;
            //         }
            //     }
            // }
            PieceTypes::Knight => {
                for delta in [
                    (1, 2),
                    (2, 1),
                    (-1, -2),
                    (-2, -1),
                    (1, -2),
                    (-2, -1),
                    (-1, 2),
                    (2, -1),
                ]
                .iter()
                .map(|(row, col)| Position::new(*row, *col))
                {
                    if let Some(place) = game.get_position(pos + delta) {
                        if place.is_none()
                            || place.is_some_and(|piece| piece.owner != game.current_player)
                        {
                            moves.push(Move::Normal {
                                piece: *self,
                                start: pos,
                                end: pos + delta,
                                captured_piece: *place,
                            });
                        }
                    }
                }
            }
            PieceTypes::Rook => {
                find_moves_loops![
                    moves,
                    pos,
                    game,
                    self,
                    (1..).map(|x| Position::new(0, x)),
                    (1..).map(|x| Position::new(0, -x)),
                    (1..).map(|x| Position::new(x, 0)),
                    (1..).map(|x| Position::new(-x, 0))
                ];
            }

            PieceTypes::Bishop => {
                find_moves_loops![
                    moves,
                    pos,
                    game,
                    self,
                    (1..).map(|x| Position::new(x, x)),
                    (1..).map(|x| Position::new(-x, -x)),
                    (1..).map(|x| Position::new(x, -x)),
                    (1..).map(|x| Position::new(-x, x))
                ];
            }

            PieceTypes::Queen => {
                find_moves_loops![
                    moves,
                    pos,
                    game,
                    self,
                    (1..).map(|x| Position::new(0, x)),
                    (1..).map(|x| Position::new(0, -x)),
                    (1..).map(|x| Position::new(x, 0)),
                    (1..).map(|x| Position::new(-x, 0)),
                    (1..).map(|x| Position::new(x, x)),
                    (1..).map(|x| Position::new(-x, -x)),
                    (1..).map(|x| Position::new(x, -x)),
                    (1..).map(|x| Position::new(-x, x))
                ];
            }
        }

        moves
    }
}

impl std::fmt::Debug for Move {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Move::Normal {
                piece,
                start,
                end,
                captured_piece,
            } => write!(
                f,
                "{:?} from {} {} to {} {}, captured {:?} ",
                piece,
                start.row(),
                start.col(),
                end.row(),
                end.col(),
                captured_piece
            ),
            _ => write!(f, "not supported"),
        }
    }
}
// impl Piece {
//     fn is_white(&self) -> bool {
//         match self {
//             Piece::WhitePawn => true,
//             Piece::WhiteRook => true,
//             Piece::WhiteKnight => true,
//             Piece::WhiteBishop => true,
//             Piece::WhiteQueen => true,
//             Piece::WhiteKing => true,
//             _ => false,
//         }
//     }

//     fn is_black(&self) -> bool {
//         !self.is_white()
//     }
// }

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn it_works() {
//         let result = add(2, 2);
//         assert_eq!(result, 4);
//     }
// }
