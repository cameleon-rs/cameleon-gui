use cameleon::payload::PayloadReceiver;
use iced::{button, Button, Container, Element, Length, Row, Text};

use super::{camera::CameraId, context::Context, style::WithBorder, Result};

#[derive(Debug, Clone)]
pub enum Msg {
    Open,
    Close,
    StartStreaming,
    StopStreaming,
}

#[derive(Debug)]
pub enum OutMsg {
    Opened(CameraId),
    Closed(CameraId),
    Started(CameraId, PayloadReceiver),
    Stopped(CameraId),
}

#[derive(Debug, Default)]
pub struct Control {
    toggle: button::State,
    start: button::State,
    stop: button::State,
}

impl Control {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn view<'a>(&'a mut self, ctx: &Context) -> Element<'a, Msg> {
        let state = ctx.selected_state();
        let toggle = if let Some(state) = state {
            if state.is_open() {
                Button::new(&mut self.toggle, Text::new("Close")).on_press(Msg::Close)
            } else {
                Button::new(&mut self.toggle, Text::new("Open")).on_press(Msg::Open)
            }
        } else {
            Button::new(&mut self.toggle, Text::new("Open"))
        };
        let mut start = Button::new(&mut self.start, Text::new("Start"));
        let mut stop = Button::new(&mut self.stop, Text::new("Stop"));

        if let Some(state) = state {
            if state.is_open() {
                if state.is_streaming() {
                    stop = stop.on_press(Msg::StopStreaming);
                } else {
                    start = start.on_press(Msg::StartStreaming)
                }
            }
        }

        let content = Row::new().push(toggle).push(start).push(stop);
        Container::new(content)
            .width(Length::Fill)
            .style(WithBorder)
            .into()
    }

    pub fn update(&mut self, msg: Msg, ctx: &mut Context) -> Result<Option<OutMsg>> {
        match msg {
            Msg::Open => {
                if let Some(cam) = ctx.selected_mut() {
                    cam.raw.open()?;
                    cam.raw.load_context()?;
                    Ok(Some(OutMsg::Opened(cam.id)))
                } else {
                    Ok(None)
                }
            }
            Msg::Close => {
                if let Some(cam) = ctx.selected_mut() {
                    cam.raw.close()?;
                    Ok(Some(OutMsg::Closed(cam.id)))
                } else {
                    Ok(None)
                }
            }
            Msg::StartStreaming => {
                if let Some(cam) = ctx.selected_mut() {
                    let receiver = cam.raw.start_streaming(1)?;
                    Ok(Some(OutMsg::Started(cam.id, receiver)))
                } else {
                    Ok(None)
                }
            }
            Msg::StopStreaming => {
                if let Some(cam) = ctx.selected_mut() {
                    cam.raw.stop_streaming()?;
                    Ok(Some(OutMsg::Stopped(cam.id)))
                } else {
                    Ok(None)
                }
            }
        }
    }
}
