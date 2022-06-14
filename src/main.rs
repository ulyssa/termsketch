use clap::{Parser, Subcommand};
use dssim_core::{Dssim, DssimImage};
use font_kit::canvas::{Canvas, Format, RasterizationOptions};
use font_kit::family_name::FamilyName;
use font_kit::font::Font;
use font_kit::hinting::HintingOptions;
use font_kit::properties::Properties;
use font_kit::source::SystemSource;
use image::imageops::FilterType;
use image::io::Reader as ImageReader;
use image::{DynamicImage, GenericImageView, GrayImage, Luma};
use imgref::ImgVec;
use pathfinder_geometry::transform2d::Transform2F;
use pathfinder_geometry::vector::{Vector2F, Vector2I};
use rayon::prelude::*;
use std::collections::HashSet;

const DEBUG_GLYPH_RASTER: bool = false;

fn even_offsets(step: usize, end: usize) -> Vec<usize> {
    let mut offsets = Vec::new();
    let mut off = 0;

    while off < end.saturating_sub(step) {
        offsets.push(off);
        off += step;
    }

    if off < end {
        offsets.push(end - step);
    }

    return offsets;
}

fn dynimg2imgvec(image: DynamicImage) -> ImgVec<f32> {
    let dims = image.dimensions();
    let width = dims.0 as usize;
    let height = dims.1 as usize;

    let gray = image.into_luma8();

    let pixels = gray
        .pixels()
        .map(|p| {
            match p {
                Luma([n]) => *n as f32 / 255f32,
            }
        })
        .collect();

    return ImgVec::new(pixels, width, height);
}

fn canvas2imgvec(canvas: Canvas) -> ImgVec<f32> {
    let width = canvas.size.x() as usize;
    let height = canvas.size.y() as usize;
    let stride = canvas.stride;

    let pixels: Vec<f32> = match canvas.format {
        Format::Rgba32 => {
            unimplemented!()
        },
        Format::Rgb24 => {
            unimplemented!()
        },
        Format::A8 => canvas.pixels.iter().map(|p| *p as f32 / 255f32).collect(),
    };

    return ImgVec::new_stride(pixels, width, height, stride);
}

fn canvas2gray(canvas: &Canvas) -> GrayImage {
    let width = canvas.size.x() as u32;
    let height = canvas.size.y() as u32;

    let mut gray = GrayImage::new(width, height);

    for x in 0..width {
        for y in 0..height {
            let pixel: Luma<u8> = match canvas.format {
                Format::Rgba32 => {
                    unimplemented!()
                },
                Format::Rgb24 => {
                    unimplemented!()
                },
                Format::A8 => {
                    let p = canvas.pixels[y as usize * width as usize + x as usize];
                    Luma([p])
                },
            };

            gray.put_pixel(x, y, pixel);
        }
    }

    return gray;
}

struct CharImageCreator {
    font_mono: Font,
    font_serif: Font,
    font_sans_serif: Font,
    dimensions: Vector2I,
}

impl Default for CharImageCreator {
    fn default() -> CharImageCreator {
        CharImageCreator::new((10, 20))
    }
}

impl CharImageCreator {
    fn new(dims: (usize, usize)) -> CharImageCreator {
        //let font = SystemSource::new().select_best_match(&[FamilyName::Title("Noto Color Emoji".to_string())],
        let font_mono = SystemSource::new()
            .select_best_match(&[FamilyName::Monospace], &Properties::new())
            .expect("Could not find Sans Serif font")
            .load()
            .unwrap();
        let font_serif = SystemSource::new()
            .select_best_match(&[FamilyName::Serif], &Properties::new())
            .expect("Could not find Sans Serif font")
            .load()
            .unwrap();
        let font_sans_serif = SystemSource::new()
            .select_best_match(&[FamilyName::SansSerif], &Properties::new())
            .expect("Could not find Sans Serif font")
            .load()
            .unwrap();

        CharImageCreator {
            font_mono,
            font_serif,
            font_sans_serif,
            dimensions: Vector2I::new(dims.0 as i32, dims.1 as i32),
        }
    }

    fn make_image_font(&self, c: char, font: &Font) -> ImgVec<f32> {
        let glyph_id = font.glyph_for_char(c).unwrap();
        let mut canvas = Canvas::new(self.dimensions, Format::A8);

        let size = 30.0;
        let hint = HintingOptions::Full(size);

        font.rasterize_glyph(
            &mut canvas,
            glyph_id,
            size,
            Transform2F::from_translation(Vector2F::new(0.0, self.dimensions.y() as f32 * 0.75)),
            hint,
            RasterizationOptions::GrayscaleAa,
        )
        .unwrap();

        if DEBUG_GLYPH_RASTER {
            let gray = canvas2gray(&canvas);
            gray.save(format!("char-{}-{}.jpg", font.full_name(), glyph_id)).unwrap();
        }

        return canvas2imgvec(canvas);
    }

    fn make_image(&self, c: char) -> ImgVec<f32> {
        let fonts = vec![&self.font_mono, &self.font_sans_serif, &self.font_serif];
        let font = fonts.into_iter().find(|font| font.glyph_for_char(c).is_some());
        let font = font.unwrap();

        return self.make_image_font(c, font);
    }

    fn make_dssim_image(&self, c: char, dssim: &Dssim) -> Option<(char, DssimImage<f32>)> {
        let image = self.make_image(c);

        return dssim.create_image(&image).map(|i| (c, i));
    }
}

struct CharSearcherGray {
    charset: Vec<char>,
    columns: usize,
}

impl CharSearcherGray {
    fn new(columns: usize, charset: Vec<char>) -> Self {
        CharSearcherGray { charset, columns }
    }

    fn scale(&self, image: DynamicImage) -> ImgVec<f32> {
        let dims = image.dimensions();
        let scale = (dims.0 as f64) / self.columns as f64;
        let h = (dims.1 as f64 / (scale * 2.0)) as usize;
        let w = (dims.0 as f64 / scale) as usize;
        let image = image.resize_exact(w as u32, h as u32, FilterType::Triangle);
        let image = dynimg2imgvec(image);

        return image;
    }

    fn convert(&self, image: DynamicImage) -> String {
        let image = self.scale(image);
        let (buf, w, h) = image.into_contiguous_buf();

        (0..h)
            .into_par_iter()
            .map(|y| -> String {
                (0..w)
                    .into_par_iter()
                    .map(|x| {
                        let px = buf[y * w + x];

                        return self.find_similar_char(px);
                    })
                    .collect()
            })
            .collect::<Vec<String>>()
            .join("\n")
    }

    fn find_similar_char(&self, pixel: f32) -> char {
        let len = self.charset.len();
        let idx = (pixel.min(1.0) * len as f32) as usize;

        if idx < len {
            return self.charset[idx];
        } else {
            return *self.charset.last().unwrap_or(&' ');
        }
    }
}

struct CharSearcherDSSIM {
    charset: Vec<(char, DssimImage<f32>)>,
    columns: usize,
    cell: (usize, usize), // width x height

    space: (char, DssimImage<f32>),
    space_bias: f64,

    dssim: Dssim,
}

impl CharSearcherDSSIM {
    fn new(columns: usize, chars: Vec<char>, cell: (usize, usize), space_bias: f64) -> Self {
        let creator = CharImageCreator::new(cell);
        let dssim = Dssim::new();

        let charset: HashSet<char> = chars.iter().cloned().collect();
        let charset: Vec<(char, DssimImage<f32>)> = charset
            .into_iter()
            .filter_map(|c| creator.make_dssim_image(c, &dssim))
            .collect();

        let space = creator.make_dssim_image(' ', &dssim).unwrap();

        return CharSearcherDSSIM {
            columns,
            charset,
            space,
            space_bias: 1.0 / space_bias,
            cell,
            dssim,
        };
    }

    fn scale(&self, image: DynamicImage) -> ImgVec<f32> {
        let dims = image.dimensions();
        let scale = (dims.0 as f64) / (self.columns * self.cell.0) as f64;
        let h = (dims.1 as f64 / scale) as usize;
        let w = (dims.0 as f64 / scale) as usize;
        let image = image.resize(w as u32, h as u32, FilterType::Triangle);
        let image = dynimg2imgvec(image);

        return image;
    }

    fn convert(&self, image: DynamicImage) -> String {
        let image = self.scale(image);
        let h = image.height() as usize;
        let w = image.width() as usize;

        let (sw, sh) = self.cell;

        let line_offsets = even_offsets(sh, h);
        let col_offsets = even_offsets(sw, w);

        line_offsets
            .into_par_iter()
            .map(|y| -> String {
                col_offsets
                    .par_iter()
                    .map(|x| {
                        let sub = image.sub_image(*x, y, sw, sh);
                        let sub = ImgVec::new(sub.pixels().collect(), sw, sh);

                        return self.find_similar_char(&sub);
                    })
                    .collect()
            })
            .collect::<Vec<String>>()
            .join("\n")
    }

    fn find_similar_char(&self, image: &ImgVec<f32>) -> char {
        let image = self.dssim.create_image(image).unwrap();
        let mut bestc = &self.space;
        let mut bests = self.dssim.compare(&image, &bestc.1).0 * self.space_bias;

        for c in self.charset.iter() {
            let s = self.dssim.compare(&image, &c.1).0;
            if s < bests {
                bestc = c;
                bests = s;
            }
        }

        return bestc.0;
    }
}

#[derive(Parser)]
#[clap(version, about, long_about = None)]
#[clap(propagate_version = true)]
struct TermSketch {
    #[clap(subcommand)]
    modes: Modes,
}

#[derive(Subcommand)]
enum Modes {
    /// Convert image by mapping pixels onto a palette of increasingly darker characters.
    #[clap(alias = "greyscale")]
    Grayscale {
        /// How many columns wide the output should be.
        #[clap(short, long, default_value_t = 80, value_parser)]
        width: usize,
        #[clap(long, default_value = "   ...',;:clodxkO0KXNWM", value_parser)]
        charmap: String,
        /// Input files
        #[clap(value_parser)]
        files: Vec<String>,
    },
    /// Convert image by using visually similar characters to draw an outline of boundaries in the
    /// image. This works better for low-detail images where you want to use negative space.
    Outline {
        /// How many columns wide the output should be.
        #[clap(short, long, default_value_t = 80, value_parser)]
        width: usize,
        /// Controls how wide each the compared subregions are.
        #[clap(long, default_value_t = 20, value_parser)]
        cell_width: usize,
        /// The likelihood of inserting a space. (Higher means more likely.)
        #[clap(long, default_value_t = 1.0, value_parser)]
        space_bias: f64,
        /// The character set to use in the converted image.
        #[clap(short, long, default_value = "⡀⠄⠂⠁⡇⠇⠃⡅⡃⡄⡆⡂⡁⠅⠆", value_parser)]
        charset: String,
        /// Input files
        #[clap(value_parser)]
        files: Vec<String>,
    },
}

fn main() -> Result<(), image::ImageError> {
    match TermSketch::parse().modes {
        Modes::Grayscale { width, charmap, files } => {
            let charmap = charmap.chars().collect();
            let converter = CharSearcherGray::new(width, charmap);

            for filename in files {
                let image = ImageReader::open(filename)?.decode()?;
                let txt = converter.convert(image);
                println!("{}", txt);
            }
        },
        Modes::Outline { width, cell_width, space_bias, charset, files } => {
            let cell_height = cell_width * 2;
            let cell = (cell_width, cell_height);
            let charset = charset.chars().collect();

            let converter = CharSearcherDSSIM::new(width, charset, cell, space_bias);

            for filename in files {
                let image = ImageReader::open(filename)?.decode()?;
                let txt = converter.convert(image);
                println!("{}", txt);
            }
        },
    }

    return Ok(());
}
