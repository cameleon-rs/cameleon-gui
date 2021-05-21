use cameleon::{payload::PayloadReceiver, DeviceControl, PayloadStream};
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
    Open(CameraId),
    Close(CameraId),
    StartStreaming(CameraId, PayloadReceiver),
    StopStreaming(CameraId),
}

#[derive(Debug, Default)]
pub struct Control {
    toggle: button::State,
    start: button::State,
    stop: button::State,
}

impl Control {
    pub fn view(&mut self, ctx: &Context) -> Element<Msg> {
        let cam_with_id = ctx.selected();
        let toggle = if let Some((cam, _)) = cam_with_id {
            if cam.ctrl.is_opened() {
                Button::new(&mut self.toggle, Text::new("Close")).on_press(Msg::Close)
            } else {
                Button::new(&mut self.toggle, Text::new("Open")).on_press(Msg::Open)
            }
        } else {
            Button::new(&mut self.toggle, Text::new("Open"))
        };
        let mut start = Button::new(&mut self.start, Text::new("Start"));
        let mut stop = Button::new(&mut self.stop, Text::new("Stop"));

        if let Some((cam, _)) = cam_with_id {
            if cam.ctrl.is_opened() {
                if cam.strm.is_loop_running() {
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
                if let Some((cam, id)) = ctx.selected_mut() {
                    cam.open()?;
                    cam.load_context()?;
                    Ok(Some(OutMsg::Open(id)))
                } else {
                    Ok(None)
                }
            }
            Msg::Close => {
                if let Some((cam, id)) = ctx.selected_mut() {
                    cam.close()?;
                    Ok(Some(OutMsg::Close(id)))
                } else {
                    Ok(None)
                }
            }
            Msg::StartStreaming => {
                if let Some((cam, id)) = ctx.selected_mut() {
                    let receiver = cam.start_streaming(1)?;
                    Ok(Some(OutMsg::StartStreaming(id, receiver)))
                } else {
                    Ok(None)
                }
            }
            Msg::StopStreaming => {
                if let Some((cam, id)) = ctx.selected_mut() {
                    cam.stop_streaming()?;
                    Ok(Some(OutMsg::StopStreaming(id)))
                } else {
                    Ok(None)
                }
            }
        }
    }
}
