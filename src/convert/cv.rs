use anyhow::anyhow;
use cameleon::payload::ImageInfo;
use cameleon_device::PixelFormat;
use image::DynamicImage;
use opencv::core as cv_core;
use opencv::imgproc;
use opencv::prelude::*;

use crate::{Error, Result};

fn code(pf: PixelFormat) -> Option<i32> {
    match pf {
        PixelFormat::Mono8 => Some(imgproc::COLOR_GRAY2BGRA),
        PixelFormat::Mono12 => None,
        PixelFormat::RGB8 => Some(imgproc::COLOR_RGB2BGRA),
        PixelFormat::BGR8 => Some(imgproc::COLOR_BGR2BGRA),
        PixelFormat::BayerRG8 => Some(imgproc::COLOR_BayerRG2RGBA), // opencv's bug
        PixelFormat::BayerBG8 => Some(imgproc::COLOR_BayerBG2BGRA),
        PixelFormat::BayerGR8 => Some(imgproc::COLOR_BayerGR2BGRA),
        PixelFormat::BayerGB8 => Some(imgproc::COLOR_BayerGB2BGRA),
        _ => None,
    }
}

fn into_failed(err: impl std::error::Error + Send + Sync + 'static) -> Error {
    Error::ConversionError(err.into())
}

pub fn convert_impl(buf: &[u8], info: &ImageInfo) -> Result<DynamicImage> {
    let mut buf = buf.to_vec();
    let width = info.width as i32;
    let height = info.height as i32;
    let pf = info.pixel_format;
    let channel = buf.len() / (width * height) as usize;
    let data = buf.as_mut_ptr() as *mut libc::c_void;
    let src = unsafe {
        Mat::new_rows_cols_with_data(
            height,
            width,
            cv_core::CV_MAKETYPE(cv_core::CV_8U, channel as i32),
            data,
            cv_core::Mat_AUTO_STEP,
        )
    }
    .map_err(into_failed)?;
    let mut dst = Mat::default();
    if let Some(code) = code(pf) {
        imgproc::cvt_color(&src, &mut dst, code, 0).map_err(into_failed)?;
        let len = (dst.cols() * dst.rows() * dst.channels().map_err(into_failed)?) as usize;
        let dst_slice = unsafe {
            std::slice::from_raw_parts(dst.data().map_err(into_failed)? as *const u8, len)
        };
        let mut dst_vec = Vec::new();
        dst_vec.extend_from_slice(dst_slice);
        Ok(DynamicImage::ImageBgra8(
            image::ImageBuffer::from_raw(width as u32, height as u32, dst_vec)
                .ok_or_else(|| Error::ConversionError(anyhow!("wrong image data")))?,
        ))
    } else {
        Err(Error::ConversionError(anyhow!(
            "unsupported pixel format: {:?}",
            pf
        )))
    }
}
