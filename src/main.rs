use dirs;
use ffmpeg_cmdline_utils as ffmpeg;
use std::env;
use std::fmt::Write;
use std::path::{Path, PathBuf};
use std::thread;
use std::time::Duration;
use std::time::Instant;

/* TODO
[1]. Use thread sleep to match framerate [DONE]
*/

fn main() {
    let pixels = vec![" ", " ", " ", ".", ",", "^", ":", "~", "|", "?", "&", "#", "@"];

    let mut path: Option<PathBuf> = None;

    if let Some(p) = env::args().nth(1) {
        if Path::new(&p).exists() {
            path = Some(PathBuf::from(p));
        }
    }

    let mut temp_path;

    if path == None {
        temp_path = dirs::video_dir().expect("Failed to find videos directory");
        temp_path.push("input.mp4");

        path = Some(temp_path);
    }

    let builder = ffmpeg::FfmpegFrameReaderBuilder::new(path.unwrap());
    let frames = builder.spawn().expect("Failed to load video");
    let mut text_frame = String::new();

    let mut before: Instant;
    for f in frames.0.into_iter() {
        before = Instant::now();

        for y in 0..f.height() {
            for x in 0..f.width() {
                let brightness = pixel_brightness(
                    (f.get_pixel(x, y)[0] as f32) / 255f32,
                    (f.get_pixel(x, y)[1] as f32) / 255f32,
                    (f.get_pixel(x, y)[2] as f32) / 255f32,
                );

                let index = (brightness * ((pixels.len() - 1) as f32)) as usize;
                let pixel = pixels[index];

                write!(text_frame, "{}", pixel).unwrap();
                write!(text_frame, "{}", pixel).unwrap();
            }

            write!(text_frame, "\n").unwrap();
        }

        let dur = Duration::from_millis(
            16u64.saturating_sub((before.elapsed().subsec_millis()).try_into().unwrap()),
        );
        thread::sleep(dur);

        print!("{esc}c", esc = 27 as char);
        println!("{}", text_frame);
        text_frame.clear();
    }
}

fn pixel_brightness(r: f32, g: f32, b: f32) -> f32 {
    0.2126 * r + 0.7152 * g + 0.0722 * b
}
