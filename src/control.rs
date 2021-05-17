use iced::{button, Button, Container, Element, Length, Row, Text};

use super::{camera::State, style::WithBorder, Msg};

#[derive(Debug, Default)]
pub struct Control {
    toggle: button::State,
    start: button::State,
    stop: button::State,
}

impl Control {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn view(&mut self, state: Option<State>) -> Element<Msg> {
        let toggle = if let Some(state) = state {
            if state.is_open() {
                Button::new(&mut self.toggle, Text::new("Close")).on_press(Msg::Close)
            } else {
                Button::new(&mut self.toggle, Text::new("Open")).on_press(Msg::Open)
            }
        } else {
            Button::new(&mut self.toggle, Text::new("Open"))
        };
        let mut start = Button::new(&mut self.start, Text::new("Start"));
        let mut stop = Button::new(&mut self.stop, Text::new("Stop"));

        if let Some(state) = state {
            if state.is_open() {
                if state.is_streaming() {
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
}
