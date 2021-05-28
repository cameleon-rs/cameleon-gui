use std::{collections::BTreeMap, time::Duration};

use iced::{
    button, scrollable, time, Button, Checkbox, Column, Container, Element, Length, Row,
    Scrollable, Space, Subscription, Text,
};

use super::style::WithBorder;
use super::{
    context::{CameraId, Context},
    Result,
};

#[derive(Debug, Clone)]
pub enum Msg {
    Select(CameraId),
    EnableAutoRefresh(bool),
    Refresh,
    Add(CameraId),
    Remove(CameraId),
}

pub enum OutMsg {
    UpdateContext,
    None,
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

    pub fn update(&mut self, msg: Msg, ctx: &mut Context) -> Result<OutMsg> {
        match msg {
            Msg::Select(id) => {
                ctx.select(id)?;
                Ok(OutMsg::None)
            }
            Msg::Refresh => Ok(OutMsg::UpdateContext),
            Msg::EnableAutoRefresh(auto_refresh) => {
                self.auto_refresh = auto_refresh;
                Ok(OutMsg::None)
            }
            Msg::Add(id) => {
                self.options
                    .entry(id)
                    .or_insert_with_key(|id| (id.name(ctx), button::State::new()));
                Ok(OutMsg::None)
            }
            Msg::Remove(id) => {
                self.options.remove(&id);
                Ok(OutMsg::None)
            }
        }
    }

    pub fn subscription(&self) -> Subscription<Msg> {
        if self.auto_refresh {
            time::every(Duration::from_secs(1)).map(|_| Msg::Refresh)
        } else {
            Subscription::none()
        }
    }
}

impl CameraId {
    fn name(self, ctx: &Context) -> String {
        let cam = &ctx[self];
        let info = cam.info();
        let name = match cam.ctrl.user_defined_name() {
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
