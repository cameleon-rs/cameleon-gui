use std::{
    collections::{HashMap, HashSet},
    hash::{Hash, Hasher},
    iter::Iterator,
    ops::{Index, IndexMut},
};

use cameleon::{
    camera::CameraInfo,
    genapi::{DefaultGenApiCtxt, ParamsCtxt},
    payload::PayloadReceiver,
    DeviceControl, PayloadStream,
};

use super::{
    camera::{enumerate_cameras, Camera, ControlHandle},
    Error, Result,
};

#[derive(Default)]
pub struct Context {
    cameras: HashMap<CameraId, Camera>,
    selected: Option<CameraId>,
}

pub enum Msg {
    Update,
}

pub enum OutMsg {
    Add(CameraId),
    Remove(CameraId),
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

    pub fn update(&mut self, _msg: Msg) -> Result<Vec<OutMsg>> {
        let old_ids: HashSet<CameraId> = self.cameras().copied().collect();
        let cameras = enumerate_cameras()?;
        let added: HashSet<CameraId> = cameras.into_iter().map(|cam| self.add(cam)).collect();
        let removed = old_ids
            .difference(&added)
            .map(|id| self.remove(*id))
            .map(OutMsg::Remove);
        let newly_added = added.difference(&old_ids).copied().map(OutMsg::Add);
        Ok(removed.chain(newly_added).collect())
    }

    fn add(&mut self, camera: Camera) -> CameraId {
        let id = id(camera.info());
        self.cameras.entry(id).or_insert(camera);
        id
    }

    fn remove(&mut self, id: CameraId) -> CameraId {
        self.cameras.remove(&id);
        if self.selected == Some(id) {
            self.selected = None
        }
        id
    }
}

fn id(info: &CameraInfo) -> CameraId {
    let mut hasher = std::collections::hash_map::DefaultHasher::new();
    info.hash(&mut hasher);
    let hash = hasher.finish();
    CameraId(hash)
}

impl Index<CameraId> for Context {
    type Output = Camera;
    fn index(&self, index: CameraId) -> &Self::Output {
        self.get(index).unwrap()
    }
}

impl IndexMut<CameraId> for Context {
    fn index_mut(&mut self, index: CameraId) -> &mut Self::Output {
        self.get_mut(index).unwrap()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct CameraId(u64);

impl CameraId {
    pub fn open(self, ctx: &mut Context) -> Result<()> {
        let cam = &mut ctx[self];
        cam.open()?;
        cam.load_context()?;
        Ok(())
    }

    pub fn close(self, ctx: &mut Context) -> Result<()> {
        let cam = &mut ctx[self];
        cam.close()?;
        Ok(())
    }

    pub fn start_streaming(self, ctx: &mut Context) -> Result<PayloadReceiver> {
        let cam = &mut ctx[self];
        let receiver = cam.start_streaming(1)?;
        Ok(receiver)
    }

    pub fn stop_streaming(self, ctx: &mut Context) -> Result<()> {
        let cam = &mut ctx[self];
        cam.stop_streaming()?;
        Ok(())
    }

    pub fn is_opened(self, ctx: &Context) -> bool {
        let cam = &ctx[self];
        cam.ctrl.is_opened()
    }

    pub fn is_streaming(self, ctx: &Context) -> bool {
        let cam = &ctx[self];
        cam.strm.is_loop_running()
    }

    pub fn params_ctxt(
        self,
        ctx: &mut Context,
    ) -> Result<ParamsCtxt<&mut ControlHandle, &mut DefaultGenApiCtxt>> {
        let cam = &mut ctx[self];
        let params_ctx = cam.params_ctxt()?;
        Ok(params_ctx)
    }
}
