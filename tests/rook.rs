#[cfg(test)]
mod tests {
    use chess::{ board::Board, bundle::rook::Rook, piece::Color, traits::PieceMethods };

    #[test]
    #[should_panic]
    fn move_() -> () {
        let mut board = Board::new();

        /* Rooks cant initially move, therefore this test should panic */
        board.move_piece_to_coordinate((0, 0), (0, 2)).unwrap();
    }

    #[test]
    fn white_moves() -> () {
        let rook_w = Rook { color: Color::White };
        let rook_b = Rook { color: Color::Black };

        assert_eq!(rook_w.get_moves_local().contains(&(2, 0)), true);
        assert_eq!(rook_b.get_moves_local().contains(&(0, -4)), true);
        assert_eq!(rook_b.get_moves_local().contains(&(1, 1)), false);
    }
}
