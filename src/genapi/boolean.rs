use cameleon::{
    genapi::{node_kind::BooleanNode, GenApiCtxt, ParamsCtxt},
    DeviceControl,
};
use iced::{Checkbox, Element, Length, Row, Text};
pub struct Node {
    inner: BooleanNode,
    name: String,
}

use super::util;

#[derive(Debug, Clone)]
pub enum Msg {
    Selected(bool),
    Ignore(bool),
}

impl Node {
    pub fn new<T: DeviceControl, U: GenApiCtxt>(
        inner: BooleanNode,
        ctxt: &mut ParamsCtxt<T, U>,
    ) -> Self {
        Self {
            inner,
            name: inner.as_node().display_name(ctxt).to_string(),
        }
    }

    pub fn view<T: DeviceControl, U: GenApiCtxt>(
        &mut self,
        cx: &mut ParamsCtxt<T, U>,
    ) -> Element<Msg> {
        let name = Text::new(&self.name).width(Length::FillPortion(1));
        let value: Element<_> = if self.inner.is_readable(cx).unwrap() {
            let value = self.inner.value(cx).unwrap();
            let msg = if self.inner.is_writable(cx).unwrap() {
                Msg::Selected
            } else {
                Msg::Ignore
            };
            Checkbox::new(value, "", msg)
                .width(Length::FillPortion(1))
                .into()
        } else {
            util::not_available().width(Length::FillPortion(1)).into()
        };
        Row::new().push(name).push(value).into()
    }

    pub fn update<T: DeviceControl, U: GenApiCtxt>(
        &mut self,
        message: Msg,
        cx: &mut ParamsCtxt<T, U>,
    ) {
        match message {
            Msg::Selected(value) => {
                if !self.inner.is_writable(cx).unwrap() {
                    return;
                }
                self.inner.set_value(cx, value).unwrap();
            }
            Msg::Ignore(_) => (),
        }
    }
}
