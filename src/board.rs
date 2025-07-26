pub mod cmove;
mod constants;
pub mod errors;
pub mod piece;
pub mod square;

use core::fmt;
use eyre::{Context, Result, bail};
use std::{
    collections::HashMap,
    fmt::{Display, Formatter},
};

use cmove::CMove;
use constants::BOARD_SIZE;
use errors::SError;
use piece::Piece;
use square::OccupiedSquare;

use crate::board::square::Location;

#[derive(Clone)]
pub struct Board {
    pub occupied_squares: HashMap<Location, Piece>,
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
                    Location::try_new(i, j)
                        .context("create Location")
                        .map_err(|_| SError::InvalidNotation)?,
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
                let c = chars.next().ok_or(SError::InvalidNotation)?;
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
                    Location::try_new(f, r)
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

    pub fn make_move(&mut self, mv: CMove) -> Result<CMove> {
        let legal_moves = self.legal_moves();
        if !legal_moves.contains(&mv) {
            eprintln!("Invalid move - {}", mv.notation());
            eprintln!("Legal moves - ");
            for m in &legal_moves {
                eprintln!("{}", m.notation());
            }
            bail!("invalid move");
        }

        self.set(*mv.to().location(), Some(mv.from().piece()));
        self.set(*mv.from().location(), None);
        self.board_state_changed();
        Ok(mv)
    }

    pub fn empty_locations(&self) -> Vec<Location> {
        let mut empty_squares = Vec::new();
        for file in 0..BOARD_SIZE {
            for rank in 0..BOARD_SIZE {
                let location = Location::try_new(file, rank).expect("valid file/rank");
                if self.occupied_squares.get(&location).is_none() {
                    empty_squares.push(location);
                }
            }
        }
        empty_squares
    }

    pub fn pretty_print(&self) {
        println!("{}", self.print(true));
        println!("{:^40}\n", format!("id: {}", self.id()));
    }

    /// Convert the board state into a u128. This is a reversible operation
    pub fn id(&self) -> u128 {
        let mut res: u128 = 0;

        for file in 0..BOARD_SIZE {
            for rank in 0..BOARD_SIZE {
                res = res << 3;
                let byte = Board::get_piece_encoding(
                    self.occupied_squares
                        .get(&Location::try_new(file, rank).expect("valid file/rank"))
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
                    .get(&Location::try_new(file, rank).expect("valid file/rank"));
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

    /// Returns if a move is legal for this board state
    fn is_legal_move(&self, cmove: &CMove) -> bool {
        match cmove.from().piece() {
            Piece::King => self.is_king_legal(&cmove),
            Piece::Queen => self.is_queen_legal(&cmove),
            Piece::Bishop => self.is_bishop_legal(&cmove),
            Piece::Knight => self.is_knight_legal(&cmove),
            Piece::Rook => self.is_rook_legal(&cmove),
            Piece::Pawn => self.is_pawn_legal(&cmove),
        }
    }

    fn is_king_legal(&self, cmove: &CMove) -> bool {
        cmove.dx().abs() <= 1 && cmove.dy().abs() <= 1
    }

    fn is_queen_legal(&self, cmove: &CMove) -> bool {
        self.is_path_free(cmove)
    }

    fn is_bishop_legal(&self, cmove: &CMove) -> bool {
        cmove.dx().abs() == cmove.dy().abs() && self.is_path_free(cmove)
    }

    fn is_knight_legal(&self, cmove: &CMove) -> bool {
        (cmove.dx().abs() == 2 && cmove.dy().abs() == 1)
            || (cmove.dx().abs() == 1 && cmove.dy().abs() == 2)
    }

    fn is_rook_legal(&self, cmove: &CMove) -> bool {
        if cmove.dx() != 0 && cmove.dy() != 0 {
            return false;
        }

        self.is_path_free(cmove)
    }

    // Pawn move is legal only if it is taking a piece
    fn is_pawn_legal(&self, cmove: &CMove) -> bool {
        cmove.dx().abs() == 1 && cmove.dy() == -1
    }

    fn is_path_free(&self, cmove: &CMove) -> bool {
        // There is no straight line or diagonal to get through
        if cmove.dx().abs() != cmove.dy().abs() && cmove.dx() != 0 && cmove.dy() != 0 {
            return false;
        }

        let x_inc = cmove.dx().signum() as i16;
        let y_inc = cmove.dy().signum() as i16;

        let mut x: i16 = cmove.from().location().file() as i16; // Safe to cast u8 to i16
        let mut y: i16 = cmove.from().location().rank() as i16; // Safe to cast u8 to i16

        loop {
            x = x + x_inc;
            y = y + y_inc;

            let file = x;
            let rank = y;
            if rank == cmove.to().location().rank() as i16
                && file == cmove.to().location().file() as i16
            {
                return true;
            }

            if self
                .occupied_squares
                .get(&Location::try_new(file as u8, rank as u8).expect("valid file/rank"))
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
        } else if self.legal_moves().is_empty() {
            BoardState::Lost
        } else {
            BoardState::InProgress
        };
    }

    /// This is just a cartesian product of {occupied_squares} x {occupied_squares}
    /// however the moves are validated against the current board state
    pub fn legal_moves(&self) -> Vec<CMove> {
        self.all_occupied_squares()
            .into_iter()
            .map(|start| {
                self.all_occupied_squares()
                    .into_iter()
                    .filter_map(move |end| CMove::try_new(start, end).ok())
            })
            .flatten()
            .filter(|m| self.is_legal_move(m))
            .collect::<Vec<CMove>>()
    }

    fn all_occupied_squares(&self) -> impl IntoIterator<Item = OccupiedSquare> {
        self.occupied_squares
            .iter()
            .map(|(location, piece)| OccupiedSquare::new(location.clone(), piece.clone()))
    }

    fn board_state_changed(&mut self) {
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
pub fn set_board_square(board: &mut Board, square: OccupiedSquare) -> Option<Piece> {
    board.set(*square.location(), Some(square.piece()))
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! sq {
        ($sq:literal) => {
            OccupiedSquare::parse($sq).unwrap()
        };
    }

    macro_rules! mv {
        ($from:literal, $to:literal) => {{ CMove::try_new(sq!($from), sq!($to)).unwrap() }};
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
            let mut legal_moves = $board.legal_moves().iter().map(|m| m.clone()).collect::<Vec<CMove>>();

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
        assert_eq!(0, board.legal_moves().len());
        assert!(board.make_move(mv!("Rb2", "Nd1")).is_err());

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
        assert!(board.make_move(mv!("Rb2", "Nd1")).is_err());

        // Modify the board to test the next set of valid moves
        board.set(Location::try_parse("b2").unwrap(), None);
        board.set(Location::try_parse("c4").unwrap(), None);
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

        assert!(board.make_move(mv!("Na1", "Rc2")).is_ok());
        assert_eq!(3, board.pieces_remaining());
        assert_eq!(BoardState::InProgress, board.game_state);

        assert!(board.make_move(mv!("Pb3", "Ka4")).is_ok());
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

        board.make_move(mv!("Qa3", "Pa4")).unwrap();
        board.make_move(mv!("Nc2", "Pa1")).unwrap();
        assert_eq!(2, board.pieces_remaining());
        assert_eq!(BoardState::InProgress, board.game_state);

        // Q . . .
        // . . . .
        // . . . .
        // N . . .
        board.make_move(mv!("Qa4", "Na1")).unwrap();
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
