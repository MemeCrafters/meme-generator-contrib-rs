use skia_safe::Image;

use meme_generator_core::error::Error;
use meme_generator_utils::{
    builder::{InputImage, MemeOptions},
    encoder::{make_gif_or_combined_gif, FrameAlign, GifInfo},
    image::ImageExt,
    shortcut,
    tools::{load_image, local_date, new_surface},
};

use crate::register_meme;

#[derive(MemeOptions)]
struct Fps {
    /// gif帧率
    #[option(long, minimum = 5, maximum = 50, default = 20)]
    fps: Option<i32>,
}

fn do_(images: Vec<InputImage>, _: Vec<String>, options: Fps) -> Result<Vec<u8>, Error> {
    let self_locs = [(116, -8), (109, 3), (130, -10)];
    let user_locs = [(2, 177), (12, 172), (6, 158)];

    let func = |i: usize, images: Vec<Image>| {
        let self_head = images[0].circle().resize_exact((122, 122)).rotate(-15.0);
        let user_head = images[1].circle().resize_exact((112, 112)).rotate(-90.0);
        let image = load_image(format!("do/{i}.png"))?;
        let mut surface = new_surface(image.dimensions());
        let canvas = surface.canvas();
        canvas.draw_image(&image, (0, 0), None);
        canvas.draw_image(&self_head, self_locs[i], None);
        canvas.draw_image(&user_head, user_locs[i], None);
        Ok(surface.image_snapshot())
    };

    make_gif_or_combined_gif(
        images,
        func,
        GifInfo {
            frame_num: 3,
            duration: 1.0 / options.fps.unwrap() as f32,
        },
        FrameAlign::ExtendLoop,
    )
}

register_meme!(
    "do",
    do_,
    min_images = 2,
    max_images = 2,
    keywords = &["撅"],
    shortcuts = &[shortcut!("狠狠地撅", options = &[("fps", 50)])],
    date_created = local_date(2023, 3, 7),
    date_modified = local_date(2023, 3, 7),
);
