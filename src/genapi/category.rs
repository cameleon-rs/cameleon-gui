use super::node;
use cameleon::{
    genapi::{node_kind::CategoryNode, GenApiCtxt, ParamsCtxt},
    DeviceControl,
};
use iced::{button, Button, Column, Element, Length, Row, Space, Text};

pub struct Node {
    inner: CategoryNode,
    name: String,
    expanded: bool,
    expand: button::State,
    features: Vec<node::Node>,
}

#[derive(Debug, Clone)]
pub enum Msg {
    Expand,
    Node(usize, node::Msg),
}

const SPACE_OFFSET: u16 = 0;

impl Node {
    pub fn new(
        inner: CategoryNode,
        cx: &mut ParamsCtxt<impl DeviceControl, impl GenApiCtxt>,
    ) -> Self {
        let name = inner.as_node().name(cx).to_string();
        Self {
            inner,
            name,
            expanded: false,
            features: inner
                .nodes(cx)
                .into_iter()
                .filter(|node| node.name(cx).starts_with("Chunk")) // Ignore Chunk related nodes
                .map(|node| node::Node::new(node, cx))
                .collect(),
            expand: button::State::new(),
        }
    }

    pub fn view(
        &mut self,
        cx: &mut ParamsCtxt<impl DeviceControl, impl GenApiCtxt>,
    ) -> Element<Msg> {
        let mut column = Column::new().push(
            Button::new(&mut self.expand, Text::new(&self.name))
                .width(Length::Fill)
                .on_press(Msg::Expand)
                .style(style::Category),
        );
        if self.expanded {
            let features =
                self.features
                    .iter_mut()
                    .enumerate()
                    .fold(Column::new(), |column, (i, feature)| {
                        column.push(feature.view(cx).map(move |msg| Msg::Node(i, msg)))
                    });
            column = column.push(
                Row::new()
                    .push(Space::new(Length::Units(20), Length::Shrink))
                    .push(features),
            );
        }
        Row::new()
            .push(Space::new(Length::Units(SPACE_OFFSET), Length::Shrink))
            .push(column)
            .into()
    }

    pub fn update(&mut self, msg: Msg, cx: &mut ParamsCtxt<impl DeviceControl, impl GenApiCtxt>) {
        match msg {
            Msg::Expand => self.expanded = !self.expanded,
            Msg::Node(i, msg) => self.features[i].update(msg, cx),
        }
    }
}

mod style {
    use iced::button::{Style, StyleSheet};

    pub struct Category;

    impl StyleSheet for Category {
        fn active(&self) -> Style {
            Style {
                border_radius: 0.,
                background: None,
                ..Default::default()
            }
        }
    }
}
