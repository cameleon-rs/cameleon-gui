use cameleon::payload::{Payload, PayloadReceiver};
use iced::{
    image::{viewer, Handle, Image, Viewer},
    Command, Container, Element, Subscription,
};
use iced_futures::BoxStream;
use std::hash::{Hash, Hasher};

use super::{context::Context, convert::convert, Result};

#[derive(Debug)]
pub enum Msg {
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

struct Receiver {
    inner: PayloadReceiver,
}

impl<H: Hasher, E> iced_futures::subscription::Recipe<H, E> for Receiver {
    type Output = Payload;
    fn hash(&self, state: &mut H) {
        struct Marker;
        std::any::TypeId::of::<Marker>().hash(state);
    }

    fn stream(self: Box<Self>, _input: BoxStream<E>) -> BoxStream<Self::Output> {
        let receiver = self.inner.clone();
        Box::pin(futures::stream::unfold(receiver, move |r| async move {
            match r.recv().await {
                Ok(p) => Some((p, r)),
                Err(e) => {
                    tracing::trace!("{}", e);
                    None
                }
            }
        }))
    }
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
            Msg::Acquired(payload) => {
                tracing::info!("Acquired!!!");
                self.handle = Some(convert(&payload)?);
                if let Some(receiver) = &mut self.receiver {
                    receiver.send_back(payload)
                }
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
        if let Some(receiver) = &self.receiver {
            Subscription::from_recipe(Receiver {
                inner: receiver.clone(),
            })
            .map(Msg::Acquired)
        } else {
            Subscription::none()
        }
    }
}
