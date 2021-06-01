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
    Opened(CameraId),
    Closed(CameraId),
    StreamingStarted(CameraId, PayloadReceiver),
    StreamingStopped(CameraId),
    None,
}

#[derive(Debug, Default)]
pub struct Control {
    open: button::State,
    close: button::State,
    start: button::State,
    stop: button::State,
}

impl Control {
    pub fn view(&mut self, ctx: &Context) -> Element<Msg> {
        let mut open = Button::new(&mut self.open, Text::new("Open"));
        let mut close = Button::new(&mut self.close, Text::new("Close"));
        let mut start = Button::new(&mut self.start, Text::new("Start"));
        let mut stop = Button::new(&mut self.stop, Text::new("Stop"));

        if let Some(selected) = ctx.selected() {
            if selected.is_opened(ctx) {
                close = close.on_press(Msg::Close);
                if selected.is_streaming(ctx) {
                    stop = stop.on_press(Msg::StopStreaming)
                } else {
                    start = start.on_press(Msg::StartStreaming)
                }
            } else {
                open = open.on_press(Msg::Open)
            }
        }

        let content = Row::new().push(open).push(close).push(start).push(stop);
        Container::new(content)
            .width(Length::Fill)
            .style(WithBorder)
            .into()
    }

    pub fn update(&mut self, msg: Msg, ctx: &mut Context) -> Result<OutMsg> {
        match msg {
            Msg::Open => ctx.selected().map_or(Ok(OutMsg::None), |cam| {
                cam.open(ctx)?;
                Ok(OutMsg::Opened(cam))
            }),
            Msg::Close => ctx.selected().map_or(Ok(OutMsg::None), |cam| {
                cam.close(ctx)?;
                Ok(OutMsg::Closed(cam))
            }),
            Msg::StartStreaming => ctx.selected().map_or(Ok(OutMsg::None), |cam| {
                let receiver = cam.start_streaming(ctx)?;
                Ok(OutMsg::StreamingStarted(cam, receiver))
            }),
            Msg::StopStreaming => ctx.selected().map_or(Ok(OutMsg::None), |cam| {
                cam.stop_streaming(ctx)?;
                Ok(OutMsg::StreamingStopped(cam))
            }),
        }
    }
}
