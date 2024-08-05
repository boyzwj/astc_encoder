use image::imageops;
use image::DynamicImage::{self, ImageRgba8};
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

    return astcenc::Context::new(config).unwrap();
}

#[rustler::nif]
fn thumbnail<'a>(
    body: Binary<'a>,
    width: u32,
    height: u32,
    block_size: u32,
    speed: u32,
) -> Result<Vec<u8>, Error> {
    // let y_flip = true;
    let swz: astcenc::Swizzle = astcenc::Swizzle::rgba();
    let mut ctx = init_context(block_size, speed);

    let image: DynamicImage = match image::load_from_memory(body.as_slice()) {
        Ok(img) => img,
        Err(_) => return Err(Error::Term(Box::new("Failed to load image"))),
    };

    let (width, height) = calc_dimension(&image, width, height);
    let thumbnail: DynamicImage = ImageRgba8(imageops::thumbnail(&image, width, height));
    let dyimage = thumbnail.to_rgba8();

    let mut img = astcenc::Image::<Vec<Vec<u8>>>::default();
    let width = dyimage.width();
    let height = dyimage.height();
    img.extents.x = width;
    img.extents.y = height;
    img.extents.z = 1;
    for _ in 0..img.extents.z {
        let mut channel_data: Vec<u8> = Vec::new();
        for y in 0..img.extents.y {
            for x in 0..img.extents.x {
                let pixel = dyimage.get_pixel(x, y);
                channel_data.push(pixel[0]);
                channel_data.push(pixel[1]);
                channel_data.push(pixel[2]);
                channel_data.push(pixel[3]);
            }
        }
        img.data.push(channel_data);
    }

    match ctx.compress(&img, swz) {
        Ok(data) => Ok(data),
        Err(_) => Err(Error::Term(Box::new("Failed to compress image"))),
    }
}

fn calc_dimension(image: &DynamicImage, width: u32, height: u32) -> (u32, u32) {
    if image.width() >= image.height() {
        // landscape
        let ratio = image.height() as f32 / image.width() as f32;
        let height = (ratio * width as f32).round() as u32;

        (width, height)
    } else {
        // portrait
        let ratio = image.width() as f32 / image.height() as f32;
        let width = (ratio * height as f32).round() as u32;

        (width, height)
    }
}
rustler::init!("Elixir.AstcEncoder.Native");
