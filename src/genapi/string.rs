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
    Changed(String),
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
        cx: &mut ParamsCtxt<impl DeviceControl, impl GenApiCtxt>,
    ) -> Element<Msg> {
        let name = Text::new(&self.name).width(Length::FillPortion(1));
        let value: Element<_> = if let Ok(value) = self.inner.value(cx) {
            match self.inner.is_writable(cx) {
                Ok(true) => TextInput::new(&mut self.state, "", &value, Msg::Changed)
                    .width(Length::FillPortion(1))
                    .into(),
                Ok(false) => Text::new(value).width(Length::FillPortion(1)).into(),
                Err(err) => {
                    tracing::error!("{}", err);
                    util::not_available().width(Length::FillPortion(1)).into()
                }
            }
        } else {
            util::not_available().width(Length::FillPortion(1)).into()
        };
        Row::new().push(name).push(value).into()
    }

    pub fn update(
        &mut self,
        msg: Msg,
        cx: &mut ParamsCtxt<impl DeviceControl, impl GenApiCtxt>,
    ) -> Result<()> {
        match msg {
            Msg::Changed(s) => {
                if self.inner.is_writable(cx)? {
                    Ok(self.inner.set_value(cx, s)?)
                } else {
                    Ok(())
                }
            }
        }
    }
}
