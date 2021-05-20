use cameleon::payload::{Payload, PayloadReceiver};
use iced::{
    image::{viewer, Handle, Image, Viewer},
    time, Command, Container, Element, Subscription,
};
use std::time::Duration;

use super::{context::Context, convert::convert, Result};

#[derive(Debug)]
pub enum Msg {
    UpdateFrame,
    Acquired(Payload),
    Attach(PayloadReceiver),
    Detach,
}

#[derive(Debug, Default)]
pub struct Frame {
    receiver: Option<PayloadReceiver>,
    handle: Option<Handle>,
    viewer: viewer::State,
}

impl Frame {
    pub fn view(&mut self, _ctx: &Context) -> Element<Msg> {
        let content: Element<_> = if let Some(ref handle) = self.handle {
            Viewer::new(&mut self.viewer, handle.clone()).into()
        } else {
            Image::new("ferris.png").into()
        };
        Container::new(content).into()
    }

    pub fn update(&mut self, msg: Msg, _ctx: &mut Context) -> Result<Command<Msg>> {
        match msg {
            Msg::UpdateFrame => {
                if let Some(receiver) = &self.receiver {
                    let payload = receiver.try_recv()?;
                    self.update(Msg::Acquired(payload), _ctx)?;
                    Ok(Command::none())
                } else {
                    Ok(Command::none())
                }
            }
            Msg::Acquired(payload) => {
                self.handle = Some(convert(&payload).unwrap());
                Ok(Command::none())
            }
            Msg::Attach(receiver) => {
                self.receiver = Some(receiver);
                Ok(Command::none())
            }
            Msg::Detach => {
                self.receiver = None;
                Ok(Command::none())
            }
        }
    }

    pub fn subscription(&self) -> Subscription<Msg> {
        time::every(Duration::from_millis(10)).map(|_| Msg::UpdateFrame)
    }
}
