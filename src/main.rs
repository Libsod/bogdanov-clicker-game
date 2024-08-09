use macroquad::prelude::*;

struct Size {
    font: u16,
    photo: Vec2,
}

struct Bogdanov<'a> {
    photo: Texture2D,
    text: &'a str,
    font: Font,
    size: Size,
}

impl<'a> Bogdanov<'a> {
    async fn default() -> Result<Bogdanov<'a>, macroquad::Error> {
        Ok(Bogdanov {
            photo: load_texture("./assets/bogdanov.png").await?,
            text: "BOGDANOV.NET",
            font: load_ttf_font("./assets/fonts/Minecraft-font.ttf").await?,
            size: Size {
                font: 8 * 8,
                photo: vec2(450., 450.),
            },
        })
    }
}

#[macroquad::main("BOGDANOV.NET")]
async fn main() -> Result<(), macroquad::Error> {
    let bogdanov = Bogdanov::default().await?;

    let text_size = measure_text(bogdanov.text, Some(&bogdanov.font), bogdanov.size.font, 1.);

    loop {
        clear_background(WHITE);

        draw_text_ex(
            bogdanov.text,
            (screen_width() / 2. - text_size.width / 2.).round(),
            (text_size.offset_y + 35.).round(),
            TextParams {
                font: Some(&bogdanov.font),
                font_size: bogdanov.size.font,
                color: BLACK,
                ..Default::default()
            },
        );

        draw_texture_ex(
            &bogdanov.photo,
            (screen_width() / 2. - bogdanov.size.photo.x / 2.).round(),
            (text_size.offset_y + 40.).round(),
            WHITE,
            DrawTextureParams {
                dest_size: Some(bogdanov.size.photo),
                ..Default::default()
            },
        );

        next_frame().await
    }
}
