#[cfg(test)]
mod tests {
    use chess::{ board::Board, bundle::king::King, piece::Color, traits::PieceMethods };

    #[test]
    #[should_panic]
    fn move_() -> () {
        let mut board = Board::new();

        /* Kings cant initially move, therefore this test should panic */
        board.move_piece_to_coordinate((2, 0), (3, 1)).unwrap();
    }

    #[test]
    fn white_moves() -> () {
        let king_w = King { color: Color::White };
        let king_b = King { color: Color::Black };

        assert_eq!(king_w.get_moves_local().contains(&(1, 1)), true);
        assert_eq!(king_b.get_moves_local().contains(&(-1, 1)), true);
        assert_eq!(king_b.get_moves_local().contains(&(0, 1)), false);
    }
}
