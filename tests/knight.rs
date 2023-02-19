#[cfg(test)]
mod tests {
    use chess::board::Board;

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
}
