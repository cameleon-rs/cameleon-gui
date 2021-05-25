use cameleon::payload::Payload;
use cameleon_device::PixelFormat;

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

pub fn convert(payload: &Payload) -> Result<image::ImageBuffer<image::Bgra<u8>, Vec<u8>>> {
    let info = payload.image_info();
    if info.is_none() {
        return Err(Error::FailedConversion);
    }
    let info = info.unwrap();
    let width = info.width as i32;
    let height = info.height as i32;
    let pf = info.pixel_format;
    let mut frame = payload.image().unwrap();
    let channel = frame.len() / (width * height) as usize;
    let data = frame.as_mut_ptr() as *mut libc::c_void;
    let src = unsafe {
        Mat::new_rows_cols_with_data(
            height,
            width,
            cv_core::CV_MAKETYPE(cv_core::CV_8U, channel as i32),
            data,
            cv_core::Mat_AUTO_STEP,
        )
    }
    .unwrap();
    let mut dst = Mat::default();
    if let Some(code) = code(pf) {
        imgproc::cvt_color(&src, &mut dst, code, 0).unwrap();
        let len = (dst.cols() * dst.rows() * dst.channels().unwrap()) as usize;
        let dst_slice =
            unsafe { std::slice::from_raw_parts(dst.data().unwrap() as *const u8, len) };
        let mut dst_vec = Vec::new();
        dst_vec.extend_from_slice(dst_slice);
        Ok(image::ImageBuffer::from_raw(width as u32, height as u32, dst_vec).unwrap())
    } else {
        Err(Error::FailedConversion)
    }
}
