use cameleon::payload::PayloadReceiver;
use iced::{button, Button, Container, Element, Length, Row, Text};

use super::{context::CameraId, context::Context, style::WithBorder, Result};

#[derive(Debug, Clone)]
pub enum Msg {
    Open,
    Close,
    StartStreaming,
    StopStreaming,
}

#[derive(Debug)]
pub enum OutMsg {
    Open(CameraId),
    Close(CameraId),
    StartStreaming(CameraId, PayloadReceiver),
    StopStreaming(CameraId),
    None,
}

#[derive(Debug, Default)]
pub struct Control {
    toggle: button::State,
    start: button::State,
    stop: button::State,
}

impl Control {
    pub fn view(&mut self, ctx: &Context) -> Element<Msg> {
        let selected = ctx.selected();
        let toggle = if let Some(selected) = selected {
            if selected.is_opened(ctx) {
                Button::new(&mut self.toggle, Text::new("Close")).on_press(Msg::Close)
            } else {
                Button::new(&mut self.toggle, Text::new("Open")).on_press(Msg::Open)
            }
        } else {
            Button::new(&mut self.toggle, Text::new("Open"))
        };
        let mut start = Button::new(&mut self.start, Text::new("Start"));
        let mut stop = Button::new(&mut self.stop, Text::new("Stop"));

        if let Some(selected) = selected {
            if selected.is_opened(ctx) {
                if selected.is_streaming(ctx) {
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

    pub fn update(&mut self, msg: Msg, ctx: &mut Context) -> Result<OutMsg> {
        match msg {
            Msg::Open => {
                if let Some(cam) = ctx.selected() {
                    cam.open(ctx)?;
                    Ok(OutMsg::Open(cam))
                } else {
                    Ok(OutMsg::None)
                }
            }
            Msg::Close => {
                if let Some(cam) = ctx.selected() {
                    cam.close(ctx)?;
                    Ok(OutMsg::Close(cam))
                } else {
                    Ok(OutMsg::None)
                }
            }
            Msg::StartStreaming => {
                if let Some(cam) = ctx.selected() {
                    let recevier = cam.start_streaming(ctx)?;
                    Ok(OutMsg::StartStreaming(cam, recevier))
                } else {
                    Ok(OutMsg::None)
                }
            }
            Msg::StopStreaming => {
                if let Some(cam) = ctx.selected() {
                    cam.stop_streaming(ctx)?;
                    Ok(OutMsg::StopStreaming(cam))
                } else {
                    Ok(OutMsg::None)
                }
            }
        }
    }
}
