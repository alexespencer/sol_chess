use macroquad::{audio, prelude::*};
use miniquad::date;
use sol_chess::game::{Game, sound::Sounds};

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
        game.draw();
        next_frame().await
    }
}

macro_rules! load_sound {
    ($file_name:expr) => {
        audio::load_sound_from_bytes(include_bytes!($file_name))
            .await
            .expect("valid sound")
    };
}

async fn init() -> Game {
    let texture_bytes = include_bytes!("../assets/pieces.png");
    let texture_res = Texture2D::from_file_with_format(&texture_bytes[..], None);
    texture_res.set_filter(FilterMode::Nearest);
    build_textures_atlas();
    let click = load_sound!("../assets/click.wav");
    let win = load_sound!("../assets/win.wav");
    let loss = load_sound!("../assets/loss.wav");
    let button = load_sound!("../assets/button.wav");
    let mode = load_sound!("../assets/mode.wav");
    let sounds = Sounds {
        click,
        win,
        loss,
        button,
        mode,
    };
    let game = Game::new(texture_res, sounds);
    game
}
