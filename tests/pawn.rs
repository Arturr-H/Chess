#[cfg(test)]
mod tests {
    use chess::board::Board;

    #[test]
    fn move_() -> () {
        let mut board = Board::new();
        board.move_piece_to_coordinate((0, 6), (0, 5)).unwrap();
    }
}
