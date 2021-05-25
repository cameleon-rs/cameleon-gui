use std::collections::HashMap;
use std::iter::Iterator;

use super::{
    camera::{Camera, CameraId},
    Error, Result,
};

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
        if !self.cameras.contains_key(&id) {
            self.cameras.insert(id, camera);
        }
        id
    }

    pub fn remove(&mut self, id: CameraId) -> Result<()> {
        if let Some(_) = self.cameras.remove(&id) {
            Ok(())
        } else {
            Err(Error::NotFound(id))
        }
    }

    pub fn get(&self, id: CameraId) -> Result<&Camera> {
        self.cameras.get(&id).ok_or_else(|| Error::NotFound(id))
    }

    pub fn get_mut(&mut self, id: CameraId) -> Result<&mut Camera> {
        self.cameras.get_mut(&id).ok_or_else(|| Error::NotFound(id))
    }

    pub fn selected(&self) -> Option<(&Camera, CameraId)> {
        self.selected
            .map(|id| self.cameras.get(&id).map(|cam| (cam, id)))
            .flatten()
    }

    pub fn selected_mut(&mut self) -> Option<(&mut Camera, CameraId)> {
        self.selected
            .map(move |id| self.cameras.get_mut(&id).map(|cam| (cam, id)))
            .flatten()
    }

    pub fn iter(&self) -> impl Iterator<Item = (&CameraId, &Camera)> {
        self.cameras.iter()
    }

    pub fn iter_mut(&mut self) -> impl Iterator<Item = (&CameraId, &mut Camera)> {
        self.cameras.iter_mut()
    }
}
