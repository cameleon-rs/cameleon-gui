#[cfg(feature = "cv")]
mod cv;
#[cfg(feature = "cv")]
use cv::convert_impl;
#[cfg(not(feature = "cv"))]
mod image;
#[cfg(not(feature = "cv"))]
use self::image::convert_impl;

use crate::{Error, Result};
use anyhow::anyhow;
use cameleon::payload::{Payload, PayloadType};

use iced::image::Handle;

pub fn convert(payload: &Payload) -> Result<Handle> {
    if let PayloadType::Chunk = payload.payload_type() {
        return Err(Error::FailedConversion(anyhow!(
            "unsupported chunk payload type"
        )));
    }
    let info = payload
        .image_info()
        .ok_or_else(|| Error::FailedConversion(anyhow!("not image")))?;
    let buf = payload
        .image()
        .ok_or_else(|| Error::FailedConversion(anyhow!("not image")))?;
    let image = convert_impl(buf, info)?;
    let bgra = image.into_bgra8();
    Ok(Handle::from_pixels(
        info.width as u32,
        info.height as u32,
        bgra.into_raw(),
    ))
}
