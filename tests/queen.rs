#[cfg(test)]
mod tests {
    use chess::{ board::Board, bundle::queen::Queen, piece::Color, traits::PieceMethods };

    #[test]
    #[should_panic]
    fn move_() -> () {
        let mut board = Board::new();

        /* Queens cant initially move, therefore this test should panic */
        board.move_piece_to_coordinate((3, 0), (3, 1)).unwrap();
    }

    #[test]
    fn white_moves() -> () {
        let queen_w = Queen { color: Color::White };
        let queen_b = Queen { color: Color::Black };

        assert_eq!(queen_w.get_moves_local().contains(&(1, 1)), true);
        assert_eq!(queen_w.get_moves_local().contains(&(-1, 1)), true);
        assert_eq!(queen_b.get_moves_local().contains(&(2, 1)), false);
    }
}
