use std::collections::{BTreeMap, HashSet};

use iced::{button, scrollable, Button, Container, Element, Length, Scrollable, Text};

use super::{
    context::{CameraId, Context},
    style::WithBorder,
    Result,
};

#[derive(Debug, Clone)]
pub enum Msg {
    Select(CameraId),
    Detected(Vec<CameraId>),
}

#[derive(Debug, Default)]
pub struct Selector {
    options: BTreeMap<CameraId, (String, button::State)>,
    scrollable: scrollable::State,
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
        Container::new(options).style(WithBorder).into()
    }

    pub fn update(&mut self, msg: Msg, ctx: &mut Context) -> Result<()> {
        match msg {
            Msg::Select(id) => ctx.select(id),
            Msg::Detected(new_ids) => {
                let old_ids: HashSet<CameraId> = self.options.keys().copied().collect();
                let new_ids: HashSet<CameraId> = new_ids.into_iter().collect();
                for dissappered in old_ids.difference(&new_ids) {
                    self.options.remove(dissappered);
                }
                for newly_added in new_ids.difference(&old_ids) {
                    self.options
                        .insert(*newly_added, (newly_added.name(ctx), button::State::new()));
                }
                Ok(())
            }
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
