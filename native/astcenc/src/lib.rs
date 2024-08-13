use fast_image_resize::images::Image;
use fast_image_resize::{self as fr, ResizeAlg, ResizeOptions};
use fast_image_resize::{IntoImageView, Resizer};
use image::imageops;
use image::DynamicImage::{self, ImageRgba8};
use image::ImageFormat;
use image::{GenericImageView, ImageReader};
use rustler::{Binary, Encoder, Env, Error, NifResult, OwnedBinary, Term};
mod astcenc;

mod atoms {
    rustler::atoms! {
        ok
    }
}

fn init_context(block_size: u32, speed: u32) -> astcenc::Context {
    let extents = match block_size {
        4 => astcenc::Extents { x: 4, y: 4, z: 1 },
        5 => astcenc::Extents { x: 5, y: 5, z: 1 },
        6 => astcenc::Extents { x: 6, y: 6, z: 1 },
        8 => astcenc::Extents { x: 8, y: 8, z: 1 },
        _ => astcenc::Extents { x: 6, y: 6, z: 1 },
    };

    let preset = match speed {
        1 => astcenc::PRESET_FASTEST,
        2 => astcenc::PRESET_FAST,
        3 => astcenc::PRESET_MEDIUM,
        4 => astcenc::PRESET_THOROUGH,
        5 => astcenc::PRESET_VERY_THOROUGH,
        6 => astcenc::PRESET_EXHAUSTIVE,
        _ => astcenc::PRESET_MEDIUM,
    };

    let config = astcenc::ConfigBuilder::new()
        .with_preset(preset)
        .with_block_size(extents)
        .build()
        .unwrap();

    astcenc::Context::new(config).unwrap()
}

#[rustler::nif(schedule = "DirtyCpu")]
fn thumbnail<'a>(
    env: Env<'a>,
    body: Binary<'a>,
    width: u32,
    height: u32,
    block_size: u32,
    speed: u32,
) -> NifResult<Term<'a>> {
    let swz = astcenc::Swizzle::rgba();
    let mut ctx = init_context(block_size, speed);
    let channel_data = resize_img(body.as_slice(), width, height)
        .map_err(|_| Error::Term(Box::new("Failed to resize image")))?;

    let img = astcenc::Image {
        extents: astcenc::Extents {
            x: width,
            y: height,
            z: 1,
        },
        data: vec![channel_data],
    };

    let compressed_data = ctx.compress(&img, swz).unwrap();
    let mut binary: OwnedBinary = OwnedBinary::new(compressed_data.len()).unwrap();

    binary.as_mut_slice().copy_from_slice(&compressed_data);

    Ok(binary.release(env).encode(env))
}

fn resize_img(
    png_buffer: &[u8],
    width: u32,
    height: u32,
) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    let img = image::ImageReader::new(std::io::Cursor::new(png_buffer))
        .with_guessed_format()?
        .decode()?;
    let (original_width, original_height) = img.dimensions();
    let src_image = Image::from_vec_u8(
        original_width,
        original_height,
        img.to_rgba8().into_raw(),
        fr::PixelType::U8x4,
    )
    .unwrap();

    let mut dst_image = Image::new(width, height, src_image.pixel_type());
    let mut resizer = fr::Resizer::new();
    #[cfg(target_arch = "x86_64")]
    unsafe {
        resizer.set_cpu_extensions(fr::CpuExtensions::Avx2);
    }
    // let option =
    //     fr::ResizeOptions::new().resize_alg(fr::ResizeAlg::Convolution(fr::FilterType::Lanczos3));
    let option = fr::ResizeOptions::new().resize_alg(fr::ResizeAlg::Nearest);
    resizer.resize(&src_image, &mut dst_image, &option).unwrap();
    Ok(dst_image.into_vec())
}

rustler::init!("Elixir.AstcEncoder.Native");
