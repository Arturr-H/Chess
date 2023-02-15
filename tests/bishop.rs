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
        let rook_w = Bishop { color: Color::White };
        let rook_b = Bishop { color: Color::Black };

        assert_eq!(rook_w.get_moves_local().contains(&(1, 1)), true);
        assert_eq!(rook_b.get_moves_local().contains(&(-1, 1)), true);
        assert_eq!(rook_b.get_moves_local().contains(&(0, 1)), false);
    }
}
