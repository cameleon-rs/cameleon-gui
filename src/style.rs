use iced::{container, Color};

pub struct WithBorder;

impl container::StyleSheet for WithBorder {
    fn style(&self) -> container::Style {
        container::Style {
            border_width: 0.5,
            border_color: Color::BLACK,
            ..Default::default()
        }
    }
}
