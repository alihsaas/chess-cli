use std::collections::HashMap;

#[derive(Debug, Copy, Clone)]
pub enum Team {
    White,
    Black,
}

#[derive(Debug, Copy, Clone)]
pub enum Piece {
    Pawn(bool),
    King,
    Queen,
    Bishop,
    Knight,
    Tower,
}

type Board = HashMap<(u8, u8), (Piece, Team)>;
type MoveBlocks = Vec<(u8, u8)>;

pub fn get_piece_char(piece: &Piece) -> String {
    match piece {
        Piece::Pawn(_) => String::from("PA"),
        Piece::King => String::from("KI"),
        Piece::Bishop => String::from("BI"),
        Piece::Queen => String::from("QU"),
        Piece::Knight => String::from("KN"),
        Piece::Tower => String::from("TO"),
    }
}

pub struct AvialableBlocks {
    pub attack_blocks: MoveBlocks,
    pub move_blocks: MoveBlocks,
}

impl AvialableBlocks {
    pub fn new() -> Self {
        Self {
            attack_blocks: vec![],
            move_blocks: vec![],
        }
    }
}

fn add_contains(blocks: &mut MoveBlocks, board: &Board, position: (u8, u8)) -> bool {
    if !board.contains_key(&position) {
        blocks.push(position);
        true
    } else {
        false
    }
}

fn add_not_contains(blocks: &mut MoveBlocks, board: &Board, position: (u8, u8)) -> bool {
    if board.contains_key(&position) {
        blocks.push(position);
        true
    } else {
        false
    }
}

fn relative_to_team(initial: u8, add: u8, team: &Team) -> u8 {
    match team {
        Team::Black => initial.checked_add(add).unwrap_or(8),
        Team::White => initial.checked_sub(add).unwrap_or(1),
    }
}

fn get_pawn(board: &Board, position: (u8, u8), has_moved: &bool, team: &Team) -> AvialableBlocks {
    let mut avialable_blocks = AvialableBlocks::new();

    if !has_moved {
        add_contains(
            &mut avialable_blocks.move_blocks,
            &board,
            (position.0, relative_to_team(position.1, 2, team)),
        );
    }

    add_contains(
        &mut avialable_blocks.move_blocks,
        &board,
        (position.0, relative_to_team(position.1, 1, team)),
    );

    add_not_contains(
        &mut avialable_blocks.attack_blocks,
        &board,
        (position.0 + 1, relative_to_team(position.1, 1, team)),
    );
    add_not_contains(
        &mut avialable_blocks.attack_blocks,
        &board,
        (position.0 - 1, relative_to_team(position.1, 1, team)),
    );

    avialable_blocks
}

pub fn get_move_blocks(
    piece: &Piece,
    team: &Team,
    board: &Board,
    position: (u8, u8),
) -> AvialableBlocks {
    match piece {
        Piece::Pawn(has_moved) => get_pawn(&board, position, &has_moved, &team),
        _ => AvialableBlocks::new(),
    }
}
