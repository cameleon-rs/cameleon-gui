use std::collections::{HashMap, HashSet};

use super::camera::{enumerate_cameras, Camera, CameraId};
use tracing::trace;

#[derive(Default)]
pub struct Context {
    pub cameras: HashMap<CameraId, Camera>,
    pub selected: Option<CameraId>,
}

impl Context {
    pub fn refresh(&mut self) {
        let dets = enumerate_cameras().unwrap(); // TODO: Add error handling
        trace!("Detected {} cameras", dets.len());
        let mut exists: HashSet<CameraId> = self.cameras.keys().copied().collect();
        for det in dets {
            let det_id = CameraId::new(det.info());
            if exists.contains(&det_id) {
                exists.remove(&det_id);
                continue;
            }
            self.cameras.insert(det_id, det);
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
}
