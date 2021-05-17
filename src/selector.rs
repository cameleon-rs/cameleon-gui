use std::collections::BTreeMap;

use iced::{button, scrollable, Button, Column, Container, Element, Length, Scrollable, Text};

use super::style::WithBorder;
use super::{camera::CameraId, Msg};

#[derive(Debug, Default)]
pub struct Selector {
    pub options: BTreeMap<CameraId, (String, button::State)>,
    scrollable: scrollable::State,
    refresh: button::State,
}

impl Selector {
    pub fn view(&mut self, selected: Option<CameraId>) -> Element<Msg> {
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
        let refresh = Button::new(&mut self.refresh, Text::new("Refresh")).on_press(Msg::Refresh);

        let content = Column::new().push(options).push(refresh);
        Container::new(content).style(WithBorder).into()
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
