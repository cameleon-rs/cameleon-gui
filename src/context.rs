use std::collections::{HashMap, HashSet};

use super::camera::{Camera, CameraId, State};
use cameleon::u3v;
use tracing::trace;

#[derive(Default)]
pub struct Context {
    pub cameras: HashMap<CameraId, Camera>,
    pub selected: Option<CameraId>,
}

impl Context {
    pub fn new() -> Self {
        let mut ctx = Self::default();
        ctx.refresh();
        ctx
    }

    pub fn refresh(&mut self) {
        let raws = u3v::enumerate_cameras().unwrap(); // TODO: Add error handling
        trace!("Detected {} cameras", raws.len());
        let mut exists: HashSet<CameraId> = self.cameras.iter().map(|(id, _)| *id).collect();
        for raw in raws {
            let camera = Camera::new(raw).unwrap();
            if exists.contains(&camera.id) {
                exists.remove(&camera.id);
                continue;
            }
            self.cameras.insert(camera.id, camera);
        }
        for removed in exists {
            // TODO: close and/or stop streaming
            self.cameras.remove(&removed);
        }

        // Drop selected if it removed
        if let Some(selected) = self.selected {
            if !self.cameras.contains_key(&selected) {
                self.selected = None
            }
        }

        // Randomly selected if it is `None`
        if self.selected.is_none() && !self.cameras.is_empty() {
            self.selected = Some(*self.cameras.keys().next().unwrap());
        }
    }

    pub fn selected(&self) -> Option<&Camera> {
        self.selected.map(|id| self.cameras.get(&id)).flatten()
    }

    pub fn selected_mut(&mut self) -> Option<&mut Camera> {
        self.selected
            .map(move |id| self.cameras.get_mut(&id))
            .flatten()
    }

    pub fn selected_state(&self) -> Option<State> {
        self.selected().map(|cam| cam.state())
    }
}
