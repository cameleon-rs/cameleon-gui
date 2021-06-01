#[cfg(feature = "cv")]
mod cv;
#[cfg(not(feature = "cv"))]
mod image;

#[cfg(feature = "cv")]
mod image_process {
    pub use super::cv::convert;
}

#[cfg(not(feature = "cv"))]
mod image_process {
    pub use super::image::convert;
}

use cameleon::payload::{Payload, PayloadType, PixelFormat};
use iced::image::Handle;
use std::borrow::Cow;

#[cfg(feature = "cv")]
use opencv::Error as CvError;

#[cfg(not(feature = "cv"))]
#[derive(Debug, thiserror::Error)]
pub enum CvError {}

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("cv error: {0:?}")]
    CvError(#[from] CvError),

    #[error("image error: {0}")]
    ImageError(#[from] ::image::error::ImageError),

    #[error("unsupported pixel format: {0:?}")]
    UnsupportedPixelFormat(PixelFormat),

    #[error("invalid data: {0}")]
    InvalidData(Cow<'static, str>),
}

pub type Result<T> = std::result::Result<T, Error>;

pub fn convert(payload: &Payload) -> Result<Handle> {
    if let PayloadType::Chunk = payload.payload_type() {
        return Err(Error::InvalidData("unsupported chunk payload type".into()));
    }
    let info = payload
        .image_info()
        .ok_or_else(|| Error::InvalidData("not image".into()))?;
    let buf = payload
        .image()
        .ok_or_else(|| Error::InvalidData("not image".into()))?;
    let image = image_process::convert(buf, info)?;
    let bgra = image.into_bgra8();
    Ok(Handle::from_pixels(
        info.width as u32,
        info.height as u32,
        bgra.into_raw(),
    ))
}
