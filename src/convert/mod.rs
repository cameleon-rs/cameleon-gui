#[cfg(feature = "opencv")]
mod cv_convert;
#[cfg(feature = "opencv")]
pub use cv_convert::convert;

use crate::{Error, Result};
use anyhow::anyhow;
use cameleon::payload::{Payload, PayloadType};
use cameleon_device::PixelFormat;

use iced::image::Handle;
use image::{Bgr, DynamicImage, GrayImage, ImageBuffer, RgbImage};

macro_rules! failed {
    ($($arg: expr),*) => {
        Error::FailedConversion(anyhow!($($arg),*))
    };
}

#[cfg(not(feature = "opencv"))]
pub fn convert(payload: &Payload) -> Result<Handle> {
    if let PayloadType::Chunk = payload.payload_type() {
        return Err(failed!("unsupported chunk payload type"));
    }
    let info = payload.image_info().ok_or_else(|| failed!("not image"))?;
    let width = info.width as u32;
    let height = info.height as u32;
    let pf = info.pixel_format;
    let buf = payload
        .image()
        .ok_or_else(|| failed!("not image"))?
        .to_vec();

    let image = match pf {
        PixelFormat::Mono8 => DynamicImage::ImageLuma8(
            GrayImage::from_raw(width, height, buf).ok_or_else(|| failed!("wrong image data"))?,
        ),
        PixelFormat::RGB8 => DynamicImage::ImageRgb8(
            RgbImage::from_raw(width, height, buf).ok_or_else(|| failed!("wrong image data"))?,
        ),
        PixelFormat::BGR8 => DynamicImage::ImageBgr8(
            ImageBuffer::<Bgr<u8>, _>::from_raw(width, height, buf)
                .ok_or_else(|| failed!("wrong image data"))?,
        ),
        _ => {
            if buf.len() / (width * height) as usize == 1 {
                DynamicImage::ImageLuma8(
                    GrayImage::from_raw(width, height, buf)
                        .ok_or_else(|| failed!("wrong image data"))?,
                )
            } else {
                return Err(failed!("unsupported pixel format: {:?}", pf));
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
