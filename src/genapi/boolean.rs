use crate::Result;
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
    Select(bool),
    Ignore,
}

impl Node {
    pub fn new<T: DeviceControl, U: GenApiCtxt>(
        inner: BooleanNode,
        ctx: &mut ParamsCtxt<T, U>,
    ) -> Self {
        Self {
            inner,
            name: inner.as_node().display_name(ctx).to_string(),
        }
    }

    pub fn view<T: DeviceControl, U: GenApiCtxt>(
        &mut self,
        ctx: &mut ParamsCtxt<T, U>,
    ) -> Result<Element<Msg>> {
        let name = Text::new(&self.name).width(Length::FillPortion(1));
        let value: Element<_> = if self.inner.is_readable(ctx)? {
            let value = self.inner.value(ctx)?;
            let msg = if self.inner.is_writable(ctx)? {
                Msg::Select
            } else {
                |_| Msg::Ignore
            };
            Checkbox::new(value, "", msg)
                .width(Length::FillPortion(1))
                .into()
        } else {
            util::not_available().width(Length::FillPortion(1)).into()
        };
        Ok(Row::new().push(name).push(value).into())
    }

    pub fn update<T: DeviceControl, U: GenApiCtxt>(
        &mut self,
        message: Msg,
        ctx: &mut ParamsCtxt<T, U>,
    ) -> Result<()> {
        if let Msg::Select(value) = message {
            if !self.inner.is_writable(ctx)? {
                return Ok(());
            }
            self.inner.set_value(ctx, value)?;
        }
        Ok(())
    }
}
