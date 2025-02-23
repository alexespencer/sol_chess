use core::fmt;
use std::fmt::{Display, Formatter};

use game::texture::PieceTexture;
use macroquad::prelude::*;
use sol_chess::{
    board::{Board, BoardState},
    generator,
};

mod game;

#[macroquad::main("Solitaire Chess")]
async fn main() {
    let background_color = Color::from_rgba(196, 195, 208, 255);
    let mut game = init().await;
    loop {
        clear_background(background_color);
        draw_heading("Solitaire Chess");
        game.update_window_size();
        game.draw();
        game.handle_input();
        next_frame().await
    }
}

fn draw_heading(title: &str) {
    let dims = measure_text(title, None, 60, 1.0);
    let x = screen_width() / 2.0 - dims.width / 2.0;
    let y = 2.0 * dims.height;
    draw_text(title, x, y, 60.0, BLACK);
}

async fn init() -> Game {
    let texture_bytes = include_bytes!("../assets/pieces.png");
    let texture_res = Texture2D::from_file_with_format(&texture_bytes[..], None);
    texture_res.set_filter(FilterMode::Nearest);
    build_textures_atlas();
    let generate = generator::generate(6, 100);
    let board = generate.board().expect("No puzzle was generated");
    let square_width = 128.0;
    let num_squares = 4;
    let x = (screen_width() - (square_width * num_squares as f32)) / 2.0;
    let y = (screen_height() - (square_width * num_squares as f32)) / 2.0;
    let game = Game::new(board, x, y, square_width, num_squares, texture_res);

    game
}

struct Game {
    original_board: Board,
    board: Board,
    squares: Vec<GameSquare>,
    texture_res: Texture2D,
    num_squares: usize,
    state: GameState,
    debug: bool,
    info_square: Rect,
    window_height: f32,
    window_width: f32,
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

#[derive(Copy, Clone)]
enum GameState {
    SelectSource(Option<(usize, usize)>),
    SelectTarget((usize, usize)),
    GameOver((usize, usize)),
}

impl Game {
    fn new(
        board: Board,
        x: f32,
        y: f32,
        square_width: f32,
        num_squares: usize,
        texture_res: Texture2D,
    ) -> Self {
        let dark = Color::from_rgba(83, 104, 120, 255);
        let light = Color::from_rgba(190, 190, 190, 255);
        let mut rects = Vec::new();
        for i in 0..num_squares {
            for j in 0..num_squares {
                let x_eff = x + (i as f32 * square_width);
                let y_eff = y + (j as f32 * square_width);
                let rect = Rect::new(x_eff, y_eff, square_width, square_width);
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

        let info_x = x;
        let info_y = y + (num_squares as f32 * square_width) + square_width / 2.0;
        let info_w = square_width * num_squares as f32;

        Self {
            original_board: board.clone(),
            board,
            squares: rects,
            num_squares,
            texture_res,
            state: GameState::SelectSource(None),
            debug: false,
            info_square: Rect::new(info_x, info_y, info_w, square_width),
            window_height: screen_height(),
            window_width: screen_width(),
        }
    }

    fn update_window_size(&mut self) {
        let new_height = screen_height();
        let new_width = screen_width();

        if new_height == self.window_height && new_width == self.window_width {
            return;
        }

        self.window_height = screen_height();
        self.window_width = screen_width();

        let square_width = 128.0;
        let num_squares = 4;
        let x = (self.window_width - (square_width * num_squares as f32)) / 2.0;
        let y = (self.window_height - (square_width * num_squares as f32)) / 2.0;

        let dark = Color::from_rgba(83, 104, 120, 255);
        let light = Color::from_rgba(190, 190, 190, 255);
        let mut rects = Vec::new();
        for i in 0..num_squares {
            for j in 0..num_squares {
                let x_eff = x + (i as f32 * square_width);
                let y_eff = y + (j as f32 * square_width);
                let rect = Rect::new(x_eff, y_eff, square_width, square_width);
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

        let info_x = x;
        let info_y = y + (num_squares as f32 * square_width) + square_width / 2.0;
        let info_w = square_width * num_squares as f32;

        self.squares = rects;
        self.info_square = Rect::new(info_x, info_y, info_w, square_width);
    }

    fn get(&mut self, i: usize, j: usize) -> &mut GameSquare {
        &mut self.squares[i * self.num_squares + j]
    }

    fn draw(&self) {
        let sprite_size = 100.0;
        let mut selected_square = None;
        self.squares.iter().for_each(|square| {
            let color = if square.is_source {
                Color::from_rgba(152, 152, 152, 255)
            } else if square.is_target {
                Color::from_rgba(152, 129, 123, 255)
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

        draw_text(
            &format!("Press 'R' to reset"),
            self.info_square.x + 20.0,
            self.info_square.y + 20.0,
            20.0,
            BLACK,
        );

        draw_text(
            &format!("Press 'N' for new game (when the current game is won)"),
            self.info_square.x + 20.0,
            self.info_square.y + 40.0,
            20.0,
            BLACK,
        );

        draw_text(
            &format!("Press 'D' to toggle debug mode"),
            self.info_square.x + 20.0,
            self.info_square.y + 60.0,
            20.0,
            GRAY,
        );

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

    fn handle_input(&mut self) {
        if is_key_released(KeyCode::R) {
            self.reset();
            return;
        }

        if is_key_released(KeyCode::N) {
            if let GameState::GameOver(_) = self.state {
                self.next_puzzle();
            }
            return;
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
        self.state = GameState::SelectSource(None);
    }

    fn next_puzzle(&mut self) {
        self.reset();
        let generate = generator::generate(6, 100);
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
