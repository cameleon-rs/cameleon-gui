use super::{
    context::CameraId,
    context::Context,
    genapi::{self, GenApi},
    Result,
};
use iced::{Element, Length, Space};
use std::{
    collections::{HashMap, HashSet},
    ops::{Index, IndexMut},
};

#[derive(Debug)]
pub enum Msg {
    GenApi(CameraId, genapi::Msg),
    Load(CameraId),
    SyncIds,
}

#[derive(Default)]
pub struct Features {
    genapis: HashMap<CameraId, GenApi>,
}

macro_rules! space {
    () => {
        Space::new(Length::Fill, Length::Fill).into()
    };
}

impl Features {
    pub fn view(&mut self, ctx: &mut Context) -> Element<Msg> {
        if let Some(selected) = ctx.selected() {
            let genapi = &mut self[selected];
            match selected.params_ctx(ctx) {
                Ok(mut params_ctx) => genapi
                    .view(&mut params_ctx)
                    .map(move |msg| Msg::GenApi(selected, msg)),
                Err(err) => {
                    tracing::trace!("{}", err);
                    space!()
                }
            }
        } else {
            space!()
        }
    }

    pub fn update(&mut self, msg: Msg, ctx: &mut Context) -> Result<()> {
        match msg {
            Msg::GenApi(id, msg) => {
                self[id].update(msg, &mut id.params_ctx(ctx)?)?;
            }
            Msg::Load(id) => {
                self[id].load(&mut id.params_ctx(ctx)?)?;
            }
            Msg::SyncIds => {
                let old_ids: HashSet<CameraId> = self.genapis.keys().copied().collect();
                let new_ids: HashSet<CameraId> = ctx.ids().copied().collect();
                for dissappered in old_ids.difference(&new_ids) {
                    self.genapis.remove(dissappered);
                }
                for newly_added in new_ids.difference(&old_ids) {
                    self.genapis.insert(*newly_added, GenApi::new());
                }
            }
        }
        Ok(())
    }
}

impl Index<CameraId> for Features {
    type Output = GenApi;
    fn index(&self, index: CameraId) -> &Self::Output {
        self.genapis.get(&index).unwrap()
    }
}

impl IndexMut<CameraId> for Features {
    fn index_mut(&mut self, index: CameraId) -> &mut Self::Output {
        self.genapis.get_mut(&index).unwrap()
    }
}
