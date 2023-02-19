#[cfg(test)]
mod tests {
    use chess::board::Board;

    #[test]
    #[should_panic]
    fn move_() -> () {
        let mut board = Board::new();

        /* Queens cant initially move, therefore this test should panic */
        board.move_piece_to_coordinate((3, 0), (3, 1)).unwrap();
    }
}
