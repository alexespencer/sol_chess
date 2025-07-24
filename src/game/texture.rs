use macroquad::prelude::*;

use crate::board::piece::Piece;

pub trait Texture {
    fn texture(&self, sprite_size: f32) -> DrawTextureParams;
}

impl Texture for Piece {
    fn texture(&self, sprite_size: f32) -> macroquad::prelude::DrawTextureParams {
        let index = match self {
            Piece::Pawn => 0,
            Piece::Knight => 1,
            Piece::Bishop => 2,
            Piece::Rook => 3,
            Piece::Queen => 4,
            Piece::King => 5,
        };

        DrawTextureParams {
            source: Some(Rect::new(index as f32 * 128.0, 0.0, 128.0, 128.0)),
            dest_size: Some(Vec2::new(sprite_size, sprite_size)),
            ..DrawTextureParams::default()
        }
    }
}
