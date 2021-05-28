use std::collections::HashSet;
use std::{collections::BTreeMap, time::Duration};

use iced::{
    button, scrollable, time, Button, Checkbox, Column, Command, Container, Element, Length, Row,
    Scrollable, Space, Subscription, Text,
};

use super::style::WithBorder;
use super::{
    camera::{enumerate_cameras, Camera},
    context::{CameraId, Context},
    Result,
};

#[derive(Debug, Clone)]
pub enum Msg {
    Select(CameraId),
    EnableAutoRefresh(bool),
    Refresh,
}

#[derive(Debug, Default)]
pub struct Selector {
    options: BTreeMap<CameraId, (String, button::State)>,
    scrollable: scrollable::State,
    refresh: button::State,
    auto_refresh: bool,
}

impl Selector {
    pub fn view(&mut self, ctx: &Context) -> Element<Msg> {
        let options = self.options.iter_mut().fold(
            Scrollable::new(&mut self.scrollable).height(Length::Units(300)),
            |scrollable, (id, (name, state))| {
                scrollable.push(
                    Button::new(state, Text::new(name.clone()))
                        .width(Length::Fill)
                        .style(style::Button::new(ctx.selected(), *id))
                        .on_press(Msg::Select(*id)),
                )
            },
        );
        let auto_refresh = Checkbox::new(self.auto_refresh, "Auto Refresh", Msg::EnableAutoRefresh);
        let refresh = Button::new(&mut self.refresh, Text::new("Refresh")).on_press(Msg::Refresh);
        let buttons = Container::new(
            Row::new()
                .push(auto_refresh)
                .push(Space::new(Length::Fill, Length::Shrink))
                .push(refresh),
        )
        .style(WithBorder);

        let content = Column::new().push(options).push(buttons);
        Container::new(content).style(WithBorder).into()
    }

    pub fn update(&mut self, msg: Msg, ctx: &mut Context) -> Result<Command<Msg>> {
        match msg {
            Msg::Select(id) => ctx.select(id)?,
            Msg::Refresh => self.refresh(ctx)?,
            Msg::EnableAutoRefresh(auto_refresh) => self.auto_refresh = auto_refresh,
        }
        Ok(Command::none())
    }

    pub fn subscription(&self) -> Subscription<Msg> {
        if self.auto_refresh {
            time::every(Duration::from_secs(1)).map(|_| Msg::Refresh)
        } else {
            Subscription::none()
        }
    }

    fn refresh(&mut self, ctx: &mut Context) -> Result<()> {
        let cameras = enumerate_cameras()?;
        let det_ids: HashSet<_> = cameras.into_iter().map(|cam| ctx.add(cam)).collect();
        let ids: HashSet<_> = ctx.cameras().copied().collect();
        for disappered in ids.difference(&det_ids) {
            ctx.remove(*disappered)?;
        }
        self.options = ctx
            .cameras()
            .map(|id| {
                ctx.with_camera_or_else(
                    *id,
                    || unreachable!(),
                    |cam| (*id, (name(cam), button::State::new())),
                )
            })
            .collect();

        if ctx.selected().is_none() {
            if let Some(id) = self.options.keys().next() {
                ctx.select(*id)?;
            }
        }

        Ok(())
    }
}

fn name(camera: &Camera) -> String {
    let info = camera.info();
    let name = match camera.ctrl.user_defined_name() {
        Some(name) => {
            if !name.is_empty() {
                name
            } else {
                &info.model_name
            }
        }
        None => &info.model_name,
    };
    format!("{} ({})", name, info.serial_number)
}

mod style {
    use super::CameraId;
    use iced::{button, Background, Color};

    pub struct Button {
        is_selected: bool,
    }

    impl Button {
        pub fn new(selected: Option<CameraId>, id: CameraId) -> Self {
            let is_selected = if let Some(selected) = selected {
                selected == id
            } else {
                false
            };
            Self { is_selected }
        }
    }

    impl button::StyleSheet for Button {
        fn active(&self) -> button::Style {
            let bg = if self.is_selected {
                Color::from_rgb8(41, 139, 200)
            } else {
                Color::WHITE
            };
            button::Style {
                background: Some(Background::Color(bg)),
                ..Default::default()
            }
        }
    }
}
