use cameleon::{
    genapi::{node_kind::FloatNode, GenApiCtxt, ParamsCtxt},
    DeviceControl,
};
use iced::{text_input, Element, Length, Row, Text, TextInput};

use super::util;

pub struct Node {
    inner: FloatNode,
    name: String,
    state: text_input::State,
}

#[derive(Debug, Clone)]
pub enum Msg {
    Changed(String),
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
    ) -> Element<Msg> {
        let name = Text::new(&self.name).width(Length::FillPortion(1));
        let value: Element<_> = if let Ok(value) = self.inner.value(cx) {
            let value = &value.to_string();
            if self.inner.is_writable(cx).unwrap() {
                TextInput::new(&mut self.state, "", value, Msg::Changed)
                    .width(Length::FillPortion(1))
                    .into()
            } else {
                Text::new(value).width(Length::FillPortion(1)).into()
            }
        } else {
            util::not_available().width(Length::FillPortion(1)).into()
        };
        Row::new().push(name).push(value).into()
    }

    pub fn update(&mut self, msg: Msg, cx: &mut ParamsCtxt<impl DeviceControl, impl GenApiCtxt>) {
        match msg {
            Msg::Changed(s) => {
                if !self.inner.is_writable(cx).unwrap() {
                    return;
                }
                if let Ok(value) = s.parse::<f64>() {
                    self.inner.set_value(cx, value).unwrap()
                }
            }
        }
    }
}
