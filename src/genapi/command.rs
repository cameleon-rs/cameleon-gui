use super::util;
use crate::Result;
use cameleon::{
    genapi::{node_kind::CommandNode, GenApiCtxt, ParamsCtxt},
    DeviceControl,
};
use iced::{button, Button, Element, Length, Row, Text};

#[derive(Debug, Clone)]
pub enum Msg {
    Execute,
}

pub struct Node {
    inner: CommandNode,
    name: String,
    execute: button::State,
}

impl Node {
    pub fn new(
        inner: CommandNode,
        ctx: &mut ParamsCtxt<impl DeviceControl, impl GenApiCtxt>,
    ) -> Self {
        Self {
            inner,
            name: inner.as_node().name(ctx).to_string(),
            execute: button::State::new(),
        }
    }

    pub fn view(
        &mut self,
        ctx: &mut ParamsCtxt<impl DeviceControl, impl GenApiCtxt>,
    ) -> Element<Msg> {
        let name = Text::new(&self.name).width(Length::FillPortion(1));
        let value: Element<_> = match self.inner.is_writable(ctx) {
            Ok(true) => Button::new(&mut self.execute, Text::new("Execute"))
                .on_press(Msg::Execute)
                .width(Length::FillPortion(1))
                .into(),
            _ => util::not_available().width(Length::FillPortion(1)).into(),
        };
        Row::new().push(name).push(value).into()
    }

    pub fn update(
        &mut self,
        msg: Msg,
        ctx: &mut ParamsCtxt<impl DeviceControl, impl GenApiCtxt>,
    ) -> Result<()> {
        match msg {
            Msg::Execute => {
                if !self.inner.is_writable(ctx)? {
                    Ok(())
                } else {
                    Ok(self.inner.execute(ctx)?)
                }
            }
        }
    }
}
