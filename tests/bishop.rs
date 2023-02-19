#[cfg(test)]
mod tests {
    use chess::{ board::Board, bundle::bishop::Bishop, piece::Color, traits::PieceMethods };

    #[test]
    #[should_panic]
    fn move_() -> () {
        let mut board = Board::new();

        /* Bishops cant initially move, therefore this test should panic */
        board.move_piece_to_coordinate((2, 0), (3, 1)).unwrap();
    }

    #[test]
    fn white_moves() -> () {
        let bishop_w = Bishop { color: Color::White };
        let bishop_b = Bishop { color: Color::Black };

        assert_eq!(bishop_w.get_moves_local(&Board::new()).contains(&(1, 1)), true);
        assert_eq!(bishop_b.get_moves_local(&Board::new()).contains(&(-1, 1)), true);
        assert_eq!(bishop_b.get_moves_local(&Board::new()).contains(&(0, 1)), false);
    }
}
