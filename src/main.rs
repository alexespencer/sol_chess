use game::texture::PieceTexture;
use macroquad::prelude::*;
use sol_chess::board::{square::Square, Board};

mod game;

#[macroquad::main("Solitaire Chess")]
async fn main() {
    let background_color = Color::from_rgba(196, 195, 208, 255);
    let game = init().await;
    loop {
        clear_background(background_color);
        game.draw();
        next_frame().await
    }
}

async fn init() -> Game {
    set_pc_assets_folder("./assets");
    let texture_res = load_texture("pieces.png").await.unwrap();
    texture_res.set_filter(FilterMode::Nearest);
    build_textures_atlas();
    let mut board = Board::new();
    board.set(Square::parse("Pa4"));
    board.set(Square::parse("Pa3"));
    board.set(Square::parse("Na2"));
    board.set(Square::parse("Na1"));
    board.set(Square::parse("Bb4"));
    board.set(Square::parse("Bb3"));
    board.set(Square::parse("Rb2"));
    board.set(Square::parse("Rb1"));
    board.set(Square::parse("Kc4"));
    board.set(Square::parse("Kc3"));
    board.set(Square::parse("Qc2"));
    board.set(Square::parse("Qc1"));

    let square_width = 128.0;
    let num_squares = 4;
    let x = (screen_width() - (square_width * num_squares as f32)) / 2.0;
    let y = (screen_height() - (square_width * num_squares as f32)) / 2.0;
    let game = Game::new(board, x, y, square_width, num_squares, texture_res);

    game
}

struct Game {
    board: Board,
    squares: Vec<GameSquare>,
    texture_res: Texture2D,
    num_squares: usize,
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

                rects.push(GameSquare { rect, color, i, j });
            }
        }

        Self {
            board,
            squares: rects,
            num_squares,
            texture_res,
        }
    }

    fn get(&mut self, i: usize, j: usize) -> &mut GameSquare {
        &mut self.squares[i * self.num_squares + j]
    }

    fn draw(&self) {
        let sprite_size = 100.0;
        self.squares.iter().for_each(|square| {
            draw_rectangle(
                square.rect.x,
                square.rect.y,
                square.rect.w,
                square.rect.h,
                square.color,
            );

            if let Some(p) = &self.board.cells[square.i][square.j] {
                let offset = (square.rect.w - sprite_size) / 2.0;
                let dtp = PieceTexture::for_piece(*p, sprite_size);
                draw_texture_ex(
                    &self.texture_res,
                    square.rect.x + offset,
                    square.rect.y + offset,
                    WHITE,
                    dtp,
                );
            }
        });
    }
}

struct GameSquare {
    rect: Rect,
    color: Color,
    i: usize,
    j: usize,
}
