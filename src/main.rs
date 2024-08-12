use macroquad::prelude::*;
use miniquad::window::screen_size;

/// Sprite
#[derive(Debug)]
struct Sprite<'a> {
    texture: &'a Texture2D,
    width: f32,
    height: f32,
    anchor: Vec2,
}

impl<'a> Sprite<'a> {
    pub fn new(texture: &'a Texture2D) -> Self {
        Self {
            texture,
            width: texture.width(),
            height: texture.height(),
            anchor: vec2(0.0, 0.0),
        }
    }

    pub fn with_size(mut self, width: f32, height: f32) -> Self {
        self.width = width;
        self.height = height;
        self
    }

    pub fn with_anchor(mut self, anchor: Vec2) -> Self {
        self.anchor = anchor;
        self
    }

    pub fn draw(&self, x: f32, y: f32) {
        draw_texture_ex(
            self.texture,
            x - self.width * self.anchor.x,
            y - self.height * self.anchor.y,
            WHITE,
            DrawTextureParams {
                dest_size: Some(vec2(self.width, self.height)),
                ..Default::default()
            },
        );
    }
}

#[macroquad::main("BOGDANOV.NET")]
async fn main() -> Result<(), macroquad::Error> {
    const TITLE: &str = "BOGDANOV.NET";
    const TITLE_FONT_SIZE: u16 = 64;
    const BOGDANOV_SPEED: f32 = 4.0;

    let mut points: usize = 0;

    let bogdanov_texture = load_texture("./assets/bogdanov.png").await?;
    let title_font = load_ttf_font("./assets/fonts/Minecraft-font.ttf").await?;

    let mut bogdanov_dir = vec2(1.0, 1.0);
    let mut bogdanov_rect = Rect::new(0.0, 0.0, 200.0, 200.0);
    let bogdanov_sprite = Sprite::new(&bogdanov_texture)
        .with_size(bogdanov_rect.w, bogdanov_rect.h);

    let title_size = measure_text(TITLE, Some(&title_font), TITLE_FONT_SIZE, 1.0);
    let title_text_params = TextParams {
        font: Some(&title_font),
        font_size: TITLE_FONT_SIZE,
        color: BLACK,
        ..Default::default()
    };

    loop {
        let time = get_time() as f32;
        let (screen_w, screen_h) = screen_size();
        let mouse_pos: Vec2 = mouse_position().into();

        // Moving bogdanov
        bogdanov_rect.x += bogdanov_dir.x * BOGDANOV_SPEED;
        bogdanov_rect.y += bogdanov_dir.y * BOGDANOV_SPEED;

        // Bounce bogdanov off the screen edges
        if bogdanov_rect.left() <= 0.0 {
            bogdanov_rect.x = 0.0;
            bogdanov_dir.x *= -1.0;
        }
        if bogdanov_rect.right() > screen_w {
            bogdanov_rect.x = screen_w - bogdanov_rect.w;
            bogdanov_dir.x *= -1.0;
        }
        if bogdanov_rect.top() <= 0.0 {
            bogdanov_rect.y = 0.0;
            bogdanov_dir.y *= -1.0;
        }
        if bogdanov_rect.bottom() > screen_h {
            bogdanov_rect.y = screen_h - bogdanov_rect.h;
            bogdanov_dir.y *= -1.0;
        }

        // Increase points if click on bogdanov
        if bogdanov_rect.contains(mouse_pos) && is_mouse_button_pressed(MouseButton::Left) {
            points += 1;
        }

        // Drawing
        clear_background(WHITE);

        let center_x = screen_w / 2.0;
        let center_y = screen_h / 2.0;

        bogdanov_sprite.draw(
            bogdanov_rect.x,
            bogdanov_rect.y,
        );

        draw_text_ex(
            TITLE,
            center_x - title_size.width / 2.0,
            title_size.offset_y + 20.0,
            title_text_params.clone(),
        );

        let points_str = points.to_string();
        let points_size = measure_text(&points_str, Some(&title_font), TITLE_FONT_SIZE, 1.0);
        draw_text_ex(
            &points_str,
            center_x - points_size.width / 2.0,
            title_size.offset_y + 20.0 + points_size.offset_y,
            title_text_params.clone(),
        );

        next_frame().await;
    }
}
