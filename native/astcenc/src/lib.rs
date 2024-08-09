use fast_image_resize as fr;
use fast_image_resize::images::Image;
use image::imageops;
use image::DynamicImage::{self, ImageRgba8};
use image::{GenericImageView, ImageReader};
use rustler::{Binary, Error};
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

#[rustler::nif]
fn thumbnail<'a>(
    body: Binary<'a>,
    width: u32,
    height: u32,
    block_size: u32,
    speed: u32,
) -> Result<Vec<u8>, Error> {
    let swz = astcenc::Swizzle::rgba();
    let mut ctx = init_context(block_size, speed);

    // let src_image = ImageReader::open("priv/test.png")
    //     .unwrap()
    //     .decode()
    //     .unwrap();

    let image = image::load_from_memory(body.as_slice())
        .map_err(|_| Error::Term(Box::new("Failed to load image")))?;

    let (width, height) = calc_dimension(&image, width, height);
    let thumbnail = ImageRgba8(imageops::thumbnail(&image, width, height));
    let dyimage = thumbnail.to_rgba8();

    let mut img = astcenc::Image::<Vec<Vec<u8>>>::default();
    img.extents.x = dyimage.width();
    img.extents.y = dyimage.height();
    img.extents.z = 1;

    let channel_data: Vec<u8> = dyimage.pixels().flat_map(|p| p.0.to_vec()).collect();
    img.data.push(channel_data);

    ctx.compress(&img, swz)
        .map_err(|_| Error::Term(Box::new("Failed to compress image")))
}

fn calc_dimension(image: &DynamicImage, width: u32, height: u32) -> (u32, u32) {
    let ratio = if image.width() >= image.height() {
        image.height() as f32 / image.width() as f32
    } else {
        image.width() as f32 / image.height() as f32
    };

    if image.width() >= image.height() {
        (width, (ratio * width as f32).round() as u32)
    } else {
        ((ratio * height as f32).round() as u32, height)
    }
}

rustler::init!("Elixir.AstcEncoder.Native");
