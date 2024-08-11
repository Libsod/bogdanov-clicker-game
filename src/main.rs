use macroquad::prelude::*;
use miniquad::window::screen_size;

const TITLE: &str = "BOGDANOV.NET";
const TITLE_FONT_SIZE: u16 = 64;

/// Sprite
#[derive(Debug)]
struct Sprite<'a> {
    texture: &'a Texture2D,
    width: f32,
    height: f32,
    anchor: Vec2
}
impl<'a> Sprite<'a> {
    pub fn new(texture: &'a Texture2D) -> Self {
        Self {
            texture,
            width: texture.width(),
            height: texture.height(),
            anchor: vec2(0.0, 0.0)
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
            &self.texture,
            x - self.width * self.anchor.x,
            y - self.height * self.anchor.y,
            WHITE,
            DrawTextureParams {
                dest_size: Some(vec2(self.width, self.height)),
                ..Default::default()
            }
        );
    }
}

#[macroquad::main("BOGDANOV.NET")]
async fn main() -> Result<(), macroquad::Error> {
    let bogdanov_tex = load_texture("./assets/bogdanov.png").await?;
    let title_font = load_ttf_font("./assets/fonts/Minecraft-font.ttf").await?;

    let bogdanov_sprite = Sprite::new(&bogdanov_tex)
        .with_size(400.0, 400.0)
        .with_anchor(vec2(0.5, 0.5));

    let title_size = measure_text(TITLE, Some(&title_font), TITLE_FONT_SIZE, 1.0);

    loop {
        clear_background(WHITE);

        let (sw, sh) = screen_size();
        let center_x = sw / 2.0;
        let center_y = sh / 2.0;

        bogdanov_sprite.draw(
            center_x,
            center_y,
        );

        draw_text_ex(
            TITLE,
            center_x - title_size.width / 2.0,
            title_size.offset_y + 20.0,
            TextParams {
                font: Some(&title_font),
                font_size: TITLE_FONT_SIZE,
                color: BLACK,
                ..Default::default()
            },
        );

        next_frame().await;
    }
}
