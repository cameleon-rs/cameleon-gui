use anyhow::Result;
use std::{
    fmt,
    hash::{Hash, Hasher},
};

use cameleon::{
    camera::CameraInfo, genapi::DefaultGenApiCtxt, payload::PayloadSender, u3v, CameleonResult,
    ControlResult, DeviceControl, PayloadStream, StreamResult,
};
use derive_more::From;

pub type Camera = cameleon::Camera<ControlHandle, StreamHandle, DefaultGenApiCtxt>;

pub fn enumerate_cameras() -> CameleonResult<Vec<Camera>> {
    Ok(u3v::enumerate_cameras()?
        .into_iter()
        .map(|cam| cam.convert_into())
        .collect())
}

#[derive(From)]
pub enum ControlHandle {
    U3V(u3v::ControlHandle),
}

impl ControlHandle {
    pub fn user_defined_name(&self) -> Option<&str> {
        match self {
            ControlHandle::U3V(handle) => handle.device_info().user_defined_name.as_deref(),
        }
    }
}

impl cameleon_genapi::Device for ControlHandle {
    fn read_mem(&mut self, address: i64, buf: &mut [u8]) -> Result<(), Box<dyn std::error::Error>> {
        match self {
            ControlHandle::U3V(handle) => handle.read_mem(address, buf),
        }
    }

    fn write_mem(&mut self, address: i64, data: &[u8]) -> Result<(), Box<dyn std::error::Error>> {
        match self {
            ControlHandle::U3V(handle) => handle.write_mem(address, data),
        }
    }
}

impl DeviceControl for ControlHandle {
    fn open(&mut self) -> ControlResult<()> {
        match self {
            ControlHandle::U3V(handle) => handle.open(),
        }
    }

    fn is_opened(&self) -> bool {
        match self {
            ControlHandle::U3V(handle) => handle.is_opened(),
        }
    }

    fn close(&mut self) -> ControlResult<()> {
        match self {
            ControlHandle::U3V(handle) => handle.close(),
        }
    }

    fn read(&mut self, address: u64, buf: &mut [u8]) -> ControlResult<()> {
        match self {
            ControlHandle::U3V(handle) => handle.read(address, buf),
        }
    }

    fn write(&mut self, address: u64, data: &[u8]) -> ControlResult<()> {
        match self {
            ControlHandle::U3V(handle) => handle.write(address, data),
        }
    }

    fn genapi(&mut self) -> ControlResult<String> {
        match self {
            ControlHandle::U3V(handle) => handle.genapi(),
        }
    }

    fn enable_streaming(&mut self) -> ControlResult<()> {
        match self {
            ControlHandle::U3V(handle) => handle.enable_streaming(),
        }
    }

    fn disable_streaming(&mut self) -> ControlResult<()> {
        match self {
            ControlHandle::U3V(handle) => handle.disable_streaming(),
        }
    }
}

#[derive(From)]
pub enum StreamHandle {
    U3V(u3v::StreamHandle),
}

impl PayloadStream for StreamHandle {
    fn open(&mut self) -> StreamResult<()> {
        match self {
            StreamHandle::U3V(handle) => handle.open(),
        }
    }

    fn close(&mut self) -> StreamResult<()> {
        match self {
            StreamHandle::U3V(handle) => handle.close(),
        }
    }

    fn start_streaming_loop(
        &mut self,
        sender: PayloadSender,
        ctrl: &mut dyn DeviceControl,
    ) -> StreamResult<()> {
        match self {
            StreamHandle::U3V(handle) => handle.start_streaming_loop(sender, ctrl),
        }
    }

    fn stop_streaming_loop(&mut self) -> StreamResult<()> {
        match self {
            StreamHandle::U3V(handle) => handle.stop_streaming_loop(),
        }
    }

    fn is_loop_running(&self) -> bool {
        match self {
            StreamHandle::U3V(handle) => handle.is_loop_running(),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct CameraId(u64);

impl fmt::Display for CameraId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_fmt(format_args!("CameraId: {:#x}", self.0))
    }
}

impl CameraId {
    pub fn new(info: &CameraInfo) -> Self {
        let mut hasher = std::collections::hash_map::DefaultHasher::new();
        info.hash(&mut hasher);
        let hash = hasher.finish();
        Self(hash)
    }
}
