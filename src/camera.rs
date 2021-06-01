use cameleon::{
    genapi::DefaultGenApiCtxt, payload::PayloadSender, u3v, CameleonResult, ControlResult,
    DeviceControl, PayloadStream, StreamResult,
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

#[derive(From)]
pub enum StreamHandle {
    U3V(u3v::StreamHandle),
}

macro_rules! delegate {
    ($(fn $method:ident(&self $(,$arg:ident: $arg_ty:ty)*) -> $rt_ty: ty;)*) => {
        $(
            fn $method(&self $(,$arg: $arg_ty)*) -> $rt_ty {
                match self {
                    Self::U3V(handle) => handle.$method($($arg),*)
                }
            }
        )*
    }
}

macro_rules! delegate_mut {
    ($(fn $method:ident(&mut self $(,$arg:ident: $arg_ty:ty)*) -> $rt_ty: ty;)*) => {
        $(
            fn $method(&mut self $(,$arg: $arg_ty)*) -> $rt_ty {
                match self {
                    Self::U3V(handle) => handle.$method($($arg),*)
                }
            }
        )*
    }
}

impl ControlHandle {
    pub fn user_defined_name(&self) -> Option<&str> {
        match self {
            ControlHandle::U3V(handle) => handle.device_info().user_defined_name.as_deref(),
        }
    }
}

impl DeviceControl for ControlHandle {
    delegate_mut! {
        fn open(&mut self) -> ControlResult<()>;
        fn close(&mut self) -> ControlResult<()>;
        fn read(&mut self, address: u64, buf: &mut [u8]) -> ControlResult<()>;
        fn write(&mut self, address: u64, data: &[u8]) -> ControlResult<()>;
        fn genapi(&mut self) -> ControlResult<String>;
        fn enable_streaming(&mut self) -> ControlResult<()>;
        fn disable_streaming(&mut self) -> ControlResult<()>;
    }

    delegate! {
        fn is_opened(&self) -> bool;
    }
}

impl PayloadStream for StreamHandle {
    delegate_mut! {
        fn open(&mut self) -> StreamResult<()>;
        fn close(&mut self) -> StreamResult<()>;
        fn start_streaming_loop(&mut self,sender: PayloadSender,ctrl: &mut dyn DeviceControl) -> StreamResult<()>;
        fn stop_streaming_loop(&mut self) -> StreamResult<()>;

    }

    delegate! {
        fn is_loop_running(&self) -> bool;
    }
}
