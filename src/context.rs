use std::{
    collections::HashMap,
    hash::{Hash, Hasher},
    iter::Iterator,
};

use cameleon::{camera::CameraInfo, payload::PayloadReceiver, DeviceControl, PayloadStream};

use super::{camera::Camera, Error, Result};

#[derive(Default)]
pub struct Context {
    cameras: HashMap<CameraId, Camera>,
    selected: Option<CameraId>,
}

impl Context {
    pub fn select(&mut self, id: CameraId) -> Result<()> {
        if self.cameras.contains_key(&id) {
            self.selected = Some(id);
            Ok(())
        } else {
            Err(Error::NotFound(id))
        }
    }

    pub fn add(&mut self, camera: Camera) -> CameraId {
        let id = CameraId::new(camera.info());
        self.cameras.entry(id).or_insert(camera);
        id
    }

    pub fn remove(&mut self, id: CameraId) -> Result<()> {
        if self.cameras.remove(&id).is_some() {
            Ok(())
        } else {
            Err(Error::NotFound(id))
        }
    }

    pub fn get(&self, id: CameraId) -> Option<&Camera> {
        self.cameras.get(&id)
    }

    pub fn get_mut(&mut self, id: CameraId) -> Option<&mut Camera> {
        self.cameras.get_mut(&id)
    }

    pub fn selected(&self) -> Option<CameraId> {
        self.selected
    }

    pub fn cameras(&self) -> impl Iterator<Item = &CameraId> {
        self.cameras.keys()
    }

    pub fn with_camera_or_else<D, F, R>(&self, id: CameraId, default: D, f: F) -> R
    where
        D: FnOnce() -> R,
        F: FnOnce(&Camera) -> R,
    {
        if let Some(cam) = self.get(id) {
            f(cam)
        } else {
            default()
        }
    }

    pub fn with_camera_mut_or_else<D, F, R>(&mut self, id: CameraId, default: D, f: F) -> R
    where
        D: FnOnce() -> R,
        F: FnOnce(&mut Camera) -> R,
    {
        if let Some(cam) = self.get_mut(id) {
            f(cam)
        } else {
            default()
        }
    }

    pub fn with_selected_or_else<D, F, R>(&self, default: D, f: F) -> R
    where
        D: FnOnce() -> R,
        F: FnOnce(&Camera) -> R,
    {
        if let Some(id) = self.selected() {
            self.with_camera_or_else(id, default, f)
        } else {
            default()
        }
    }

    pub fn with_selected_mut_or_else<D, F, R>(&mut self, default: D, f: F) -> R
    where
        D: FnOnce() -> R,
        F: FnOnce(&mut Camera) -> R,
    {
        if let Some(id) = self.selected() {
            self.with_camera_mut_or_else(id, default, f)
        } else {
            default()
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct CameraId(u64);

impl CameraId {
    pub fn open(self, ctx: &mut Context) -> Result<()> {
        ctx.with_camera_mut_or_else(
            self,
            || Err(Error::NotFound(self)),
            |cam| {
                cam.open()?;
                cam.load_context()?;
                Ok(())
            },
        )
    }

    pub fn close(self, ctx: &mut Context) -> Result<()> {
        ctx.with_camera_mut_or_else(
            self,
            || Err(Error::NotFound(self)),
            |cam| cam.close().map_err(Into::into),
        )
    }

    pub fn start_streaming(self, ctx: &mut Context) -> Result<PayloadReceiver> {
        ctx.with_camera_mut_or_else(
            self,
            || Err(Error::NotFound(self)),
            |cam| cam.start_streaming(1).map_err(Into::into),
        )
    }

    pub fn stop_streaming(self, ctx: &mut Context) -> Result<()> {
        ctx.with_camera_mut_or_else(
            self,
            || Err(Error::NotFound(self)),
            |cam| cam.stop_streaming().map_err(Into::into),
        )
    }

    pub fn is_opened(self, ctx: &Context) -> bool {
        ctx.with_camera_or_else(self, || false, |cam| cam.ctrl.is_opened())
    }

    pub fn is_streaming(self, ctx: &Context) -> bool {
        ctx.with_camera_or_else(self, || false, |cam| cam.strm.is_loop_running())
    }

    fn new(info: &CameraInfo) -> Self {
        let mut hasher = std::collections::hash_map::DefaultHasher::new();
        info.hash(&mut hasher);
        let hash = hasher.finish();
        Self(hash)
    }
}
