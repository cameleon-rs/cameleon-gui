use super::util;
use crate::Result;
use cameleon::{
    genapi::{node_kind::FloatNode, GenApiCtxt, ParamsCtxt},
    DeviceControl,
};
use iced::{text_input, Element, Length, Row, Text, TextInput};

pub struct Node {
    inner: FloatNode,
    name: String,
    state: text_input::State,
}

#[derive(Debug, Clone)]
pub enum Msg {
    Change(String),
}

impl Node {
    pub fn new(inner: FloatNode, cx: &mut ParamsCtxt<impl DeviceControl, impl GenApiCtxt>) -> Self {
        Self {
            inner,
            name: inner.as_node().name(cx).to_string(),
            state: text_input::State::new(),
        }
    }

    pub fn view(
        &mut self,
        cx: &mut ParamsCtxt<impl DeviceControl, impl GenApiCtxt>,
    ) -> Result<Element<Msg>> {
        let name = Text::new(&self.name).width(Length::FillPortion(1));
        let value: Element<_> = if let Ok(value) = self.inner.value(cx) {
            let value = &value.to_string();
            if self.inner.is_writable(cx)? {
                TextInput::new(&mut self.state, "", value, Msg::Change)
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
        cx: &mut ParamsCtxt<impl DeviceControl, impl GenApiCtxt>,
    ) -> Result<()> {
        let Msg::Change(s) = msg;
        if !self.inner.is_writable(cx)? {
            return Ok(());
        }
        if let Ok(value) = s.parse::<f64>() {
            self.inner.set_value(cx, value)?;
        }
        Ok(())
    }
}
