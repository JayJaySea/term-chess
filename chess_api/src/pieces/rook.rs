use super::{Piece, PieceColor};
use crate::movement::Move;
use crate::board::Board;

pub struct Rook {
    color: PieceColor
}

impl Rook {
    pub fn new(color: PieceColor) -> Rook {
        Rook {
            color
        }
    }
}

impl Piece for Rook {
    fn can_move_to(&self, _b: &Board, m: Move) -> (bool, bool) {
        let ((sx, sy), (ex, ey)) = m.to_coords();
        ( sx == ex || sy == ey, true )
    }

    fn get_character(&self) -> char {
        match self.color {
            PieceColor::BLACK => 'r',
            PieceColor::WHITE => 'R',
        }
    }

    fn color(&self) -> PieceColor {
        self.color
    }
}


#[cfg(test)]
mod test {
    use super::*;
    use crate::movement::Square;

    #[test]
    fn basic_rook_movement() {
        let mut board = Board::new_clear();

        board.set(Square::new(1, 1), Some(Box::new(Rook::new(PieceColor::WHITE))));
        
        assert_eq!(board.is_move_possible(Move::new(Square::new(1, 1), Square::new(1, 5))), true);
        assert_eq!(board.is_move_possible(Move::new(Square::new(1, 1), Square::new(5, 1))), true);
   
        assert_eq!(board.is_move_possible(Move::new(Square::new(1, 1), Square::new(5, 5))), false);
        assert_eq!(board.is_move_possible(Move::new(Square::new(1, 1), Square::new(5, 2))), false);

        board.set(Square::new(1, 1), None);

        board.set(Square::new(5, 5), Some(Box::new(Rook::new(PieceColor::WHITE))));

        assert_eq!(board.is_move_possible(Move::new(Square::new(5, 5), Square::new(1, 5))), true);
        assert_eq!(board.is_move_possible(Move::new(Square::new(5, 5), Square::new(5, 1))), true);
    }

    #[test]
    fn rook_taking_oppnent_pieces() {
        let mut board = Board::new_clear();

        board.set(Square::new(1, 1), Some(Box::new(Rook::new(PieceColor::WHITE))));
        board.set(Square::new(1, 5), Some(Box::new(Rook::new(PieceColor::BLACK))));
        board.set(Square::new(5, 1), Some(Box::new(Rook::new(PieceColor::BLACK))));

        assert_eq!(board.is_move_possible(Move::new(Square::new(1, 1), Square::new(1, 5))), true);
        assert_eq!(board.is_move_possible(Move::new(Square::new(1, 1), Square::new(5, 1))), true);
    }

    #[test]
    fn rook_taking_allied_pieces() {
        let mut board = Board::new_clear();

        board.set(Square::new(1, 1), Some(Box::new(Rook::new(PieceColor::WHITE))));
        board.set(Square::new(1, 5), Some(Box::new(Rook::new(PieceColor::WHITE))));
        board.set(Square::new(5, 1), Some(Box::new(Rook::new(PieceColor::WHITE))));

        assert_eq!(board.is_move_possible(Move::new(Square::new(1, 1), Square::new(1, 5))), false);
        assert_eq!(board.is_move_possible(Move::new(Square::new(1, 1), Square::new(5, 1))), false);
    }

    #[test]
    fn rook_blocked() {
        let mut board = Board::new_clear();

        board.set(Square::new(1, 1), Some(Box::new(Rook::new(PieceColor::WHITE))));
        board.set(Square::new(1, 5), Some(Box::new(Rook::new(PieceColor::WHITE))));
        board.set(Square::new(5, 1), Some(Box::new(Rook::new(PieceColor::BLACK))));
    
        assert_eq!(board.is_move_possible(Move::new(Square::new(1, 1), Square::new(1, 6))), false);
        assert_eq!(board.is_move_possible(Move::new(Square::new(1, 1), Square::new(6, 1))), false);
    }
}
