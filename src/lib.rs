#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub enum Players {
    White,
    Black,
}

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

#[derive(Clone)]
struct Board<T> {
    content: [[T; 8]; 8],
}

#[derive(Clone)]
pub struct ChessGame {
    board: [[Option<Piece>; 8]; 8],
    pub move_stack: Vec<Move>, // debug
    current_player: Players,
}

mod mod_piece;

use arrayvec::ArrayVec;
use mod_piece::*;

mod mod_position;
use mod_position::*;

impl Players {
    fn the_other(&self) -> Self {
        match self {
            Self::White => Self::Black,
            Self::Black => Self::White,
        }
    }
}

impl<T> Board<T> {
    pub fn get(&self, position: Position) -> Option<&T> {
        self.content
            .get(position.row() as usize)
            .and_then(|row| row.get(position.col() as usize))
    }

    fn set(&mut self, position: Position, new_item: T) {
        self.content
            .get_mut(position.row() as usize)
            .and_then(|row| {
                row.get_mut(position.col() as usize)
                    .map(|item| *item = new_item)
            });
    }
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
            #[rustfmt::skip]
            Move::Normal { piece, start, end, .. } => {
                self.set_position(start, None);
                self.set_position(end, Some(piece));
            }
            // TODO: other moves
            _ => (),
        };
        self.current_player = self.current_player.the_other();
    }

    pub fn pop(&mut self) -> Move {
        let _move = self.move_stack.pop().expect("Tried to pop a new game");

        match _move {
            #[rustfmt::skip]
            Move::Normal { piece, start, end, captured_piece }=> {             
                self.set_position(start, Some(piece));
                self.set_position(end, captured_piece);
            }
            // TODO: other moves
            _ => (),
        };

        self.current_player = self.current_player.the_other();
        _move
    }

    pub fn get_moves(&self) -> ArrayVec<Move, 64> {
        let piece_moves = self
            .board
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
            .map(|(position, place)| {
                place
                    .map(|piece| piece.get_moves(self, position))
                    .unwrap_or_default()
            });

        let mut moves = ArrayVec::new();
        for list in piece_moves {
            for item in list.iter() {
                unsafe {
                    moves.push_unchecked(*item);
                }
            }
        }

        moves
    }

    pub fn get_targeted(&self, player: Players) -> Board<u8> {
        let targeted = Board {
            content: [[0; 8]; 8],
        };

        targeted
    }
}

impl std::fmt::Debug for Piece {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let piece = match self.piece_type {
            PieceTypes::Pawn => "p",
            PieceTypes::Rook => "r",
            PieceTypes::Bishop => "b",
            PieceTypes::Knight => "k",
            PieceTypes::Queen => "q",
            PieceTypes::King => "_",
        };
        write!(
            f,
            "{}",
            match self.owner {
                Players::White => piece.to_owned(),
                Players::Black => piece.to_uppercase(),
            }
        )
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
                "{:?} {:?} from {} {} to {} {}, captured {:?} ",
                piece.owner,
                piece.piece_type,
                start.row(),
                start.col(),
                end.row(),
                end.col(),
                captured_piece.map(|piece| format!("{:?} {:?}", piece.owner, piece.piece_type))
            ),
            _ => write!(f, "not supported"),
        }
    }
}

impl std::fmt::Debug for ChessGame {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "\n\n{:?},\n{:?},\n{:?},\n{:?},\n{:?},\n{:?},\n{:?},\n{:?}\n",
            self.board[0].map(|place| place.unwrap_or(Piece {
                piece_type: PieceTypes::King,
                owner: Players::White
            })),
            self.board[1].map(|place| place.unwrap_or(Piece {
                piece_type: PieceTypes::King,
                owner: Players::White
            })),
            self.board[2].map(|place| place.unwrap_or(Piece {
                piece_type: PieceTypes::King,
                owner: Players::White
            })),
            self.board[3].map(|place| place.unwrap_or(Piece {
                piece_type: PieceTypes::King,
                owner: Players::White
            })),
            self.board[4].map(|place| place.unwrap_or(Piece {
                piece_type: PieceTypes::King,
                owner: Players::White
            })),
            self.board[5].map(|place| place.unwrap_or(Piece {
                piece_type: PieceTypes::King,
                owner: Players::White
            })),
            self.board[6].map(|place| place.unwrap_or(Piece {
                piece_type: PieceTypes::King,
                owner: Players::White
            })),
            self.board[7].map(|place| place.unwrap_or(Piece {
                piece_type: PieceTypes::King,
                owner: Players::White
            })),
        )
    }
}

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn it_works() {
//         let result = add(2, 2);
//         assert_eq!(result, 4);
//     }
// }
