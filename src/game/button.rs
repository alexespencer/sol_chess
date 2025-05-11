use macroquad::prelude::*;

pub struct Button {
    pub text: String,
    pub is_active: bool,
    pub action: ButtonAction,
    is_down: bool,
    is_clicked: bool,
    rect: Rect,
    shadow_width: f32,
}

#[derive(Debug, Clone, Copy)]
pub enum ButtonAction {
    Reset,
    Next,
}

impl Button {
    pub fn new(text: &str, x: f32, y: f32, width: f32, height: f32, action: ButtonAction) -> Self {
        let rect = Rect::new(x, y, width, height);
        Self {
            text: text.to_string(),
            is_down: false,
            is_clicked: false,
            is_active: true,
            rect,
            action,
            shadow_width: 5.0,
        }
    }

    pub fn is_clicked(&mut self) -> bool {
        if self.is_clicked {
            self.is_clicked = false;
            return true;
        }

        false
    }

    pub fn draw(&self) {
        self.draw_button();
        self.draw_label();
    }

    fn draw_button(&self) {
        let bg_color = Color::from_rgba(190, 190, 190, 255);
        let button_draw_offset = self.get_button_draw_offset();
        draw_rectangle(
            self.rect.x + button_draw_offset,
            self.rect.y + button_draw_offset,
            self.rect.w,
            self.rect.h,
            bg_color,
        );

        self.draw_shadow();
    }

    fn draw_shadow(&self) {
        if !self.is_active {
            return;
        }

        if self.is_down {
            return;
        }

        let color = Color::from_rgba(0, 0, 0, 100);
        draw_rectangle(
            self.rect.x + self.rect.w,
            self.rect.y + self.shadow_width,
            self.shadow_width,
            self.rect.h,
            color,
        );

        draw_rectangle(
            self.rect.x + self.shadow_width,
            self.rect.y + self.rect.h,
            self.rect.w - self.shadow_width,
            self.shadow_width,
            color,
        );

        draw_triangle(
            vec2(self.rect.x + self.rect.w, self.rect.y),
            vec2(
                self.rect.x + self.rect.w + self.shadow_width,
                self.rect.y + self.shadow_width,
            ),
            vec2(self.rect.x + self.rect.w, self.rect.y + self.shadow_width),
            color,
        );

        draw_triangle(
            vec2(self.rect.x, self.rect.y + self.rect.h),
            vec2(
                self.rect.x + self.shadow_width,
                self.rect.y + self.rect.h + self.shadow_width,
            ),
            vec2(self.rect.x + self.shadow_width, self.rect.y + self.rect.h),
            color,
        );
    }

    fn draw_label(&self) {
        let font_color = match self.is_active { 
            true => Color::from_rgba(0, 0, 0, 255),
            false => Color::from_rgba(100, 100, 100, 255)
        };

        let font_size = (0.5 * self.rect.h) as u16;
        let dims = measure_text(&self.text, None, font_size, 1.0);
        let button_draw_offset = self.get_button_draw_offset();

        draw_text(
            &self.text,
            self.rect.x + (self.rect.w - dims.width) * 0.5 + button_draw_offset,
            self.rect.y + (self.rect.h - dims.height) * 0.5 + dims.offset_y + button_draw_offset,
            font_size as f32,
            font_color,
        );
    }

    fn get_button_draw_offset(&self) -> f32 {
        let button_pressed_correction = match self.is_down {
            true => self.shadow_width,
            false => match self.is_active {
                true => 0.0,
                false => self.shadow_width,
            },
        };
        button_pressed_correction
    }

    pub fn handle_input(&mut self) {
        if !self.is_active {
            self.is_down = false;
            return;
        }

        let (mx, my) = mouse_position();
        let c = Circle::new(mx, my, 0.0);

        if is_mouse_button_pressed(MouseButton::Left) {
            if c.overlaps_rect(&self.rect) {
                self.is_down = true;
                return;
            }
        }

        if is_mouse_button_released(MouseButton::Left) {
            if c.overlaps_rect(&self.rect) {
                self.is_clicked = true;
                self.is_down = false;
                return;
            }

            self.is_down = false;
        }
    }
}
