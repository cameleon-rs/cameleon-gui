use super::{
    camera::CameraId,
    context::Context,
    genapi::{self, GenApi},
    Result,
};
use cameleon::DeviceControl;
use iced::{Element, Length, Space};
use if_chain::if_chain;
use std::collections::HashMap;

#[derive(Debug)]
pub enum Msg {
    GenApi(CameraId, genapi::Msg),
    Load(CameraId),
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
        if_chain! {
            if let Some((cam, id)) = ctx.selected_mut();
            if cam.ctrl.is_opened();
            if let Some(genapi) = self.genapis.get_mut(&id);
            if let Ok(params_ctx) = &mut cam.params_ctxt();
            then {
                genapi.view(params_ctx).map(move |msg| Msg::GenApi(id, msg))
            } else {
                space!()
            }
        }
    }

    pub fn update(&mut self, msg: Msg, ctx: &mut Context) -> Result<()> {
        match msg {
            Msg::GenApi(id, msg) => {
                if let Some(genapi) = self.genapis.get_mut(&id) {
                    let cam = ctx.get_mut(id)?;
                    genapi.update(msg, &mut cam.params_ctxt()?)?;
                }
            }
            Msg::Load(id) => {
                if !self.genapis.contains_key(&id) {
                    let cam = ctx.get_mut(id)?;
                    let genapi = GenApi::new(&mut cam.params_ctxt()?);
                    self.genapis.insert(id, genapi);
                }
            }
        }
        Ok(())
    }
}
