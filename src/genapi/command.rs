use super::util;
use crate::Result;
use cameleon::{
    genapi::{CommandNode, GenApiCtxt, ParamsCtxt},
    DeviceControl,
};
use iced::{button, Button, Element, Length, Row, Text};
use std::time;

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
    ) -> Result<Element<Msg>> {
        let name = Text::new(&self.name).width(Length::FillPortion(1));
        let value: Element<_> = if self.inner.is_writable(ctx)? {
            Button::new(&mut self.execute, Text::new("Execute"))
                .on_press(Msg::Execute)
                .width(Length::FillPortion(1))
                .into()
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
            Msg::Execute => {
                if !self.inner.is_writable(ctx)? {
                    Ok(())
                } else {
                    self.inner.execute(ctx)?;
                    let now = time::Instant::now();
                    loop {
                        if now.elapsed() > time::Duration::from_secs(1) {
                            break;
                        }
                        if self.inner.is_done(ctx)? {
                            break;
                        }
                    }
                    Ok(())
                }
            }
        }
    }
}
