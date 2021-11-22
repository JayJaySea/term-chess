use crate::movement::{Move, Square};
use crate::board::Board;
use std::ops::Not;


#[derive(Debug, PartialEq, Copy, Clone)]
pub enum PieceColor {
    WHITE,
    BLACK
}

impl Not for PieceColor {
    type Output = Self;

    fn not(self) -> Self::Output {
        match self {
            PieceColor::WHITE => PieceColor::BLACK,
            PieceColor::BLACK => PieceColor::WHITE
        }
    }
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum PieceType {
    Pawn, Rook, Knight, Bishop, Queen, King
}

#[derive(Debug)]
pub struct Piece {
    piece_type: PieceType,
    piece_color: PieceColor,
    moved: bool
}




impl Piece {
    pub fn new(piece_type: PieceType, piece_color: PieceColor) -> Piece {
        Piece {
            piece_type,
            piece_color,
            moved: false
        }
    }

    pub fn can_move_to(&self, board: &Board, m: Move) -> (bool, bool) {
        let (dx, dy) = m.to_deltas();

        match self.piece_type {
            PieceType::Knight => ( (dx == 2 && dy == 1) || (dx == 1 && dy == 2), false ),
            PieceType::Queen => ( dx == 0 || dy == 0 || dx == dy, true),
            PieceType::King => ( dx <= 1 && dy <= 1, false ),
            PieceType::Rook => ( dx == 0 || dy == 0, true ),
            PieceType::Bishop => ( dx == dy, true ),
            PieceType::Pawn => ({
                let dest_occupied = board.get_piece(m.end()).is_some();

                let ((sx, sy), (ex, ey)) = m.to_coords();

                let (distance, forward) = match self.piece_color {
                    PieceColor::WHITE => {
                        if ey > sy { 
                            (ey - sy, sy + 1)
                        } else {
                            return (false, false);
                        } 
                    },
                    PieceColor::BLACK => {
                        if ey < sy {
                            (sy - ey, sy - 1)
                        } else {
                            return (false, false);
                        }
                    }
                };


                if sx == ex && !dest_occupied {
                    match distance {
                        1 => true,
                        2 => !self.moved && board.get_piece(Square::new(ex, forward)).is_none(),
                        _ => false
                    }
                } else if dest_occupied && distance == 1 {
                    if sx > ex {
                        sx - ex == 1 
                    } else if sx < ex {
                        ex - sx == 1 
                    } else { false }
                } else { false } // todo en passant 
            }, false),
        }
    }

    pub fn color(&self) -> PieceColor {
        self.piece_color
    }

    pub fn piece_type(&self) -> PieceType {
        self.piece_type
    }
}

#[cfg(test)]
mod test {
    use super::*;
        
    #[test]
    fn basic_white_pawn_movement() {
        let mut board = Board::new_clear();

        board.set(Square::new(3, 3), Some(Piece::new(PieceType::Pawn, PieceColor::WHITE)));
    
        assert_eq!(board.is_move_possible(Move::new(Square::new(3, 3), Square::new(3, 4))), true);
        assert_eq!(board.is_move_possible(Move::new(Square::new(3, 3), Square::new(3, 5))), true);

        assert_eq!(board.is_move_possible(Move::new(Square::new(3, 3), Square::new(3, 2))), false);

        assert_eq!(board.is_move_possible(Move::new(Square::new(3, 3), Square::new(4, 2))), false);
        assert_eq!(board.is_move_possible(Move::new(Square::new(3, 3), Square::new(5, 2))), false);
        assert_eq!(board.is_move_possible(Move::new(Square::new(3, 3), Square::new(4, 3))), false);
        assert_eq!(board.is_move_possible(Move::new(Square::new(3, 3), Square::new(5, 3))), false);
        assert_eq!(board.is_move_possible(Move::new(Square::new(3, 3), Square::new(4, 4))), false);
        assert_eq!(board.is_move_possible(Move::new(Square::new(3, 3), Square::new(5, 4))), false);
    }


    #[test]
    fn basic_black_pawn_movement() {
        let mut board = Board::new_clear();

        board.set(Square::new(3, 3), Some(Piece::new(PieceType::Pawn, PieceColor::BLACK)));
    
        assert_eq!(board.is_move_possible(Move::new(Square::new(3, 3), Square::new(3, 2))), true);
        assert_eq!(board.is_move_possible(Move::new(Square::new(3, 3), Square::new(3, 1))), true);

        assert_eq!(board.is_move_possible(Move::new(Square::new(3, 3), Square::new(3, 4))), false);

        assert_eq!(board.is_move_possible(Move::new(Square::new(3, 3), Square::new(4, 2))), false);
        assert_eq!(board.is_move_possible(Move::new(Square::new(3, 3), Square::new(5, 2))), false);
        assert_eq!(board.is_move_possible(Move::new(Square::new(3, 3), Square::new(4, 3))), false);
        assert_eq!(board.is_move_possible(Move::new(Square::new(3, 3), Square::new(5, 3))), false);
        assert_eq!(board.is_move_possible(Move::new(Square::new(3, 3), Square::new(4, 4))), false);
        assert_eq!(board.is_move_possible(Move::new(Square::new(3, 3), Square::new(5, 4))), false);
    }

    #[test]
    fn blocked_basic_white_pawn_movement() {
        let mut board = Board::new_clear();

        board.set(Square::new(3, 3), Some(Piece::new(PieceType::Pawn, PieceColor::WHITE)));
        board.set(Square::new(3, 5), Some(Piece::new(PieceType::Pawn, PieceColor::BLACK)));
    
        assert_eq!(board.is_move_possible(Move::new(Square::new(3, 3), Square::new(3, 4))), true);
        assert_eq!(board.is_move_possible(Move::new(Square::new(3, 3), Square::new(3, 5))), false);

        board.set(Square::new(3, 3), None);
        board.set(Square::new(3, 4), Some(Piece::new(PieceType::Pawn, PieceColor::BLACK)));

        assert_eq!(board.is_move_possible(Move::new(Square::new(3, 3), Square::new(3, 4))), false);
        assert_eq!(board.is_move_possible(Move::new(Square::new(3, 3), Square::new(3, 5))), false);
    }

    #[test]
    fn blocked_basic_black_pawn_movement() {
        let mut board = Board::new_clear();

        board.set(Square::new(3, 3), Some(Piece::new(PieceType::Pawn, PieceColor::BLACK)));
        board.set(Square::new(3, 1), Some(Piece::new(PieceType::Pawn, PieceColor::WHITE)));
    
        assert_eq!(board.is_move_possible(Move::new(Square::new(3, 3), Square::new(3, 2))), true);
        assert_eq!(board.is_move_possible(Move::new(Square::new(3, 3), Square::new(3, 1))), false);

        board.set(Square::new(3, 3), None);
        board.set(Square::new(3, 2), Some(Piece::new(PieceType::Pawn, PieceColor::WHITE)));

        assert_eq!(board.is_move_possible(Move::new(Square::new(3, 3), Square::new(3, 2))), false);
        assert_eq!(board.is_move_possible(Move::new(Square::new(3, 3), Square::new(3, 1))), false);
    }

    #[test]
    fn white_pawn_taking_opponent_pieces() {
        let mut board = Board::new_clear();

        board.set(Square::new(3, 3), Some(Piece::new(PieceType::Pawn, PieceColor::WHITE)));
        board.set(Square::new(4, 5), Some(Piece::new(PieceType::Pawn, PieceColor::BLACK)));
        board.set(Square::new(2, 5), Some(Piece::new(PieceType::Pawn, PieceColor::BLACK)));
        board.set(Square::new(4, 4), Some(Piece::new(PieceType::Pawn, PieceColor::BLACK)));
        board.set(Square::new(2, 4), Some(Piece::new(PieceType::Pawn, PieceColor::BLACK)));
        board.set(Square::new(4, 3), Some(Piece::new(PieceType::Pawn, PieceColor::BLACK)));
        board.set(Square::new(2, 3), Some(Piece::new(PieceType::Pawn, PieceColor::BLACK)));
        board.set(Square::new(4, 2), Some(Piece::new(PieceType::Pawn, PieceColor::BLACK)));
        board.set(Square::new(2, 2), Some(Piece::new(PieceType::Pawn, PieceColor::BLACK)));
        board.set(Square::new(5, 4), Some(Piece::new(PieceType::Pawn, PieceColor::BLACK)));
        board.set(Square::new(1, 4), Some(Piece::new(PieceType::Pawn, PieceColor::BLACK)));

        assert_eq!(board.is_move_possible(Move::new(Square::new(3, 3), Square::new(4, 5))), false);
        assert_eq!(board.is_move_possible(Move::new(Square::new(3, 3), Square::new(2, 5))), false);
        assert_eq!(board.is_move_possible(Move::new(Square::new(3, 3), Square::new(4, 4))), true);
        assert_eq!(board.is_move_possible(Move::new(Square::new(3, 3), Square::new(2, 4))), true);
        assert_eq!(board.is_move_possible(Move::new(Square::new(3, 3), Square::new(4, 3))), false);
        assert_eq!(board.is_move_possible(Move::new(Square::new(3, 3), Square::new(2, 3))), false);
        assert_eq!(board.is_move_possible(Move::new(Square::new(3, 3), Square::new(4, 2))), false);
        assert_eq!(board.is_move_possible(Move::new(Square::new(3, 3), Square::new(2, 2))), false);
        assert_eq!(board.is_move_possible(Move::new(Square::new(3, 3), Square::new(5, 4))), false);
        assert_eq!(board.is_move_possible(Move::new(Square::new(3, 3), Square::new(1, 4))), false);
    }

    #[test]
    fn black_pawn_taking_opponent_pieces() {
        let mut board = Board::new_clear();

        board.set(Square::new(3, 3), Some(Piece::new(PieceType::Pawn, PieceColor::BLACK)));
        board.set(Square::new(4, 1), Some(Piece::new(PieceType::Pawn, PieceColor::WHITE)));
        board.set(Square::new(2, 1), Some(Piece::new(PieceType::Pawn, PieceColor::WHITE)));
        board.set(Square::new(4, 2), Some(Piece::new(PieceType::Pawn, PieceColor::WHITE)));
        board.set(Square::new(2, 2), Some(Piece::new(PieceType::Pawn, PieceColor::WHITE)));
        board.set(Square::new(4, 3), Some(Piece::new(PieceType::Pawn, PieceColor::WHITE)));
        board.set(Square::new(2, 3), Some(Piece::new(PieceType::Pawn, PieceColor::WHITE)));
        board.set(Square::new(4, 3), Some(Piece::new(PieceType::Pawn, PieceColor::WHITE)));
        board.set(Square::new(2, 3), Some(Piece::new(PieceType::Pawn, PieceColor::WHITE)));
        board.set(Square::new(5, 2), Some(Piece::new(PieceType::Pawn, PieceColor::WHITE)));
        board.set(Square::new(1, 2), Some(Piece::new(PieceType::Pawn, PieceColor::WHITE)));

        assert_eq!(board.is_move_possible(Move::new(Square::new(3, 3), Square::new(4, 1))), false);
        assert_eq!(board.is_move_possible(Move::new(Square::new(3, 3), Square::new(2, 1))), false);
        assert_eq!(board.is_move_possible(Move::new(Square::new(3, 3), Square::new(4, 2))), true);
        assert_eq!(board.is_move_possible(Move::new(Square::new(3, 3), Square::new(2, 2))), true);
        assert_eq!(board.is_move_possible(Move::new(Square::new(3, 3), Square::new(4, 3))), false);
        assert_eq!(board.is_move_possible(Move::new(Square::new(3, 3), Square::new(2, 3))), false);
        assert_eq!(board.is_move_possible(Move::new(Square::new(3, 3), Square::new(4, 4))), false);
        assert_eq!(board.is_move_possible(Move::new(Square::new(3, 3), Square::new(2, 4))), false);
        assert_eq!(board.is_move_possible(Move::new(Square::new(3, 3), Square::new(5, 2))), false);
        assert_eq!(board.is_move_possible(Move::new(Square::new(3, 3), Square::new(1, 2))), false);
    }

    #[test]
    fn white_pawn_taking_allied_pieces() {
        let mut board = Board::new_clear();

        board.set(Square::new(3, 3), Some(Piece::new(PieceType::Pawn, PieceColor::WHITE)));
        board.set(Square::new(4, 4), Some(Piece::new(PieceType::Pawn, PieceColor::WHITE)));
        board.set(Square::new(2, 4), Some(Piece::new(PieceType::Pawn, PieceColor::WHITE)));

        assert_eq!(board.is_move_possible(Move::new(Square::new(3, 3), Square::new(4, 4))), false);
        assert_eq!(board.is_move_possible(Move::new(Square::new(3, 3), Square::new(2, 4))), false);
    }

    #[test]
    fn black_pawn_taking_allied_pieces() {
        let mut board = Board::new_clear();

        board.set(Square::new(3, 3), Some(Piece::new(PieceType::Pawn, PieceColor::BLACK)));
        board.set(Square::new(4, 2), Some(Piece::new(PieceType::Pawn, PieceColor::BLACK)));
        board.set(Square::new(2, 2), Some(Piece::new(PieceType::Pawn, PieceColor::BLACK)));

        assert_eq!(board.is_move_possible(Move::new(Square::new(3, 3), Square::new(4, 2))), false);
        assert_eq!(board.is_move_possible(Move::new(Square::new(3, 3), Square::new(2, 2))), false);
    }

    #[test]
    fn basic_bishop_movement() {
        let mut board = Board::new_clear();

        board.set(Square::new(3, 3), Some(Piece::new(PieceType::Bishop, PieceColor::WHITE)));

        assert_eq!(board.is_move_possible(Move::new(Square::new(3, 3), Square::new(2, 2))), true);
        assert_eq!(board.is_move_possible(Move::new(Square::new(3, 3), Square::new(2, 4))), true);
        assert_eq!(board.is_move_possible(Move::new(Square::new(3, 3), Square::new(4, 2))), true);
        assert_eq!(board.is_move_possible(Move::new(Square::new(3, 3), Square::new(4, 4))), true);
        
        assert_eq!(board.is_move_possible(Move::new(Square::new(3, 3), Square::new(0, 0))), true);
        assert_eq!(board.is_move_possible(Move::new(Square::new(3, 3), Square::new(0, 6))), true);
        assert_eq!(board.is_move_possible(Move::new(Square::new(3, 3), Square::new(6, 0))), true);
        assert_eq!(board.is_move_possible(Move::new(Square::new(3, 3), Square::new(7, 7))), true);

        assert_eq!(board.is_move_possible(Move::new(Square::new(3, 3), Square::new(3, 2))), false);
        assert_eq!(board.is_move_possible(Move::new(Square::new(3, 3), Square::new(3, 4))), false);
        assert_eq!(board.is_move_possible(Move::new(Square::new(3, 3), Square::new(2, 3))), false);
        assert_eq!(board.is_move_possible(Move::new(Square::new(3, 3), Square::new(4, 3))), false);
        
        assert_eq!(board.is_move_possible(Move::new(Square::new(3, 3), Square::new(0, 2))), false);
        assert_eq!(board.is_move_possible(Move::new(Square::new(3, 3), Square::new(2, 6))), false);
        assert_eq!(board.is_move_possible(Move::new(Square::new(3, 3), Square::new(6, 2))), false);
        assert_eq!(board.is_move_possible(Move::new(Square::new(3, 3), Square::new(7, 6))), false);
    }

    #[test]
    fn bishop_taking_opponent_pieces() {
        let mut board = Board::new_clear();

        board.set(Square::new(3, 3), Some(Piece::new(PieceType::Bishop, PieceColor::WHITE)));
        
        board.set(Square::new(2, 4), Some(Piece::new(PieceType::Bishop, PieceColor::BLACK)));
        board.set(Square::new(2, 2), Some(Piece::new(PieceType::Bishop, PieceColor::BLACK)));
        board.set(Square::new(4, 2), Some(Piece::new(PieceType::Bishop, PieceColor::BLACK)));
        board.set(Square::new(4, 4), Some(Piece::new(PieceType::Bishop, PieceColor::BLACK)));

        assert_eq!(board.is_move_possible(Move::new(Square::new(3, 3), Square::new(2, 2))), true);
        assert_eq!(board.is_move_possible(Move::new(Square::new(3, 3), Square::new(2, 4))), true);
        assert_eq!(board.is_move_possible(Move::new(Square::new(3, 3), Square::new(4, 2))), true);
        assert_eq!(board.is_move_possible(Move::new(Square::new(3, 3), Square::new(4, 4))), true);

        let mut board = Board::new_clear();

        board.set(Square::new(3, 3), Some(Piece::new(PieceType::Bishop, PieceColor::WHITE)));
        
        board.set(Square::new(0, 0), Some(Piece::new(PieceType::Bishop, PieceColor::BLACK)));
        board.set(Square::new(0, 6), Some(Piece::new(PieceType::Bishop, PieceColor::BLACK)));
        board.set(Square::new(6, 0), Some(Piece::new(PieceType::Bishop, PieceColor::BLACK)));
        board.set(Square::new(7, 7), Some(Piece::new(PieceType::Bishop, PieceColor::BLACK)));

        assert_eq!(board.is_move_possible(Move::new(Square::new(3, 3), Square::new(0, 0))), true);
        assert_eq!(board.is_move_possible(Move::new(Square::new(3, 3), Square::new(0, 6))), true);
        assert_eq!(board.is_move_possible(Move::new(Square::new(3, 3), Square::new(6, 0))), true);
        assert_eq!(board.is_move_possible(Move::new(Square::new(3, 3), Square::new(7, 7))), true);
    }

    #[test]
    fn bishop_taking_allied_pieces() {
        let mut board = Board::new_clear();

        board.set(Square::new(3, 3), Some(Piece::new(PieceType::Bishop, PieceColor::WHITE)));

        board.set(Square::new(2, 4), Some(Piece::new(PieceType::Bishop, PieceColor::WHITE)));
        board.set(Square::new(2, 2), Some(Piece::new(PieceType::Bishop, PieceColor::WHITE)));
        board.set(Square::new(4, 2), Some(Piece::new(PieceType::Bishop, PieceColor::WHITE)));
        board.set(Square::new(4, 4), Some(Piece::new(PieceType::Bishop, PieceColor::WHITE)));

        assert_eq!(board.is_move_possible(Move::new(Square::new(3, 3), Square::new(2, 2))), false);
        assert_eq!(board.is_move_possible(Move::new(Square::new(3, 3), Square::new(2, 4))), false);
        assert_eq!(board.is_move_possible(Move::new(Square::new(3, 3), Square::new(4, 2))), false);
        assert_eq!(board.is_move_possible(Move::new(Square::new(3, 3), Square::new(4, 4))), false);

        let mut board = Board::new_clear();

        board.set(Square::new(3, 3), Some(Piece::new(PieceType::Bishop, PieceColor::WHITE)));
        
        board.set(Square::new(0, 0), Some(Piece::new(PieceType::Bishop, PieceColor::WHITE)));
        board.set(Square::new(0, 6), Some(Piece::new(PieceType::Bishop, PieceColor::WHITE)));
        board.set(Square::new(6, 0), Some(Piece::new(PieceType::Bishop, PieceColor::WHITE)));
        board.set(Square::new(7, 7), Some(Piece::new(PieceType::Bishop, PieceColor::WHITE)));

        assert_eq!(board.is_move_possible(Move::new(Square::new(3, 3), Square::new(0, 0))), false);
        assert_eq!(board.is_move_possible(Move::new(Square::new(3, 3), Square::new(0, 6))), false);
        assert_eq!(board.is_move_possible(Move::new(Square::new(3, 3), Square::new(6, 0))), false);
        assert_eq!(board.is_move_possible(Move::new(Square::new(3, 3), Square::new(7, 7))), false);
    }

    #[test]
    fn bishop_blocked() {
        let mut board = Board::new_clear();

        board.set(Square::new(3, 3), Some(Piece::new(PieceType::Bishop, PieceColor::WHITE)));
        
        board.set(Square::new(2, 4), Some(Piece::new(PieceType::Bishop, PieceColor::BLACK)));
        board.set(Square::new(2, 2), Some(Piece::new(PieceType::Bishop, PieceColor::BLACK)));
        board.set(Square::new(4, 2), Some(Piece::new(PieceType::Bishop, PieceColor::BLACK)));
        board.set(Square::new(4, 4), Some(Piece::new(PieceType::Bishop, PieceColor::BLACK)));

        assert_eq!(board.is_move_possible(Move::new(Square::new(3, 3), Square::new(0, 0))), false);
        assert_eq!(board.is_move_possible(Move::new(Square::new(3, 3), Square::new(0, 6))), false);
        assert_eq!(board.is_move_possible(Move::new(Square::new(3, 3), Square::new(6, 0))), false);
        assert_eq!(board.is_move_possible(Move::new(Square::new(3, 3), Square::new(7, 7))), false);
    }

    #[test]
    fn basic_knight_movement() {
        let mut board = Board::new_clear();

        board.set(Square::new(3, 3), Some(Piece::new(PieceType::Knight, PieceColor::WHITE)));

        assert_eq!(board.is_move_possible(Move::new(Square::new(3, 3), Square::new(4, 5))), true);
        assert_eq!(board.is_move_possible(Move::new(Square::new(3, 3), Square::new(5, 4))), true);
        assert_eq!(board.is_move_possible(Move::new(Square::new(3, 3), Square::new(2, 1))), true);
        assert_eq!(board.is_move_possible(Move::new(Square::new(3, 3), Square::new(1, 2))), true);
        assert_eq!(board.is_move_possible(Move::new(Square::new(3, 3), Square::new(4, 1))), true);
        assert_eq!(board.is_move_possible(Move::new(Square::new(3, 3), Square::new(5, 2))), true);
        assert_eq!(board.is_move_possible(Move::new(Square::new(3, 3), Square::new(1, 4))), true);
        assert_eq!(board.is_move_possible(Move::new(Square::new(3, 3), Square::new(2, 5))), true);

        assert_eq!(board.is_move_possible(Move::new(Square::new(3, 3), Square::new(2, 2))), false);
        assert_eq!(board.is_move_possible(Move::new(Square::new(3, 3), Square::new(2, 3))), false);
        assert_eq!(board.is_move_possible(Move::new(Square::new(3, 3), Square::new(2, 4))), false);
        assert_eq!(board.is_move_possible(Move::new(Square::new(3, 3), Square::new(3, 2))), false);
        assert_eq!(board.is_move_possible(Move::new(Square::new(3, 3), Square::new(3, 4))), false);
        assert_eq!(board.is_move_possible(Move::new(Square::new(3, 3), Square::new(4, 2))), false);
        assert_eq!(board.is_move_possible(Move::new(Square::new(3, 3), Square::new(4, 3))), false);
        assert_eq!(board.is_move_possible(Move::new(Square::new(3, 3), Square::new(4, 4))), false);
    }

    #[test]
    fn knight_movement_near_positive_board_edges() {
        let mut board = Board::new_clear();

        board.set(Square::new(6, 6), Some(Piece::new(PieceType::Knight, PieceColor::WHITE)));

        assert_eq!(board.is_move_possible(Move::new(Square::new(6, 6), Square::new(4, 7))), true);
        assert_eq!(board.is_move_possible(Move::new(Square::new(6, 6), Square::new(4, 5))), true);
        assert_eq!(board.is_move_possible(Move::new(Square::new(6, 6), Square::new(5, 4))), true);
        assert_eq!(board.is_move_possible(Move::new(Square::new(6, 6), Square::new(7, 4))), true);
    }

    #[test]
    fn knight_movement_near_negative_board_edges() {
        let mut board = Board::new_clear();

        board.set(Square::new(1, 1), Some(Piece::new(PieceType::Knight, PieceColor::WHITE)));

        assert_eq!(board.is_move_possible(Move::new(Square::new(1, 1), Square::new(0, 3))), true);
        assert_eq!(board.is_move_possible(Move::new(Square::new(1, 1), Square::new(2, 3))), true);
        assert_eq!(board.is_move_possible(Move::new(Square::new(1, 1), Square::new(3, 0))), true);
        assert_eq!(board.is_move_possible(Move::new(Square::new(1, 1), Square::new(3, 2))), true);
    }

    #[test]
    fn queen_rook_basic_movement() {
        let mut board = Board::new_clear();

        board.set(Square::new(1, 1), Some(Piece::new(PieceType::Queen, PieceColor::WHITE)));
        
        assert_eq!(board.is_move_possible(Move::new(Square::new(1, 1), Square::new(1, 5))), true);
        assert_eq!(board.is_move_possible(Move::new(Square::new(1, 1), Square::new(5, 1))), true);

        assert_eq!(board.is_move_possible(Move::new(Square::new(1, 1), Square::new(5, 2))), false);

        board.set(Square::new(1, 1), None);

        board.set(Square::new(5, 5), Some(Piece::new(PieceType::Queen, PieceColor::WHITE)));

        assert_eq!(board.is_move_possible(Move::new(Square::new(5, 5), Square::new(1, 5))), true);
        assert_eq!(board.is_move_possible(Move::new(Square::new(5, 5), Square::new(5, 1))), true);
        
    }

    #[test]
    fn queen_bishop_basic_movement() {
        let mut board = Board::new_clear();

        board.set(Square::new(3, 3), Some(Piece::new(PieceType::Queen, PieceColor::WHITE)));

        assert_eq!(board.is_move_possible(Move::new(Square::new(3, 3), Square::new(2, 2))), true);
        assert_eq!(board.is_move_possible(Move::new(Square::new(3, 3), Square::new(2, 4))), true);
        assert_eq!(board.is_move_possible(Move::new(Square::new(3, 3), Square::new(4, 2))), true);
        assert_eq!(board.is_move_possible(Move::new(Square::new(3, 3), Square::new(4, 4))), true);
        
        assert_eq!(board.is_move_possible(Move::new(Square::new(3, 3), Square::new(0, 0))), true);
        assert_eq!(board.is_move_possible(Move::new(Square::new(3, 3), Square::new(0, 6))), true);
        assert_eq!(board.is_move_possible(Move::new(Square::new(3, 3), Square::new(6, 0))), true);
        assert_eq!(board.is_move_possible(Move::new(Square::new(3, 3), Square::new(7, 7))), true);

        assert_eq!(board.is_move_possible(Move::new(Square::new(3, 3), Square::new(0, 2))), false);
        assert_eq!(board.is_move_possible(Move::new(Square::new(3, 3), Square::new(2, 6))), false);
        assert_eq!(board.is_move_possible(Move::new(Square::new(3, 3), Square::new(6, 2))), false);
        assert_eq!(board.is_move_possible(Move::new(Square::new(3, 3), Square::new(7, 6))), false);
    }

    #[test]
    fn queen_rook_taking_oppnent_pieces() {
        let mut board = Board::new_clear();

        board.set(Square::new(1, 1), Some(Piece::new(PieceType::Queen, PieceColor::WHITE)));
        board.set(Square::new(1, 5), Some(Piece::new(PieceType::Queen, PieceColor::BLACK)));
        board.set(Square::new(5, 1), Some(Piece::new(PieceType::Queen, PieceColor::BLACK)));

        assert_eq!(board.is_move_possible(Move::new(Square::new(1, 1), Square::new(1, 5))), true);
        assert_eq!(board.is_move_possible(Move::new(Square::new(1, 1), Square::new(5, 1))), true);
    }

    #[test]
    fn queen_bishop_taking_opponent_pieces() {
        let mut board = Board::new_clear();

        board.set(Square::new(3, 3), Some(Piece::new(PieceType::Queen, PieceColor::WHITE)));
        
        board.set(Square::new(2, 4), Some(Piece::new(PieceType::Queen, PieceColor::BLACK)));
        board.set(Square::new(2, 2), Some(Piece::new(PieceType::Queen, PieceColor::BLACK)));
        board.set(Square::new(4, 2), Some(Piece::new(PieceType::Queen, PieceColor::BLACK)));
        board.set(Square::new(4, 4), Some(Piece::new(PieceType::Queen, PieceColor::BLACK)));

        assert_eq!(board.is_move_possible(Move::new(Square::new(3, 3), Square::new(2, 2))), true);
        assert_eq!(board.is_move_possible(Move::new(Square::new(3, 3), Square::new(2, 4))), true);
        assert_eq!(board.is_move_possible(Move::new(Square::new(3, 3), Square::new(4, 2))), true);
        assert_eq!(board.is_move_possible(Move::new(Square::new(3, 3), Square::new(4, 4))), true);

        let mut board = Board::new_clear();

        board.set(Square::new(3, 3), Some(Piece::new(PieceType::Queen, PieceColor::WHITE)));
        
        board.set(Square::new(0, 0), Some(Piece::new(PieceType::Queen, PieceColor::BLACK)));
        board.set(Square::new(0, 6), Some(Piece::new(PieceType::Queen, PieceColor::BLACK)));
        board.set(Square::new(6, 0), Some(Piece::new(PieceType::Queen, PieceColor::BLACK)));
        board.set(Square::new(7, 7), Some(Piece::new(PieceType::Queen, PieceColor::BLACK)));

        assert_eq!(board.is_move_possible(Move::new(Square::new(3, 3), Square::new(0, 0))), true);
        assert_eq!(board.is_move_possible(Move::new(Square::new(3, 3), Square::new(0, 6))), true);
        assert_eq!(board.is_move_possible(Move::new(Square::new(3, 3), Square::new(6, 0))), true);
        assert_eq!(board.is_move_possible(Move::new(Square::new(3, 3), Square::new(7, 7))), true);
    }
   
    #[test]
    fn queen_rook_taking_allied_pieces() {
        let mut board = Board::new_clear();

        board.set(Square::new(1, 1), Some(Piece::new(PieceType::Queen, PieceColor::WHITE)));
        board.set(Square::new(1, 5), Some(Piece::new(PieceType::Queen, PieceColor::WHITE)));
        board.set(Square::new(5, 1), Some(Piece::new(PieceType::Queen, PieceColor::WHITE)));

        assert_eq!(board.is_move_possible(Move::new(Square::new(1, 1), Square::new(1, 5))), false);
        assert_eq!(board.is_move_possible(Move::new(Square::new(1, 1), Square::new(5, 1))), false);
    }

    #[test]
    fn queen_bishop_taking_allied_pieces() {
        let mut board = Board::new_clear();

        board.set(Square::new(3, 3), Some(Piece::new(PieceType::Queen, PieceColor::WHITE)));
        
        board.set(Square::new(2, 4), Some(Piece::new(PieceType::Queen, PieceColor::WHITE)));
        board.set(Square::new(2, 2), Some(Piece::new(PieceType::Queen, PieceColor::WHITE)));
        board.set(Square::new(4, 2), Some(Piece::new(PieceType::Queen, PieceColor::WHITE)));
        board.set(Square::new(4, 4), Some(Piece::new(PieceType::Queen, PieceColor::WHITE)));

        assert_eq!(board.is_move_possible(Move::new(Square::new(3, 3), Square::new(2, 2))), false);
        assert_eq!(board.is_move_possible(Move::new(Square::new(3, 3), Square::new(2, 4))), false);
        assert_eq!(board.is_move_possible(Move::new(Square::new(3, 3), Square::new(4, 2))), false);
        assert_eq!(board.is_move_possible(Move::new(Square::new(3, 3), Square::new(4, 4))), false);

        let mut board = Board::new_clear();

        board.set(Square::new(3, 3), Some(Piece::new(PieceType::Queen, PieceColor::WHITE)));
        
        board.set(Square::new(0, 0), Some(Piece::new(PieceType::Queen, PieceColor::WHITE)));
        board.set(Square::new(0, 6), Some(Piece::new(PieceType::Queen, PieceColor::WHITE)));
        board.set(Square::new(6, 0), Some(Piece::new(PieceType::Queen, PieceColor::WHITE)));
        board.set(Square::new(7, 7), Some(Piece::new(PieceType::Queen, PieceColor::WHITE)));

        assert_eq!(board.is_move_possible(Move::new(Square::new(3, 3), Square::new(0, 0))), false);
        assert_eq!(board.is_move_possible(Move::new(Square::new(3, 3), Square::new(0, 6))), false);
        assert_eq!(board.is_move_possible(Move::new(Square::new(3, 3), Square::new(6, 0))), false);
        assert_eq!(board.is_move_possible(Move::new(Square::new(3, 3), Square::new(7, 7))), false);
    }

    #[test]
    fn queen_rook_blocked() {
        let mut board = Board::new_clear();

        board.set(Square::new(1, 1), Some(Piece::new(PieceType::Queen, PieceColor::WHITE)));
        board.set(Square::new(1, 5), Some(Piece::new(PieceType::Queen, PieceColor::WHITE)));
        board.set(Square::new(5, 1), Some(Piece::new(PieceType::Queen, PieceColor::BLACK)));
    
        assert_eq!(board.is_move_possible(Move::new(Square::new(1, 1), Square::new(1, 6))), false);
        assert_eq!(board.is_move_possible(Move::new(Square::new(1, 1), Square::new(6, 1))), false);
    }

    #[test]
    fn queen_bishop_blocked() {
        let mut board = Board::new_clear();

        board.set(Square::new(3, 3), Some(Piece::new(PieceType::Queen, PieceColor::WHITE)));
        
        board.set(Square::new(2, 4), Some(Piece::new(PieceType::Queen, PieceColor::BLACK)));
        board.set(Square::new(2, 2), Some(Piece::new(PieceType::Queen, PieceColor::BLACK)));
        board.set(Square::new(4, 2), Some(Piece::new(PieceType::Queen, PieceColor::BLACK)));
        board.set(Square::new(4, 4), Some(Piece::new(PieceType::Queen, PieceColor::BLACK)));

        assert_eq!(board.is_move_possible(Move::new(Square::new(3, 3), Square::new(0, 0))), false);
        assert_eq!(board.is_move_possible(Move::new(Square::new(3, 3), Square::new(0, 6))), false);
        assert_eq!(board.is_move_possible(Move::new(Square::new(3, 3), Square::new(6, 0))), false);
        assert_eq!(board.is_move_possible(Move::new(Square::new(3, 3), Square::new(7, 7))), false);
    }

    #[test]
    fn basic_rook_movement() {
        let mut board = Board::new_clear();

        board.set(Square::new(1, 1), Some(Piece::new(PieceType::Rook, PieceColor::WHITE)));
        
        assert_eq!(board.is_move_possible(Move::new(Square::new(1, 1), Square::new(1, 5))), true);
        assert_eq!(board.is_move_possible(Move::new(Square::new(1, 1), Square::new(5, 1))), true);
   
        assert_eq!(board.is_move_possible(Move::new(Square::new(1, 1), Square::new(5, 5))), false);
        assert_eq!(board.is_move_possible(Move::new(Square::new(1, 1), Square::new(5, 2))), false);

        board.set(Square::new(1, 1), None);

        board.set(Square::new(5, 5), Some(Piece::new(PieceType::Rook, PieceColor::WHITE)));

        assert_eq!(board.is_move_possible(Move::new(Square::new(5, 5), Square::new(1, 5))), true);
        assert_eq!(board.is_move_possible(Move::new(Square::new(5, 5), Square::new(5, 1))), true);
    }

    #[test]
    fn rook_taking_oppnent_pieces() {
        let mut board = Board::new_clear();

        board.set(Square::new(1, 1), Some(Piece::new(PieceType::Rook, PieceColor::WHITE)));
        board.set(Square::new(1, 5), Some(Piece::new(PieceType::Rook, PieceColor::BLACK)));
        board.set(Square::new(5, 1), Some(Piece::new(PieceType::Rook, PieceColor::BLACK)));

        assert_eq!(board.is_move_possible(Move::new(Square::new(1, 1), Square::new(1, 5))), true);
        assert_eq!(board.is_move_possible(Move::new(Square::new(1, 1), Square::new(5, 1))), true);
    }

    #[test]
    fn rook_taking_allied_pieces() {
        let mut board = Board::new_clear();

        board.set(Square::new(1, 1), Some(Piece::new(PieceType::Rook, PieceColor::WHITE)));
        board.set(Square::new(1, 5), Some(Piece::new(PieceType::Rook, PieceColor::WHITE)));
        board.set(Square::new(5, 1), Some(Piece::new(PieceType::Rook, PieceColor::WHITE)));

        assert_eq!(board.is_move_possible(Move::new(Square::new(1, 1), Square::new(1, 5))), false);
        assert_eq!(board.is_move_possible(Move::new(Square::new(1, 1), Square::new(5, 1))), false);
    }

    #[test]
    fn rook_blocked() {
        let mut board = Board::new_clear();

        board.set(Square::new(1, 1), Some(Piece::new(PieceType::Rook, PieceColor::WHITE)));
        board.set(Square::new(1, 5), Some(Piece::new(PieceType::Rook, PieceColor::WHITE)));
        board.set(Square::new(5, 1), Some(Piece::new(PieceType::Rook, PieceColor::BLACK)));
    
        assert_eq!(board.is_move_possible(Move::new(Square::new(1, 1), Square::new(1, 6))), false);
        assert_eq!(board.is_move_possible(Move::new(Square::new(1, 1), Square::new(6, 1))), false);
    }
}
