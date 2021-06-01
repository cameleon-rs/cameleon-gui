use cameleon::{
    genapi::{GenApiCtxt, ParamsCtxt},
    DeviceControl,
};
use derive_more::From;
use iced::{Container, Element, Length};
use tracing::trace;

use super::{boolean, category, command, enumeration, float, integer, string};
use crate::{style::WithBorder, Result};

#[derive(Debug, Clone, From)]
pub enum Msg {
    Enumeration(enumeration::Msg),
    Bool(boolean::Msg),
    Float(float::Msg),
    Integer(integer::Msg),
    Command(command::Msg),
    String(string::Msg),
    Category(category::Msg),
}

pub enum Node {
    Boolean(boolean::Node),
    Integer(integer::Node),
    Float(float::Node),
    Enumeration(enumeration::Node),
    Command(command::Node),
    String(string::Node),
    Category(category::Node),
}

impl Node {
    pub fn new<T: DeviceControl, U: GenApiCtxt>(
        node: cameleon::genapi::Node,
        ctx: &mut ParamsCtxt<T, U>,
    ) -> Option<Self> {
        if let Some(node) = node.as_boolean(ctx) {
            Some(Node::Boolean(boolean::Node::new(node, ctx)))
        } else if let Some(node) = node.as_integer(ctx) {
            Some(Node::Integer(integer::Node::new(node, ctx)))
        } else if let Some(node) = node.as_float(ctx) {
            Some(Node::Float(float::Node::new(node, ctx)))
        } else if let Some(node) = node.as_enumeration(ctx) {
            Some(Node::Enumeration(enumeration::Node::new(node, ctx)))
        } else if let Some(node) = node.as_command(ctx) {
            Some(Node::Command(command::Node::new(node, ctx)))
        } else if let Some(node) = node.as_string(ctx) {
            Some(Node::String(string::Node::new(node, ctx)))
        } else if let Some(node) = node.as_category(ctx) {
            Some(Node::Category(category::Node::new(node, ctx)))
        } else {
            trace!("Ignore {} node", node.name(ctx));
            None
        }
    }

    pub fn view(
        &mut self,
        ctx: &mut ParamsCtxt<impl DeviceControl, impl GenApiCtxt>,
    ) -> Result<Element<Msg>> {
        let content = match self {
            Node::Integer(node) => node.view(ctx)?.map(Into::into),
            Node::Float(node) => node.view(ctx)?.map(Into::into),
            Node::Enumeration(node) => node.view(ctx)?.map(Into::into),
            Node::Boolean(node) => node.view(ctx)?.map(Into::into),
            Node::Command(node) => node.view(ctx)?.map(Into::into),
            Node::String(node) => node.view(ctx)?.map(Into::into),
            Node::Category(node) => node.view(ctx).map(Into::into),
        };
        Ok(Container::new(content)
            .width(Length::Fill)
            .style(WithBorder)
            .into())
    }

    pub fn update(
        &mut self,
        msg: Msg,
        ctx: &mut ParamsCtxt<impl DeviceControl, impl GenApiCtxt>,
    ) -> Result<()> {
        match (self, msg) {
            (Node::Enumeration(node), Msg::Enumeration(msg)) => node.update(msg, ctx)?,
            (Node::Boolean(node), Msg::Bool(msg)) => node.update(msg, ctx)?,
            (Node::Float(node), Msg::Float(msg)) => node.update(msg, ctx)?,
            (Node::Integer(node), Msg::Integer(msg)) => node.update(msg, ctx)?,
            (Node::Command(node), Msg::Command(msg)) => node.update(msg, ctx)?,
            (Node::String(node), Msg::String(msg)) => node.update(msg, ctx)?,
            (Node::Category(node), Msg::Category(msg)) => node.update(msg, ctx)?,
            _ => (),
        }
        Ok(())
    }
}
