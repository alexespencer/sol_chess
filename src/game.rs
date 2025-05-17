use std::{
    collections::HashMap,
    fmt::{self, Display, Formatter},
};

use button::{Button, ButtonColor};
use macroquad::{math, prelude::*, rand};
use sol_chess::{
    board::{Board, BoardState},
    generator::{self, RandomRange},
};
use texture::PieceTexture;

pub mod button;
pub mod texture;

pub struct MacroquadRandAdapter;
impl RandomRange for MacroquadRandAdapter {
    fn gen_range(&self, min: usize, max: usize) -> usize {
        rand::gen_range(min, max)
    }
}

pub struct Game {
    // The generated puzzle. We keep a copy of this to reset the game.
    original_board: Board,

    // What is shown to the user
    board: Board,

    // Constants througout the game
    texture_res: Texture2D,
    num_squares: usize,
    heading_font_size: f32,
    heading_text: String,

    // Update below on handle input
    state: GameState,
    debug: bool,

    // Update below on window resize
    // Used for drawing the state
    square_width: f32,
    window_height: f32,
    window_width: f32,
    squares: Vec<GameSquare>,
    heading_rect: Rect,
    btns: HashMap<ButtonAction, Button>,
}

struct GameSquare {
    rect: Rect,
    color: Color,
    is_source: bool,
    is_target: bool,
    is_previous_target: bool,
    i: usize,
    j: usize,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub enum ButtonAction {
    Reset,
    Next,
}

#[derive(Copy, Clone)]
enum GameState {
    SelectSource(Option<(usize, usize)>),
    SelectTarget((usize, usize)),
    GameOver((usize, usize)),
}

impl Game {
    pub fn new(board: Board, texture_res: Texture2D) -> Self {
        let num_squares: usize = 4;

        Self {
            original_board: board.clone(),
            board,
            squares: Vec::new(),
            heading_rect: Rect::new(0., 0., 0., 0.),
            heading_text: "Solitaire Chess".to_string(),
            heading_font_size: 0.,
            num_squares,
            texture_res,
            state: GameState::SelectSource(None),
            debug: false,
            btns: HashMap::new(),
            window_height: 0.,
            window_width: 0.,
            square_width: 0.,
        }
    }

    pub fn draw(&self) {
        self.draw_heading();
        self.draw_board();
        self.draw_buttons();
        self.draw_debug();
    }

    pub fn update_window_size(&mut self) {
        let new_height = math::clamp(screen_height(), 100.0, 10000.0);
        let new_width = math::clamp(screen_width(), 100.0, 10000.0);
        if new_height == self.window_height && new_width == self.window_width {
            return;
        }

        self.window_height = new_height;
        self.window_width = new_width;
        self.update_drawables();
    }

    pub fn handle_input(&mut self) {
        let mut btn_clicked = None;
        for btn in &mut self.btns {
            btn.1.handle_input();
            if btn.1.is_clicked() {
                btn_clicked = Some(btn.0.clone());
                break;
            }
        }

        if let Some(action) = btn_clicked {
            match action {
                ButtonAction::Reset => self.reset(),
                ButtonAction::Next => self.next_puzzle(),
            }
        }

        if is_key_released(KeyCode::D) {
            self.debug = !self.debug;
            return;
        }

        if is_key_released(KeyCode::Q) {
            std::process::exit(0);
        }

        if is_mouse_button_released(MouseButton::Left) {
            let current_state = self.state.clone();
            let new_state = match current_state {
                GameState::SelectSource(previous_target) => {
                    self.handle_select_source(mouse_position(), previous_target)
                }
                GameState::SelectTarget(source) => {
                    let next = self.handle_select_target(mouse_position(), source);
                    if let GameState::SelectTarget(_) = next {
                        self.reset_squares();
                        GameState::SelectSource(None)
                    } else {
                        next
                    }
                }

                GameState::GameOver(previous_target) => GameState::GameOver(previous_target),
            };
            self.state = new_state;
            return;
        }

        if is_mouse_button_pressed(MouseButton::Left) {
            let current_state = self.state.clone();
            let new_state = match current_state {
                GameState::SelectSource(previous_target) => {
                    self.handle_select_source(mouse_position(), previous_target)
                }
                GameState::SelectTarget(source) => GameState::SelectTarget(source),
                GameState::GameOver(previous_target) => GameState::GameOver(previous_target),
            };

            self.state = new_state;
        }
    }

    fn draw_heading(&self) {
        let f = self.heading_font_size.floor() as u16;
        let dims = measure_text(self.heading_text.as_str(), None, f, 1.0);
        draw_text(
            self.heading_text.as_str(),
            self.heading_rect.x,
            self.heading_rect.y + dims.offset_y,
            self.heading_font_size,
            BLACK,
        );
    }

    fn draw_board(&self) {
        let sprite_size = 0.8 * self.square_width;
        let mut selected_square = None;
        self.squares.iter().for_each(|square| {
            let color = if square.is_source {
                Color::from_rgba(112, 105, 141, 255)
            } else if square.is_target {
                Color::from_rgba(112, 150, 141, 255)
            } else {
                square.color
            };

            draw_rectangle(
                square.rect.x,
                square.rect.y,
                square.rect.w,
                square.rect.h,
                color,
            );

            if let Some(p) = &self.board.cells[square.i][square.j] {
                let offset = (square.rect.w - sprite_size) / 2.0;
                let dtp = PieceTexture::for_piece(*p, sprite_size);
                if !square.is_source {
                    draw_texture_ex(
                        &self.texture_res,
                        square.rect.x + offset,
                        square.rect.y + offset,
                        WHITE,
                        dtp,
                    );
                } else {
                    selected_square = Some(square);
                }
            }
        });

        if let Some(selected_square) = selected_square {
            if let Some(p) = self.board.cells[selected_square.i][selected_square.j] {
                let dtp = PieceTexture::for_piece(p, sprite_size);
                draw_texture_ex(
                    &self.texture_res,
                    mouse_position().0 - sprite_size / 2.0,
                    mouse_position().1 - sprite_size / 2.0,
                    WHITE,
                    dtp,
                );
            }
        }
    }

    fn draw_buttons(&self) {
        for btn in &self.btns {
            btn.1.draw();
        }
    }

    fn draw_debug(&self) {
        if self.debug {
            let mut debug_lines = vec![];
            let (mx, my) = mouse_position();
            let hover_square = self.squares.iter().find(|s| {
                let c = Circle::new(mx, my, 0.0);
                if c.overlaps_rect(&s.rect) {
                    return true;
                }
                return false;
            });
            debug_lines.push(format!("Game State: {}", self.state));
            debug_lines.push(format!("Board State: {}", self.board.game_state));
            if let Some(hover_square) = hover_square {
                debug_lines.push(format!("Hover: [ {}, {} ]", hover_square.i, hover_square.j));
            }
            self.add_debug_info(debug_lines);

            self.show_fps();
        }
    }

    fn get(&mut self, i: usize, j: usize) -> &mut GameSquare {
        &mut self.squares[i * self.num_squares + j]
    }

    fn update_drawables(&mut self) {
        let min_dimension = f32::min(self.window_height, self.window_width);
        self.square_width = 0.15 * min_dimension;
        let board_width = self.square_width * self.num_squares as f32;
        let board_x = (self.window_width - board_width) / 2.0;
        let board_y = (self.window_height - board_width) / 2.0;

        self.heading_font_size = 0.07 * min_dimension;
        let f = self.heading_font_size.floor() as u16;
        let dims = measure_text(self.heading_text.as_str(), None, f, 1.0);
        self.heading_rect = Rect::new(
            board_x + (board_width - dims.width) / 2.0,
            (board_y - dims.height) / 2.0,
            dims.width,
            dims.height,
        );

        let dark = Color::from_rgba(83, 104, 120, 255);
        let light = Color::from_rgba(190, 190, 190, 255);
        let mut rects = Vec::new();
        for i in 0..self.num_squares {
            for j in 0..self.num_squares {
                let x_eff = board_x + (i as f32 * self.square_width);
                let y_eff = board_y + (j as f32 * self.square_width);
                let rect = Rect::new(x_eff, y_eff, self.square_width, self.square_width);
                let color = match (i + j) % 2 {
                    1 => dark,
                    _ => light,
                };

                rects.push(GameSquare {
                    rect,
                    color,
                    i,
                    j,
                    is_source: false,
                    is_target: false,
                    is_previous_target: false,
                });
            }
        }

        self.squares = rects;

        let btn_h = 0.08 * self.window_height;
        let btn_w = board_width * 0.2;

        let btn_y = board_width + board_y + 0.3 * self.square_width;
        let btn_x_offset = 0.5 * (board_width / 2. - btn_w);
        let reset_btn = Button::new(
            "Reset",
            Rect::new(board_x + btn_x_offset, btn_y, btn_w, btn_h),
            ButtonColor::Yellow,
        );
        let mut next_btn = Button::new(
            "Next",
            Rect::new(
                board_x + (0.5 * board_width) + btn_x_offset,
                btn_y,
                btn_w,
                btn_h,
            ),
            ButtonColor::Green,
        );

        next_btn.is_active = false;

        self.btns = HashMap::new();
        self.btns.insert(ButtonAction::Next, next_btn);
        self.btns.insert(ButtonAction::Reset, reset_btn);
    }

    fn handle_select_source(
        &mut self,
        mouse_position: (f32, f32),
        previous_target: Option<(usize, usize)>,
    ) -> GameState {
        self.reset_squares();
        let (x, y) = mouse_position;
        let mouse = Circle::new(x, y, 0.0);
        let mut selected = None;
        for square in &mut self.squares {
            if mouse.overlaps_rect(&square.rect) {
                if let Some(_) = self.board.cells[square.i][square.j] {
                    selected = Some((square.i, square.j));
                }
            }
        }

        if let Some((i, j)) = selected {
            self.get(i, j).is_source = true;
            let mut target_squares = vec![];
            for m in self.board.legal_moves.iter() {
                if m.from.file == i && m.from.rank == j {
                    target_squares.push((m.to.file, m.to.rank));
                }
            }

            for (i, j) in target_squares {
                self.get(i, j).is_target = true;
            }

            return GameState::SelectTarget(selected.unwrap());
        }

        if let Some((i, j)) = previous_target {
            self.get(i, j).is_previous_target = true;
        }

        return GameState::SelectSource(None);
    }

    fn handle_select_target(
        &mut self,
        mouse_position: (f32, f32),
        source: (usize, usize),
    ) -> GameState {
        let (x, y) = mouse_position;
        let mouse = Circle::new(x, y, 0.0);

        let mut selected = None;
        for square in &mut self.squares {
            if mouse.overlaps_rect(&square.rect) {
                if let Some(_) = self.board.cells[square.i][square.j] {
                    selected = Some((square.i, square.j));
                }
            }
        }

        let (s_x, s_y) = source;
        let Some((x, y)) = selected else {
            self.get(s_x, s_y).is_source = true;
            return GameState::SelectTarget(source);
        };

        if x == s_x && y == s_y {
            self.get(s_x, s_y).is_source = true;
            return GameState::SelectTarget(source);
        }

        let mut is_legal = false;
        if self.get(x, y).is_target {
            is_legal = true;
        }

        if is_legal {
            let m = self.board.legal_moves.iter().find(|m| {
                m.from.file == s_x && m.from.rank == s_y && m.to.file == x && m.to.rank == y
            });

            let m = m.expect("legal move should be found");

            self.board.make_move(m.clone());

            if self.board.game_state == BoardState::Won || self.board.game_state == BoardState::Lost
            {
                self.reset_squares();
                if self.board.game_state == BoardState::Won {
                    let next_btn = self
                        .btns
                        .get_mut(&ButtonAction::Next)
                        .expect("Cannot find next button");
                    next_btn.is_active = true;
                }

                return GameState::GameOver((x, y));
            }

            self.reset_squares();
            self.get(x, y).is_target = true;
            return GameState::SelectSource(Some((x, y)));
        }

        self.reset_squares();
        return GameState::SelectSource(None);
    }

    fn reset(&mut self) {
        self.board = self.original_board.clone();
        self.reset_squares();

        let next_button = self
            .btns
            .get_mut(&ButtonAction::Next)
            .expect("Cannot find next button");
        next_button.is_active = false;

        self.state = GameState::SelectSource(None);
    }

    fn next_puzzle(&mut self) {
        self.reset();
        let generate = generator::generate(6, 100, &MacroquadRandAdapter);
        let board = generate.board().expect("No puzzle was generated");
        self.original_board = board.clone();
        self.board = board;
    }

    fn reset_squares(&mut self) {
        for i in 0..self.num_squares {
            for j in 0..self.num_squares {
                self.get(i, j).is_source = false;
                self.get(i, j).is_target = false;
            }
        }
    }

    fn add_debug_info(&self, lines: Vec<String>) {
        let mut y = 20.0;
        for line in lines {
            draw_text(&line, 10.0, y, 20.0, BLACK);
            y += 25.0;
        }
    }

    fn show_fps(&self) {
        let fps = get_fps();
        draw_text(
            &format!("FPS: {}", fps),
            10.0,
            screen_height() - 20.0,
            20.0,
            BLACK,
        );
    }
}

impl Display for GameState {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            GameState::SelectSource(Some(x)) => write!(f, "Select Source [ {}, {} ]", x.0, x.1),
            GameState::SelectSource(None) => write!(f, "Select Source [ ]"),
            GameState::SelectTarget(x) => write!(f, "Select Target [ {}, {} ]", x.0, x.1),
            GameState::GameOver(x) => write!(f, "Game Over [ {}, {} ]", x.0, x.1),
        }
    }
}
