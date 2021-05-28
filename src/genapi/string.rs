use super::util;
use crate::Result;
use cameleon::{
    genapi::{node_kind::StringNode, GenApiCtxt, ParamsCtxt},
    DeviceControl,
};
use iced::{text_input, Element, Length, Row, Text, TextInput};

pub struct Node {
    inner: StringNode,
    name: String,
    state: text_input::State,
}

#[derive(Debug, Clone)]
pub enum Msg {
    Change(String),
}

impl Node {
    pub fn new(
        inner: StringNode,
        ctx: &mut ParamsCtxt<impl DeviceControl, impl GenApiCtxt>,
    ) -> Self {
        Self {
            inner,
            name: inner.as_node().name(ctx).to_string(),
            state: text_input::State::new(),
        }
    }

    pub fn view(
        &mut self,
        ctx: &mut ParamsCtxt<impl DeviceControl, impl GenApiCtxt>,
    ) -> Result<Element<Msg>> {
        let name = Text::new(&self.name).width(Length::FillPortion(1));
        let value: Element<_> = if let Ok(value) = self.inner.value(ctx) {
            if self.inner.is_writable(ctx)? {
                TextInput::new(&mut self.state, "", &value, Msg::Change)
                    .width(Length::FillPortion(1))
                    .into()
            } else {
                Text::new(value).width(Length::FillPortion(1)).into()
            }
        } else {
            util::not_available().width(Length::FillPortion(1)).into()
        };
        Ok(Row::new().push(name).push(value).into())
    }

    pub fn update(
        &mut self,
        msg: Msg,
        ctx: &mut ParamsCtxt<impl DeviceControl, impl GenApiCtxt>,
    ) -> Result<()> {
        match msg {
            Msg::Change(s) => {
                if self.inner.is_writable(ctx)? {
                    Ok(self.inner.set_value(ctx, s)?)
                } else {
                    Ok(())
                }
            }
        }
    }
}
