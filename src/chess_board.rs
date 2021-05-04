use crate::pieces;
use console::{Style, Term};
use crossterm::event::{read, Event, KeyCode};
use std::{
    collections::HashMap,
    fmt::{Error, Write},
};

type Board = HashMap<(u8, u8), (pieces::Piece, pieces::Team)>;

#[derive(Debug)]
pub struct ChessBoard {
    pub board: Board,
    user_selected: (u8, u8),
    box_selected: Option<(u8, u8)>,
    drawn: bool,
    term: Term,
}

impl ChessBoard {
    pub fn new() -> Self {
        Self {
            board: ChessBoard::construct_board(),
            user_selected: (1, 1),
            box_selected: Option::None,
            drawn: false,
            term: Term::stdout(),
        }
    }

    fn construct_board() -> Board {
        let mut board: Board = HashMap::new();

        // Adds TOWER KNIGHTS AND BISHOPS
        let default_pieces: Vec<((u8, u8), pieces::Piece)> = vec![
            ((1, 1), pieces::Piece::Tower),
            ((2, 1), pieces::Piece::Bishop),
            ((3, 1), pieces::Piece::Knight),
        ];

        for piece in &default_pieces {
            board.insert(piece.0.clone(), (piece.1, pieces::Team::Black));
            board.insert((9 - piece.0 .0, piece.0 .1), (piece.1, pieces::Team::White));
        }
        for piece in &default_pieces {
            board.insert((piece.0 .0, 9 - piece.0 .1), (piece.1, pieces::Team::Black));
            board.insert(
                (9 - piece.0 .0, 9 - piece.0 .1),
                (piece.1, pieces::Team::White),
            );
        }

        // ADDS KING AND QUEEN
        let default_pieces: Vec<((u8, u8), pieces::Piece)> = vec![
            ((4, 1), pieces::Piece::King),
            ((5, 1), pieces::Piece::Queen),
        ];

        for piece in default_pieces {
            board.insert(piece.0, (piece.1, pieces::Team::Black));
            board.insert((piece.0 .0, 9 - piece.0 .1), (piece.1, pieces::Team::White));
        }

        // ADDS PAWN
        for role in 1..9 {
            board.insert((role, 2), (pieces::Piece::Pawn(false), pieces::Team::Black));
            board.insert((role, 7), (pieces::Piece::Pawn(false), pieces::Team::White));
        }

        board
    }

    fn get_avialable_blocks(&self) -> pieces::AvialableBlocks {
        match self.box_selected {
            Some((x, y)) => {
                let piece = self.board.get(&(x, y)).unwrap();
                pieces::get_move_blocks(&piece.0, &piece.1, &self.board, (x, y))
            }
            None => pieces::AvialableBlocks::new(),
        }
    }

    pub fn draw(&self) -> Result<(), Error> {
        if self.drawn {
            self.term.clear_last_lines(9).unwrap();
        }

        let mut result = String::new();

        let avialable_blocks: pieces::AvialableBlocks = self.get_avialable_blocks();

        for rank in 1u8..9u8 {
            for role in 1u8..9u8 {
                let empty = String::from("  ");
                let text = match self.board.get(&(role, rank)) {
                    Some(piece) => pieces::get_piece_char(&piece.0),
                    None => empty,
                };
                let style = if (role % 2 == 0 && rank % 2 == 0) || (role % 2 != 0 && rank % 2 != 0)
                {
                    Style::new().on_white().black()
                } else {
                    Style::new().on_black().white()
                };
                let style = if avialable_blocks.move_blocks.contains(&(role, rank)) {
                    style.on_blue()
                } else if avialable_blocks.attack_blocks.contains(&(role, rank)) {
                    style.on_red()
                } else {
                    style
                };
                let style = if role == self.user_selected.0 && rank == self.user_selected.1 {
                    style.on_magenta()
                } else {
                    style
                };

                write!(&mut result, "{}", style.apply_to(&format!("{}", &text)))?;
            }
            writeln!(&mut result)?;
        }

        println!("{}", &result.trim());

        Ok(())
    }

    pub fn begin(&mut self) -> crossterm::Result<()> {
        println!("");
        self.draw().unwrap();
        println!("");
        self.drawn = true;

        loop {
            match read()? {
                Event::Key(event) => {
                    match event.code {
                        KeyCode::Up => {
                            self.user_selected = (
                                self.user_selected.0,
                                self.user_selected.1.checked_sub(1).unwrap_or(1).max(1),
                            )
                        }
                        KeyCode::Down => {
                            self.user_selected =
                                (self.user_selected.0, (self.user_selected.1 + 1).min(8))
                        }
                        KeyCode::Left => {
                            self.user_selected = (
                                self.user_selected.0.checked_sub(1).unwrap_or(1).max(1),
                                self.user_selected.1,
                            )
                        }
                        KeyCode::Right => {
                            self.user_selected =
                                ((self.user_selected.0 + 1).min(8), self.user_selected.1)
                        }
                        KeyCode::Enter => {
                            let avialable_blocks = self.get_avialable_blocks();
                            if avialable_blocks.move_blocks.contains(&self.user_selected)
                                || avialable_blocks.attack_blocks.contains(&self.user_selected)
                            {
                                self.move_block(self.box_selected.unwrap(), self.user_selected);
                                self.box_selected = Option::None;
                            } else {
                                if self.board.contains_key(&self.user_selected) {
                                    self.box_selected = Option::Some(self.user_selected);
                                }
                            }
                        }
                        _ => (),
                    };
                    self.draw().unwrap();
                    println!("{:?} {:?}", self.user_selected, event);
                }
                _ => (),
            }
        }
    }
    fn move_block(&mut self, pos: (u8, u8), new_pos: (u8, u8)) {
        let piece = self.board.get(&pos).unwrap().to_owned();
        self.board.remove(&pos);
        self.board.insert(
            new_pos,
            (
                if let pieces::Piece::Pawn(_) = piece.0 {
                    pieces::Piece::Pawn(true)
                } else {
                    piece.0
                },
                piece.1,
            ),
        );
    }
}
