#[cfg(feature = "opencv")]
mod cv_convert;
#[cfg(feature = "opencv")]
use cv_convert::convert;

use anyhow::{anyhow, Result};
use cameleon::payload::{Payload, PayloadType};
use cameleon_device::PixelFormat;

use iced::image::Handle;
use image::{Bgr, DynamicImage, GrayImage, ImageBuffer, RgbImage};

#[cfg(not(feature = "opencv"))]
pub fn convert(payload: &Payload) -> Result<Handle> {
    if let PayloadType::Chunk = payload.payload_type() {
        return Err(anyhow!("unsupported chunk payload type"));
    }
    let info = payload.image_info().unwrap();
    let width = info.width as u32;
    let height = info.height as u32;
    let pf = info.pixel_format;
    let buf = payload.image().unwrap().to_vec();

    let image = match pf {
        PixelFormat::Mono8 => DynamicImage::ImageLuma8(
            GrayImage::from_raw(width, height, buf).ok_or_else(|| anyhow!("Wrong image data"))?,
        ),
        PixelFormat::RGB8 => DynamicImage::ImageRgb8(
            RgbImage::from_raw(width, height, buf).ok_or_else(|| anyhow!("Wrong image data"))?,
        ),
        PixelFormat::BGR8 => DynamicImage::ImageBgr8(
            ImageBuffer::<Bgr<u8>, _>::from_raw(width, height, buf)
                .ok_or_else(|| anyhow!("Wrong image data"))?,
        ),
        _ => {
            if buf.len() / (width * height) as usize == 1 {
                DynamicImage::ImageLuma8(
                    GrayImage::from_raw(width, height, buf)
                        .ok_or_else(|| anyhow!("Wrong image data"))?,
                )
            } else {
                return Err(anyhow!("unsupported pixel format: {:?}", pf));
            }
        }
    };
    let bgra = image.into_bgra8();
    Ok(Handle::from_pixels(
        width as u32,
        height as u32,
        bgra.into_raw(),
    ))
}
