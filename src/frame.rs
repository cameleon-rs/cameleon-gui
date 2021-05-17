use anyhow::Result;
use cameleon::payload::PayloadReceiver;
use iced::{
    image::{viewer, Handle, Image, Viewer},
    Container, Element,
};

use super::convert::convert;
use super::Msg;

#[derive(Debug, Default)]
pub struct Frame {
    receiver: Option<PayloadReceiver>,
    handle: Option<Handle>,
    viewer: viewer::State,
}

impl Frame {
    pub fn view(&mut self) -> Element<Msg> {
        let content: Element<_> = if let Some(ref handle) = self.handle {
            Viewer::new(&mut self.viewer, handle.clone()).into()
        } else {
            Image::new("ferris.png").into()
        };
        Container::new(content).into()
    }

    pub fn update(&mut self) -> Result<()> {
        if let Some(receiver) = &self.receiver {
            let payload = receiver.try_recv()?;
            let image = convert(&payload)?;
            receiver.send_back(payload);
            self.handle = Some(image);
        }
        Ok(())
    }

    pub fn attach(&mut self, receiver: PayloadReceiver) {
        self.receiver = Some(receiver)
    }

    pub fn detach(&mut self) {
        self.receiver = None
    }
}
