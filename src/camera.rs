use anyhow::Result;
use std::fmt;

use cameleon::{
    genapi::DefaultGenApiCtxt, payload::PayloadSender, u3v, CameleonResult, ControlResult,
    DeviceControl, PayloadStream, StreamResult,
};
use derive_more::From;

pub type RawCamera = cameleon::Camera<ControlHandle, StreamHandle, DefaultGenApiCtxt>;

pub fn enumerate_raw_cameras() -> CameleonResult<Vec<RawCamera>> {
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
    fn serial_number(&self) -> &str {
        match self {
            ControlHandle::U3V(handle) => &handle.device_info().serial_number,
        }
    }

    fn model_name(&self) -> &str {
        match self {
            ControlHandle::U3V(handle) => &handle.device_info().model_name,
        }
    }

    fn guid(&self) -> &str {
        match self {
            ControlHandle::U3V(handle) => &handle.device_info().guid,
        }
    }

    fn user_defined_name(&self) -> Option<&str> {
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

pub struct Camera {
    pub raw: cameleon::Camera<ControlHandle, StreamHandle, DefaultGenApiCtxt>,
    pub name: String,
    pub id: CameraId,
}

impl Camera {
    pub fn new(
        raw: cameleon::Camera<ControlHandle, StreamHandle, DefaultGenApiCtxt>,
    ) -> Result<Self> {
        let id = CameraId::new(&raw.ctrl.guid())?;
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
        !matches!(self, State::Closed)
    }

    pub fn is_streaming(&self) -> bool {
        matches!(self, State::Streaming)
    }
}

fn name(raw: &cameleon::Camera<ControlHandle, StreamHandle, DefaultGenApiCtxt>) -> String {
    let name = match raw.ctrl.user_defined_name() {
        Some(ref name) => {
            if !name.is_empty() {
                name
            } else {
                raw.ctrl.model_name()
            }
        }
        None => raw.ctrl.model_name(),
    };
    format!("{} ({})", name, raw.ctrl.serial_number())
}
