mod chess_board;
mod pieces;

fn main() {
    let mut chess_board = chess_board::ChessBoard::new();
    chess_board.begin().unwrap();
}
