use super::{
    context::CameraId,
    context::Context,
    genapi::{self, GenApi},
    Result,
};
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
            if let Some(selected) = ctx.selected();
            if selected.is_opened(ctx);
            if let Some(genapi) = self.genapis.get_mut(&selected);
            if let Some(cam) = ctx.get_mut(selected);
            if let Ok(params_ctx) = &mut cam.params_ctxt();
            then {
                genapi.view(params_ctx).map(move |msg| Msg::GenApi(selected, msg))
            } else {
                space!()
            }
        }
    }

    pub fn update(&mut self, msg: Msg, ctx: &mut Context) -> Result<()> {
        match msg {
            Msg::GenApi(id, msg) => {
                if let Some(genapi) = self.genapis.get_mut(&id) {
                    let cam = ctx.get_mut(id).unwrap();
                    genapi.update(msg, &mut cam.params_ctxt()?)?;
                }
            }
            Msg::Load(id) => {
                if !self.genapis.contains_key(&id) {
                    if let Some(cam) = ctx.get_mut(id) {
                        let genapi = GenApi::new(&mut cam.params_ctxt()?);
                        self.genapis.insert(id, genapi);
                    }
                }
            }
        }
        Ok(())
    }
}
