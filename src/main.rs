use clap::Parser;
use image::{imageops, GenericImageView, ImageBuffer, Rgba, RgbaImage};
use imageproc::filter::filter3x3;
use resvg::tiny_skia::{Pixmap, Transform};
use std::{path::PathBuf, sync::Arc};

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

#[derive(Parser)]
struct Cli {
    input: PathBuf,                     // artwork
    logo: PathBuf,                      // SVG logo
    #[arg(default_value = "2560x1600")]
    resolution: String,                 // WxH
    #[arg(short, long, default_value = "framed.png")]
    output: PathBuf,                    // PNG out
    #[arg(short, long, default_value_t = 0.08)]
    mat_ratio: f32,                     // mat size
    #[arg(long)]
    with_logo: bool,                    // stamp logo
}

fn main() -> Result<()> {
    let cli        = Cli::parse();
    let (cw, ch)   = parse_res(&cli.resolution)?;
    let mat        = (cw.min(ch) as f32 * cli.mat_ratio).round() as u32;
    let mut canvas = ImageBuffer::from_pixel(cw, ch, Rgba([255, 255, 255, 255]));

    // artwork to canvas
    let mut art    = image::open(&cli.input)?;
    let (aw, ah)   = art.dimensions();
    let scale      = ((cw - 2 * mat) as f32 / aw as f32)
                         .min((ch - 2 * mat) as f32 / ah as f32);
    let (nw, nh)   = ((aw as f32 * scale) as u32, (ah as f32 * scale) as u32);
    art            = art.resize_exact(nw, nh, imageops::FilterType::Lanczos3);
    imageops::overlay(&mut canvas, &art,
                      (cw - nw) as i64 / 2, (ch - nh) as i64 / 2);

    // logo
    if cli.with_logo {
        let logo_px = (ch as f32 * 0.12) as u32;
        let raw     = rasterize_svg(&cli.logo, logo_px)?;
        let logo    = emboss_logo(raw);                     // embossed
        let margin  = (mat as f32 * 0.25).round() as i64;   // subtle inset
        imageops::overlay(&mut canvas, &logo,
                          margin,
                          ch as i64 - margin - logo.height() as i64);
    }

    canvas.save(&cli.output)?;
    Ok(())
}

// WxH → (w,h)
fn parse_res(s: &str) -> Result<(u32, u32)> {
    s.split_once('x')
        .and_then(|(w, h)| Some((w.parse().ok()?, h.parse().ok()?)))
        .ok_or_else(|| "resolution must be WIDTHxHEIGHT".into())
}

// rasterise SVG with ×3 supersampling
fn rasterize_svg(path: &PathBuf, target_h: u32) -> Result<RgbaImage> {
    const SS: u32 = 3;

    // need to load fonts
    let data   = std::fs::read(path)?;
    let mut db = usvg::fontdb::Database::new();
    db.load_system_fonts();
    let mut opt = usvg::Options::default();
    opt.fontdb  = Arc::new(db);
    let tree    = usvg::Tree::from_data(&data, &opt)?;

    let scale   = (target_h * SS) as f32
                    / tree.size().height().max(tree.size().width());
    let (rw, rh) = ((tree.size().width()  * scale).round() as u32,
                    (tree.size().height() * scale).round() as u32);
    let mut pm  = Pixmap::new(rw, rh).ok_or("pixmap alloc failed")?;
    resvg::render(&tree, Transform::from_scale(scale, scale), &mut pm.as_mut());

    // premul → straight RGBA
    let mut img = RgbaImage::new(rw, rh);
    for (dst, src) in img.as_mut().chunks_exact_mut(4).zip(pm.data().chunks_exact(4)) {
        let a = src[3];
        if a == 0 { continue }
        let inv = 255f32 / a as f32;
        dst[0] = (src[0] as f32 * inv) as u8;
        dst[1] = (src[1] as f32 * inv) as u8;
        dst[2] = (src[2] as f32 * inv) as u8;
        dst[3] = if a > 240 { 255 } else { a };
    }

    // antialias
    Ok(imageops::resize(&img, rw / SS, rh / SS, imageops::FilterType::CatmullRom))
}

// simple bevel/emboss
fn emboss_logo(mut img: RgbaImage) -> RgbaImage {
    // grayscale height‑map
    let mut height = ImageBuffer::from_fn(img.width(), img.height(), |x, y| {
        let p = img.get_pixel(x, y);
        let l = (0.299 * p[0] as f32 + 0.587 * p[1] as f32 + 0.114 * p[2] as f32) as u8;
        image::Luma([l])
    });

    // 3 × 3 emboss kernel (↗ light, ↙ shadow)
    const K: [f32; 9] = [-2., -1., 0., -1., 1., 1., 0., 1., 2.];
    height = filter3x3(&height, &K);

    // tone tweak
    for (x, y, p) in img.enumerate_pixels_mut() {
        let v = height.get_pixel(x, y)[0] as i16 - 128;
        if v > 0 {
            for c in 0..3 { p[c] = p[c].saturating_add((v as u8) / 3); }
        } else if v < 0 {
            for c in 0..3 { p[c] = p[c].saturating_sub((-v as u8) / 3); }
        }
    }
    img
}
