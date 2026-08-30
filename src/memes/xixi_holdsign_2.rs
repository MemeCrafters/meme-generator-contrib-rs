use meme_generator_core::error::Error;
use meme_generator_utils::{
    builder::InputImage,
    encoder::GifEncoder,
    image::ImageExt,
    text::Text2Image,
    text_params,
    tools::{color_from_hex_code, load_image, local_date, new_paint},
};
use skia_safe::{textlayout::TextAlign, Canvas};

use crate::{options::NoOptions, register_meme};

const FRAME_NUM: u32 = 30;
const FPS: f32 = 30.0;

const MAX_TEXT_WIDTH: f32 = 130.0;
const MIN_FONT_SIZE: f32 = 18.0;
const MAX_FONT_SIZE: f32 = 40.0;

const DEFAULT_TEXT: &str = "点亮语乂";

/// Per-frame text center (cx, cy) and sign/text rotation angle (deg), measured
/// from the reference images. The sign is held at a near-constant tilt of about
/// +3.94 deg, but its position in the frame shifts horizontally (a small sway
/// in the holder's arm), so the text tracks that.
///
/// The raw text-pixels centroid is biased left by ~3.5 px (and up by ~0.9 px)
/// due to glyph weight and anti-aliased halo. Pre-shift each entry so the
/// rendered text lands at the sign's actual geometric center.
const TEXT_CENTERS: [(f32, f32); FRAME_NUM as usize] = [
    (192.3, 92.4), (190.1, 92.5), (189.7, 92.4), (189.8, 92.4), (190.0, 92.4),
    (190.9, 92.4), (191.4, 92.4), (191.4, 92.4), (190.7, 92.4), (190.6, 92.4),
    (191.1, 92.5), (194.3, 92.4), (195.2, 92.4), (196.1, 92.5), (193.5, 92.4),
    (192.1, 92.5), (191.0, 92.5), (193.0, 92.4), (195.4, 92.4), (196.5, 92.4),
    (195.5, 92.4), (192.6, 92.4), (190.6, 92.4), (191.4, 92.4), (193.1, 92.4),
    (195.0, 92.4), (197.9, 92.4), (196.7, 92.4), (195.3, 92.4), (190.5, 92.4),
];

const TEXT_ANGLES: [f32; FRAME_NUM as usize] = [
    3.96, 3.97, 3.93, 3.94, 3.99,
    3.92, 3.95, 3.95, 3.93, 3.91,
    3.94, 3.94, 3.94, 3.96, 3.93,
    3.95, 3.94, 3.92, 3.95, 3.94,
    3.91, 3.89, 3.92, 3.96, 3.94,
    3.94, 3.93, 3.91, 3.95, 3.91,
];

fn pick_font_size(text: &str, paint: &skia_safe::Paint) -> f32 {
    let mut size = MAX_FONT_SIZE;
    while size >= MIN_FONT_SIZE {
        let img = Text2Image::from_text(
            text,
            size,
            text_params!(
                text_align = TextAlign::Center,
                font_families = &["Kingnammm Maiyuan 2"],
                paint = paint.clone(),
            ),
        );
        if img.longest_line() <= MAX_TEXT_WIDTH {
            return size;
        }
        size -= 1.0;
    }
    MIN_FONT_SIZE
}

fn draw_rotated_text(
    canvas: &Canvas,
    text: &str,
    center: (f32, f32),
    angle: f32,
    font_size: f32,
    paint: skia_safe::Paint,
) {
    let text2image = Text2Image::from_text(
        text,
        font_size,
        text_params!(
            text_align = TextAlign::Center,
            font_families = &["Kingnammm Maiyuan 2"],
            paint = paint,
        ),
    );
    let w = text2image.longest_line();
    let h = text2image.height();
    canvas.save();
    canvas.translate((center.0, center.1));
    canvas.rotate(angle, None);
    text2image.draw_on_canvas(canvas, (-w / 2.0, -h / 2.0));
    canvas.restore();
}

fn xixi_holdsign_2(
    _: Vec<InputImage>,
    texts: Vec<String>,
    _: NoOptions,
) -> Result<Vec<u8>, Error> {
    let text = if texts.is_empty() {
        DEFAULT_TEXT
    } else {
        texts[0].as_str()
    };

    let paint = new_paint(color_from_hex_code("#f8b860"));
    let font_size = pick_font_size(text, &paint);

    let mut encoder = GifEncoder::new();
    let duration = 1.0 / FPS;
    for i in 0..FRAME_NUM {
        let frame = load_image(format!("xixi_holdsign_2/{i}.png"))?;
        let mut surface = frame.to_surface();
        draw_rotated_text(
            surface.canvas(),
            text,
            TEXT_CENTERS[i as usize],
            TEXT_ANGLES[i as usize],
            font_size,
            paint.clone(),
        );
        encoder.add_frame(surface.image_snapshot(), duration)?;
    }
    Ok(encoder.finish()?)
}

register_meme!(
    "xixi_holdsign_2",
    xixi_holdsign_2,
    min_texts = 0,
    max_texts = 1,
    default_texts = &[DEFAULT_TEXT],
    keywords = &["西西举牌2"],
    date_created = local_date(2026, 8, 30),
    date_modified = local_date(2026, 8, 30),
);
