use anyhow::Result;
use std::fmt;

use cameleon::{genapi::DefaultGenApiCtxt, u3v, DeviceControl, PayloadStream};

pub struct Camera {
    pub raw: cameleon::Camera<u3v::ControlHandle, u3v::StreamHandle, DefaultGenApiCtxt>,
    pub name: String,
    pub id: CameraId,
}

impl Camera {
    pub fn new(
        raw: cameleon::Camera<u3v::ControlHandle, u3v::StreamHandle, DefaultGenApiCtxt>,
    ) -> Result<Self> {
        let id = CameraId::new(&raw.ctrl.device_info().guid)?;
        let name = name(&raw);
        Ok(Self { raw, name, id })
    }

    pub fn state(&self) -> State {
        if self.raw.strm.is_loop_running() {
            State::Streaming
        } else if self.raw.ctrl.is_opened() {
            State::Opened
        } else {
            State::Closed
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct CameraId(usize);

impl fmt::Display for CameraId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_fmt(format_args!("CameraId: {:#x}", self.0))
    }
}

impl CameraId {
    pub fn new<T: AsRef<str>>(guid: T) -> Result<Self> {
        Ok(CameraId(usize::from_str_radix(guid.as_ref(), 16)?))
    }
}

#[derive(Debug, Clone, Copy)]
pub enum State {
    Closed,
    Opened,
    Streaming,
}

impl State {
    pub fn is_open(&self) -> bool {
        match self {
            State::Closed => false,
            _ => true,
        }
    }

    pub fn is_streaming(&self) -> bool {
        match self {
            State::Streaming => true,
            _ => false,
        }
    }
}

fn name(
    raw: &cameleon::Camera<u3v::ControlHandle, u3v::StreamHandle, DefaultGenApiCtxt>,
) -> String {
    let info = raw.ctrl.device_info();
    let name = match info.user_defined_name {
        Some(ref name) => {
            if name != "" {
                name
            } else {
                &info.model_name
            }
        }
        None => &info.model_name,
    };
    format!("{} ({})", name, info.serial_number)
}
