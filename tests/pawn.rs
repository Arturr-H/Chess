#[cfg(test)]
mod tests {
    use chess::{ board::Board, bundle::pawn::Pawn, piece::Color, traits::PieceMethods };

    #[test]
    fn move_() -> () {
        let mut board = Board::new();
        board.move_piece_to_coordinate((0, 6), (0, 5)).unwrap();
    }

    #[test]
    fn white_moves() -> () {
        let pawn = Pawn { color: Color::White, has_moved: true };
        assert_eq!(pawn.get_moves_local() == vec![(0, -1)], true);
    }

    #[test]
    fn black_moves() -> () {
        let pawn = Pawn { color: Color::Black, has_moved: true };
        assert_eq!(pawn.get_moves_local() == vec![(0, 1)], true);
    }
}
