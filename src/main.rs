use game::{Game, MacroquadRandAdapter};
use macroquad::prelude::*;
use miniquad::date;
use sol_chess::generator;

mod game;

fn window_conf() -> Conf {
    let window_name = match std::env::var("TESTING") {
        Ok(_) => "DEV TESTING MOVE TO WORKSPACE 10",
        Err(_) => "Solitaire Chess",
    };

    Conf {
        window_title: window_name.to_string(),
        fullscreen: false,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    rand::srand(date::now() as u64);
    let background_color = Color::from_rgba(196, 195, 208, 255);
    let mut game = init().await;
    loop {
        clear_background(background_color);
        game.handle_input();
        game.update_window_size();
        game.draw();
        next_frame().await
    }
}

async fn init() -> Game {
    let texture_bytes = include_bytes!("../assets/pieces.png");
    let texture_res = Texture2D::from_file_with_format(&texture_bytes[..], None);
    texture_res.set_filter(FilterMode::Nearest);
    build_textures_atlas();
    let generate = generator::generate(6, 100, &MacroquadRandAdapter);
    let board = generate.board().expect("No puzzle was generated");
    let game = Game::new(board, texture_res);

    game
}
