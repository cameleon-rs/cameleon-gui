use std::collections::HashSet;

use super::{
    camera::enumerate_cameras,
    context::{CameraId, Context},
    style::WithBorder,
    Result,
};
use iced::{
    button, time, Button, Checkbox, Container, Element, Length, Row, Space, Subscription, Text,
};

#[derive(Debug, Clone)]
pub enum Msg {
    Scan,
    EnableAutoScan(bool),
}

#[derive(Debug)]
pub enum OutMsg {
    CameraListRefreshed,
    None,
}

#[derive(Debug, Default)]
pub struct Scanner {
    scan: button::State,
    auto_scan: bool,
}

impl Scanner {
    pub fn view(&mut self, _ctx: &mut Context) -> Element<Msg> {
        let auto_scan = Checkbox::new(self.auto_scan, "Auto Scan", Msg::EnableAutoScan);
        let scan = Button::new(&mut self.scan, Text::new("Scan")).on_press(Msg::Scan);
        let content = Row::new()
            .push(auto_scan)
            .push(Space::new(Length::Fill, Length::Shrink))
            .push(scan);
        Container::new(content).style(WithBorder).into()
    }

    pub fn update(&mut self, msg: Msg, ctx: &mut Context) -> Result<OutMsg> {
        match msg {
            Msg::Scan => {
                self.scan(ctx)?;
                Ok(OutMsg::CameraListRefreshed)
            }
            Msg::EnableAutoScan(v) => {
                self.auto_scan = v;
                Ok(OutMsg::None)
            }
        }
    }

    pub fn subscription(&self) -> Subscription<Msg> {
        if self.auto_scan {
            time::every(std::time::Duration::from_secs(1)).map(|_| Msg::Scan)
        } else {
            Subscription::none()
        }
    }

    fn scan(&self, ctx: &mut Context) -> Result<()> {
        let old_ids: HashSet<CameraId> = ctx.ids().copied().collect();
        let cameras = enumerate_cameras()?;
        let added: HashSet<CameraId> = cameras.into_iter().map(|cam| ctx.add(cam)).collect();
        for dissappered in old_ids.difference(&added) {
            ctx.remove(*dissappered)
        }
        Ok(())
    }
}
