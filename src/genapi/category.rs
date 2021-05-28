use super::node;
use crate::Result;
use cameleon::{
    genapi::{node_kind::CategoryNode, GenApiCtxt, ParamsCtxt},
    DeviceControl,
};
use iced::{button, Button, Column, Element, Length, Row, Space, Text};
use tracing::trace;

pub struct Node {
    _inner: CategoryNode,
    name: String,
    expanded: bool,
    expand: button::State,
    features: Vec<node::Node>,
}

#[derive(Debug, Clone)]
pub enum Msg {
    Expand,
    Node(usize, Box<node::Msg>),
}

const SPACE_OFFSET: u16 = 0;

impl Node {
    pub fn new(
        inner: CategoryNode,
        ctx: &mut ParamsCtxt<impl DeviceControl, impl GenApiCtxt>,
    ) -> Self {
        let name = inner.as_node().name(ctx).to_string();
        let nodes = inner.nodes(ctx);
        let nodes: Vec<_> = nodes
            .into_iter()
            .filter(|node| !node.name(ctx).starts_with("Chunk"))
            .collect();
        Self {
            _inner: inner,
            name,
            expanded: false,
            features: nodes
                .into_iter()
                .filter_map(|node| node::Node::new(node, ctx))
                .collect(),
            expand: button::State::new(),
        }
    }

    pub fn view(
        &mut self,
        ctx: &mut ParamsCtxt<impl DeviceControl, impl GenApiCtxt>,
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
                        match feature.view(ctx) {
                            Ok(elm) => column.push(elm.map(move |msg| Msg::Node(i, Box::new(msg)))),
                            Err(err) => {
                                trace!("{}", err);
                                column
                            }
                        }
                    });
            column = column.push(
                Row::new()
                    .push(Space::new(Length::Units(10), Length::Shrink))
                    .push(features),
            );
        }
        Row::new()
            .push(Space::new(Length::Units(SPACE_OFFSET), Length::Shrink))
            .push(column)
            .into()
    }

    #[tracing::instrument(skip(self, ctx), level = "trace")]
    pub fn update(
        &mut self,
        msg: Msg,
        ctx: &mut ParamsCtxt<impl DeviceControl, impl GenApiCtxt>,
    ) -> Result<()> {
        match msg {
            Msg::Expand => {
                self.expanded = !self.expanded;
                trace!(self.expanded);
                trace!("num of features: {}", self.features.len());
                Ok(())
            }
            Msg::Node(i, msg) => self.features[i].update(*msg, ctx),
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
