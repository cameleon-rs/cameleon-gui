use std::{collections::BTreeMap, time::Duration};

use iced::{
    button, scrollable, time, Button, Checkbox, Column, Command, Container, Element, Length, Row,
    Scrollable, Subscription, Text,
};

use super::style::WithBorder;
use super::{camera::CameraId, context::Context};

#[derive(Debug, Clone)]
pub enum Msg {
    Selected(CameraId),
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
    pub fn view<'a>(&'a mut self, ctx: &Context) -> Element<'a, Msg> {
        let selected = ctx.selected;
        let options = self.options.iter_mut().fold(
            Scrollable::new(&mut self.scrollable),
            |scrollable, (id, (name, state))| {
                scrollable.push(
                    Button::new(state, Text::new(name.clone()))
                        .width(Length::Fill)
                        .style(style::Button::new(selected, *id))
                        .on_press(Msg::Selected(*id)),
                )
            },
        );
        let auto_refresh = Checkbox::new(self.auto_refresh, "Auto Refresh", Msg::EnableAutoRefresh);
        let refresh = Button::new(&mut self.refresh, Text::new("Refresh")).on_press(Msg::Refresh);
        let row = Row::new().push(auto_refresh).push(refresh);

        let content = Column::new().push(options).push(row);
        Container::new(content).style(WithBorder).into()
    }

    pub fn update(&mut self, msg: Msg, ctx: &mut Context) -> Command<Msg> {
        match msg {
            Msg::Selected(id) => ctx.selected = Some(id),
            Msg::Refresh => {
                ctx.refresh();
                self.options = ctx
                    .cameras
                    .iter()
                    .map(|(id, cam)| (*id, (cam.name.clone(), button::State::new())))
                    .collect();
            }
            Msg::EnableAutoRefresh(auto_refresh) => self.auto_refresh = auto_refresh,
        }
        Command::none()
    }

    pub fn subscription(&self) -> Subscription<Msg> {
        if self.auto_refresh {
            time::every(Duration::from_millis(100)).map(|_| Msg::Refresh)
        } else {
            Subscription::none()
        }
    }
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
