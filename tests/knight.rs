#[cfg(test)]
mod tests {
    use chess::{ board::Board, bundle::knight::Knight, piece::Color, traits::PieceMethods };

    #[test]
    #[should_panic]
    fn move_panic() -> () {
        let mut board = Board::new();

        board.move_piece_to_coordinate((1, 0), (1, 2)).unwrap();
    }

    #[test]
    fn move_() -> () {
        let mut board = Board::new();
        board.move_piece_to_coordinate((1, 7), (2, 5)).unwrap();
    }

    #[test]
    fn white_moves() -> () {
        let knight_w = Knight { color: Color::White };
        let knight_b = Knight { color: Color::Black };

        assert_eq!(knight_w.get_moves_local().contains(&(2, 1)),   true);
        assert_eq!(knight_w.get_moves_local().contains(&(-1, -2)), true);
        assert_eq!(knight_b.get_moves_local().contains(&(0, 1)),   false);
    }
}
