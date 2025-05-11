use macroquad::prelude::*;

pub struct Button {
    pub is_active: bool,
    text: String,
    is_down: bool,
    is_clicked: bool,
    rect: Rect,
    shadow_width: f32,
    color: ButtonColor,
}

pub enum ButtonColor {
    Grey,
    Green,
    Yellow,
}

impl ButtonColor {
    fn to_bg_color(&self) -> Color {
        match self {
            ButtonColor::Grey => Color::from_rgba(140, 140, 140, 200),
            ButtonColor::Green => Color::from_rgba(112, 140, 141, 200),
            ButtonColor::Yellow => Color::from_rgba(123, 70, 85, 200),
        }
    }

    fn to_shadow_color(&self) -> Color {
        let bg_color = self.to_bg_color();
        Color::from_rgba(
            (bg_color.r * 255.) as u8,
            (bg_color.g * 255.) as u8,
            (bg_color.b * 255.) as u8,
            100,
        )
    }
}

impl Button {
    pub fn new(text: &str, rect: Rect, color: ButtonColor) -> Self {
        Self {
            text: text.to_string(),
            is_down: false,
            is_clicked: false,
            is_active: true,
            rect,
            shadow_width: 5.0,
            color,
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
        let bg_color = match self.is_active {
            true => self.color.to_bg_color(),
            false => ButtonColor::Grey.to_bg_color(),
        };

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

        let color = self.color.to_shadow_color();
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
            false => Color::from_rgba(100, 100, 100, 255),
        };

        let font_size = (0.3 * self.rect.w) as u16;
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
