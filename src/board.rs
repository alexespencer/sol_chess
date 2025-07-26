pub mod cmove;
mod constants;
pub mod errors;
pub mod piece;
pub mod square;

use core::fmt;
use std::{
    collections::{HashMap, HashSet},
    fmt::{Display, Formatter},
};

use cmove::CMove;
use constants::BOARD_SIZE;
use errors::SError;
use eyre::Context;
use piece::Piece;
use square::{Square, SquarePair};

use crate::board::square::Location;

#[derive(Clone)]
pub struct Board {
    pub occupied_squares: HashMap<Location, Piece>,
    pub legal_moves: HashSet<CMove>,
    pub game_state: BoardState,
}

#[derive(PartialEq, Eq, Debug, Clone)]
pub enum BoardState {
    NotStarted,
    InProgress,
    Lost,
    Won,
}

impl Board {
    pub fn pieces_remaining(&self) -> u8 {
        self.occupied_squares.len() as u8
    }

    pub fn new() -> Self {
        Board {
            occupied_squares: HashMap::new(),
            legal_moves: HashSet::new(),
            game_state: BoardState::NotStarted,
        }
    }

    pub fn from_id(board_id: u128) -> Result<Self, SError> {
        let mut board = Board::new();
        let mut working = board_id;
        for i in (0..BOARD_SIZE).rev() {
            for j in (0..BOARD_SIZE).rev() {
                let mask = 0b111;
                let piece = Board::get_piece_from_encoding((working & mask) as u8);
                working = working >> 3;
                let piece = piece?;
                board.set(
                    Location::new(i, j)
                        .context("create Location")
                        .map_err(|_| SError::InvalidBoard)?,
                    piece,
                );
            }
        }
        Ok(board)
    }

    pub fn from_string(board_string: String) -> Result<Self, SError> {
        if board_string.chars().count() != 16 {
            return Err(SError::InvalidBoard);
        }

        let mut board = Board::new();
        let mut chars = board_string.chars();
        for r in 0..BOARD_SIZE {
            for f in 0..BOARD_SIZE {
                let c = chars.next().unwrap();
                let piece = match c {
                    'K' => Piece::King,
                    'Q' => Piece::Queen,
                    'B' => Piece::Bishop,
                    'N' => Piece::Knight,
                    'R' => Piece::Rook,
                    'P' => Piece::Pawn,
                    '.' => continue,
                    _ => return Err(SError::InvalidBoard),
                };

                board.set(
                    Location::new(f, r)
                        .context("create Location")
                        .map_err(|_| SError::InvalidBoard)?,
                    Some(piece),
                );
            }
        }
        Ok(board)
    }

    /// Changes the location to the provided Piece, returns the piece previously at that location
    pub fn set(&mut self, location: Location, piece: Option<Piece>) -> Option<Piece> {
        let existing_piece = self.occupied_squares.get(&location).cloned();
        match piece {
            Some(piece) => self.occupied_squares.insert(location, piece),
            None => self.occupied_squares.remove(&location),
        };
        self.board_state_changed();
        existing_piece
    }

    pub fn make_move(&mut self, mv: CMove) -> Option<CMove> {
        if !self.legal_moves.contains(&mv) {
            println!("Invalid move - {}", mv.notation());
            println!("Legal moves - ");
            for m in &self.legal_moves {
                println!("{}", m.notation());
            }
            return None;
        }

        self.set(*mv.to.location(), mv.from.piece());
        self.set(*mv.from.location(), None);
        self.board_state_changed();
        Some(mv)
    }

    pub fn empty_squares(&self) -> Vec<Square> {
        let mut empty_squares = Vec::new();
        for file in 0..BOARD_SIZE {
            for rank in 0..BOARD_SIZE {
                let location = Location::new(file, rank).expect("valid file/rank");
                if self.occupied_squares.get(&location).is_none() {
                    empty_squares.push(Square::new(location, None));
                }
            }
        }
        empty_squares
    }

    pub fn pretty_print(&self) {
        println!("{}", self.print(true));
        println!("{:^40}\n", format!("id: {}", self.id()));
    }

    /// TODO: replace with Hash using Derivative crate
    pub fn id(&self) -> u128 {
        let mut res: u128 = 0;

        for file in 0..BOARD_SIZE {
            for rank in 0..BOARD_SIZE {
                res = res << 3;
                let byte = Board::get_piece_encoding(
                    self.occupied_squares
                        .get(&Location::new(file, rank).expect("valid file/rank"))
                        .cloned(),
                );
                res = res | byte as u128
            }
        }

        res
    }

    fn print(&self, pretty: bool) -> String {
        let mut board_string = String::new();
        for rank in 0..BOARD_SIZE {
            let mut row = String::new();
            for file in 0..BOARD_SIZE {
                let piece = self
                    .occupied_squares
                    .get(&Location::new(file, rank).expect("valid file/rank"));
                row.push_str(&get_square_for_display(piece, pretty));
            }

            if pretty {
                board_string.push_str(&format!("{:^40}\n", row));
            } else {
                board_string.push_str(&row);
            }

            board_string.push('\n');
        }

        board_string
    }

    fn calc_legal_moves(&mut self) {
        self.legal_moves = self
            .all_possible_move_pairs()
            .into_iter()
            .filter_map(|pair| self.is_legal_move(pair))
            .collect()
    }

    fn is_legal_move(&self, pair: SquarePair) -> Option<CMove> {
        // The below block is just to make the compiler happy. Start will always
        // have a piece
        // TODO: if start will always have a piece, make this go away
        let Some(piece) = pair.start().piece() else {
            return None;
        };

        let legal = match piece {
            Piece::King => self.is_king_legal(&pair),
            Piece::Queen => self.is_queen_legal(&pair),
            Piece::Bishop => self.is_bishop_legal(&pair),
            Piece::Knight => self.is_knight_legal(&pair),
            Piece::Rook => self.is_rook_legal(&pair),
            Piece::Pawn => self.is_pawn_legal(&pair),
        };

        if legal {
            return Some(CMove::new(*pair.start(), *pair.end()));
        }

        None
    }

    fn is_king_legal(&self, pair: &SquarePair) -> bool {
        pair.dx().abs() <= 1 && pair.dy().abs() <= 1
    }

    fn is_queen_legal(&self, pair: &SquarePair) -> bool {
        self.is_path_free(pair)
    }

    fn is_bishop_legal(&self, pair: &SquarePair) -> bool {
        pair.dx().abs() == pair.dy().abs() && self.is_path_free(pair)
    }

    fn is_knight_legal(&self, pair: &SquarePair) -> bool {
        (pair.dx().abs() == 2 && pair.dy().abs() == 1)
            || (pair.dx().abs() == 1 && pair.dy().abs() == 2)
    }

    fn is_rook_legal(&self, pair: &SquarePair) -> bool {
        if pair.dx() != 0 && pair.dy() != 0 {
            return false;
        }

        self.is_path_free(pair)
    }

    // Pawn move is legal only if it is taking a piece
    fn is_pawn_legal(&self, pair: &SquarePair) -> bool {
        pair.dx().abs() == 1 && pair.dy() == -1
    }

    fn is_path_free(&self, pair: &SquarePair) -> bool {
        // There is no straight line or diagonal to get through
        if pair.dx().abs() != pair.dy().abs() && pair.dx() != 0 && pair.dy() != 0 {
            return false;
        }

        let x_inc = pair.dx().signum() as i16;
        let y_inc = pair.dy().signum() as i16;

        let mut x: i16 = pair.start().location().file() as i16; // Safe to cast u8 to i16
        let mut y: i16 = pair.start().location().rank() as i16; // Safe to cast u8 to i16

        loop {
            x = x + x_inc;
            y = y + y_inc;

            let file = x;
            let rank = y;
            if rank == pair.end().location().rank() as i16
                && file == pair.end().location().file() as i16
            {
                return true;
            }

            if self
                .occupied_squares
                .get(&Location::new(file as u8, rank as u8).expect("valid file/rank"))
                .is_some()
            {
                return false;
            }
        }
    }

    fn calc_game_state(&mut self) {
        self.game_state = if self.pieces_remaining() == 0 {
            BoardState::NotStarted
        } else if self.pieces_remaining() == 1 {
            BoardState::Won
        } else if self.legal_moves.is_empty() {
            BoardState::Lost
        } else {
            BoardState::InProgress
        }
    }

    /// This is just a cartesian product of {occupied_squares} x {occupied_squares}
    fn all_possible_move_pairs(&self) -> impl IntoIterator<Item = SquarePair> {
        let ret = self
            .all_occupied_squares()
            .into_iter()
            .map(|start| {
                self.all_occupied_squares()
                    .into_iter()
                    .filter_map(move |end| SquarePair::try_new(start, end).ok())
            })
            .flatten()
            .collect::<Vec<SquarePair>>();

        return ret;
    }

    fn all_occupied_squares(&self) -> impl IntoIterator<Item = Square> {
        self.occupied_squares
            .iter()
            .map(|(location, piece)| Square::new(location.clone(), Some(piece.clone())))
    }

    fn board_state_changed(&mut self) {
        self.calc_legal_moves();
        self.calc_game_state();
    }

    fn get_piece_encoding(piece: Option<Piece>) -> u8 {
        match piece {
            Some(p) => match p {
                Piece::King => 0b001,
                Piece::Queen => 0b010,
                Piece::Rook => 0b011,
                Piece::Bishop => 0b100,
                Piece::Knight => 0b101,
                Piece::Pawn => 0b110,
            },
            None => 0b000,
        }
    }

    fn get_piece_from_encoding(encoding: u8) -> Result<Option<Piece>, SError> {
        match encoding {
            0b001 => Ok(Some(Piece::King)),
            0b010 => Ok(Some(Piece::Queen)),
            0b011 => Ok(Some(Piece::Rook)),
            0b100 => Ok(Some(Piece::Bishop)),
            0b101 => Ok(Some(Piece::Knight)),
            0b110 => Ok(Some(Piece::Pawn)),
            0b000 => Ok(None),
            _ => Err(SError::InvalidBoard),
        }
    }
}

fn get_square_for_display(piece: Option<&Piece>, pretty: bool) -> String {
    let contents = if let Some(piece) = piece {
        if pretty {
            piece.pretty()
        } else {
            piece.to_string()
        }
    } else {
        ".".to_string()
    };

    if pretty {
        format!(" {} ", contents)
    } else {
        contents
    }
}

impl Display for BoardState {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let display = match self {
            BoardState::NotStarted => "Not Started",
            BoardState::InProgress => "In Progress",
            BoardState::Lost => "Lost",
            BoardState::Won => "Won",
        };

        write!(f, "{}", display)
    }
}

#[cfg(test)]
pub fn set_board_square(board: &mut Board, square: Square) -> Option<Piece> {
    board.set(*square.location(), square.piece())
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! sq {
        ($sq:literal) => {
            Square::parse($sq).unwrap()
        };
    }

    macro_rules! mv {
        ($from:literal, $to:literal) => {{ CMove::new(sq!($from), sq!($to)) }};
    }

    macro_rules! validate_board {
        ($board:expr, $row1:literal, $row2:literal, $row3:literal, $row4:literal) => {
            let printed = $board.print(false);
            assert_eq!(
                printed,
                format!("{}\n{}\n{}\n{}\n", $row1, $row2, $row3, $row4)
            );
        };
    }

    macro_rules! validate_legal_moves {
        ($board:expr, $($move:expr,)*) => {
            let mut legal_moves = $board.legal_moves.iter().map(|m| m.clone()).collect::<Vec<CMove>>();

            $(
                assert!(legal_moves.contains(&$move));
                let position = legal_moves.iter().position(|m| m == &$move).unwrap();
                legal_moves.remove(position);
            )*

            if (legal_moves.len() > 0) {
                println!("The following moves were not matched - ");
                for m in &legal_moves {
                    println!("{}", m.notation());
                }

                assert!(false);
            }
        };
    }

    #[test]
    fn test_board_place() {
        let mut board = Board::new();
        assert!(set_board_square(&mut board, sq!("Ka1")).is_none());
        assert!(set_board_square(&mut board, sq!("Qa2")).is_none());
        assert!(set_board_square(&mut board, sq!("Bc3")).is_none());
        assert!(set_board_square(&mut board, sq!("Nc4")).is_none());
        assert!(set_board_square(&mut board, sq!("Rd1")).is_none());
        assert!(set_board_square(&mut board, sq!("Pd4")).is_none());
        assert!(set_board_square(&mut board, sq!("Nb2")).is_none());
        let existing = set_board_square(&mut board, sq!("Pc4"));
        assert!(existing.is_some());
        assert_eq!(Piece::Knight, existing.unwrap());
        validate_board!(board, "..PP", "..B.", "QN..", "K..R");
    }

    #[test]
    fn test_legal_moves() {
        let mut board = Board::new();
        assert_eq!(0, board.pieces_remaining());
        assert_eq!(0, board.legal_moves.len());
        assert!(board.make_move(mv!("Rb2", "Nd1")).is_none());

        set_board_square(&mut board, sq!("Qa4"));
        set_board_square(&mut board, sq!("Ka2"));
        set_board_square(&mut board, sq!("Pa1"));
        set_board_square(&mut board, sq!("Pb3"));
        set_board_square(&mut board, sq!("Rb2"));
        set_board_square(&mut board, sq!("Pc4"));
        set_board_square(&mut board, sq!("Kc3"));
        set_board_square(&mut board, sq!("Bc1"));
        set_board_square(&mut board, sq!("Bd2"));
        set_board_square(&mut board, sq!("Nd1"));

        assert_eq!(10, board.pieces_remaining());
        board.pretty_print();

        // Q . P .
        // . P K .
        // K R . B
        // P . B N
        validate_legal_moves!(
            board,
            mv!("Ka2", "Pa1"),
            mv!("Ka2", "Rb2"),
            mv!("Ka2", "Pb3"),
            mv!("Kc3", "Rb2"),
            mv!("Kc3", "Pb3"),
            mv!("Kc3", "Pc4"),
            mv!("Kc3", "Bd2"),
            mv!("Pa1", "Rb2"),
            mv!("Pb3", "Pc4"),
            mv!("Pb3", "Qa4"),
            mv!("Qa4", "Ka2"),
            mv!("Qa4", "Pb3"),
            mv!("Qa4", "Pc4"),
            mv!("Rb2", "Ka2"),
            mv!("Rb2", "Pb3"),
            mv!("Rb2", "Bd2"),
            mv!("Bc1", "Rb2"),
            mv!("Bc1", "Bd2"),
            mv!("Bd2", "Kc3"),
            mv!("Bd2", "Bc1"),
            mv!("Nd1", "Rb2"),
            mv!("Nd1", "Kc3"),
        );

        assert_eq!(10, board.pieces_remaining());

        // Validate some illegal moves
        assert!(board.make_move(mv!("Ka2", "Pa2")).is_none());
        assert!(board.make_move(mv!("Rb2", "Nd1")).is_none());

        set_board_square(&mut board, sq!(".b2"));
        set_board_square(&mut board, sq!(".c4"));
        set_board_square(&mut board, sq!("Rc1"));

        // Q . . .
        // . P K .
        // K . . B
        // P . R N
        validate_legal_moves!(
            board,
            mv!("Ka2", "Pa1"),
            mv!("Ka2", "Pb3"),
            mv!("Kc3", "Pb3"),
            mv!("Kc3", "Bd2"),
            mv!("Pb3", "Qa4"),
            mv!("Bd2", "Kc3"),
            mv!("Bd2", "Rc1"),
            mv!("Qa4", "Ka2"),
            mv!("Qa4", "Pb3"),
            mv!("Rc1", "Pa1"),
            mv!("Rc1", "Kc3"),
            mv!("Rc1", "Nd1"),
            mv!("Nd1", "Kc3"),
        );

        assert_eq!(8, board.pieces_remaining());
    }

    #[test]
    fn test_smoke_puzzle() {
        let mut board = Board::new();
        assert_eq!(BoardState::NotStarted, board.game_state);
        assert_eq!(0, board.pieces_remaining());

        // K . . .
        // . P . .
        // . . R .
        // N . . .
        set_board_square(&mut board, sq!("Ka4"));
        assert_eq!(BoardState::Won, board.game_state);

        set_board_square(&mut board, sq!("Pb3"));
        set_board_square(&mut board, sq!("Rc2"));
        set_board_square(&mut board, sq!("Na1"));

        assert_eq!(BoardState::InProgress, board.game_state);
        assert_eq!(4, board.pieces_remaining());

        assert!(board.make_move(mv!("Na1", "Rc2")).is_some());
        assert_eq!(3, board.pieces_remaining());
        assert_eq!(BoardState::InProgress, board.game_state);

        assert!(board.make_move(mv!("Pb3", "Ka4")).is_some());
        assert_eq!(2, board.pieces_remaining());
        assert_eq!(BoardState::Lost, board.game_state);

        // P . . .
        // . . . .
        // . . N .
        // . . . .

        set_board_square(&mut board, sq!("Pa1"));
        set_board_square(&mut board, sq!("Qa3"));

        // P . . .
        // Q . . .
        // . . N .
        // P . . .
        assert_eq!(4, board.pieces_remaining());
        assert_eq!(BoardState::InProgress, board.game_state);

        board.make_move(mv!("Qa3", "Pa4"));
        board.make_move(mv!("Nc2", "Pa1"));
        assert_eq!(2, board.pieces_remaining());
        assert_eq!(BoardState::InProgress, board.game_state);

        // Q . . .
        // . . . .
        // . . . .
        // N . . .
        board.make_move(mv!("Qa4", "Na1"));
        assert_eq!(1, board.pieces_remaining());
        assert_eq!(BoardState::Won, board.game_state);
    }

    #[test]
    fn test_encoding() {
        let mut board = Board::new();
        set_board_square(&mut board, sq!("Pa1"));
        set_board_square(&mut board, sq!("Ra2"));
        set_board_square(&mut board, sq!("Qb2"));
        set_board_square(&mut board, sq!("Kd2"));
        set_board_square(&mut board, sq!("Bd4"));
        set_board_square(&mut board, sq!("Nc4"));

        let id = board.id();
        let board2 = Board::from_id(id);
        let board2 = board2.unwrap();

        validate_board!(board2, "..NB", "....", "RQ.K", "P...");
    }
}
