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
        node: cameleon::genapi::node_kind::Node,
        ctxt: &mut ParamsCtxt<T, U>,
    ) -> Option<Self> {
        if let Some(node) = node.as_boolean(ctxt) {
            Some(Node::Boolean(boolean::Node::new(node, ctxt)))
        } else if let Some(node) = node.as_integer(ctxt) {
            Some(Node::Integer(integer::Node::new(node, ctxt)))
        } else if let Some(node) = node.as_float(ctxt) {
            Some(Node::Float(float::Node::new(node, ctxt)))
        } else if let Some(node) = node.as_enumeration(ctxt) {
            Some(Node::Enumeration(enumeration::Node::new(node, ctxt)))
        } else if let Some(node) = node.as_command(ctxt) {
            Some(Node::Command(command::Node::new(node, ctxt)))
        } else if let Some(node) = node.as_string(ctxt) {
            Some(Node::String(string::Node::new(node, ctxt)))
        } else if let Some(node) = node.as_category(ctxt) {
            Some(Node::Category(category::Node::new(node, ctxt)))
        } else {
            trace!("Ignore {} node", node.name(ctxt));
            None
        }
    }

    pub fn view(
        &mut self,
        cx: &mut ParamsCtxt<impl DeviceControl, impl GenApiCtxt>,
    ) -> Result<Element<Msg>> {
        let content = match self {
            Node::Integer(node) => node.view(cx)?.map(Into::into),
            Node::Float(node) => node.view(cx)?.map(Into::into),
            Node::Enumeration(node) => node.view(cx)?.map(Into::into),
            Node::Boolean(node) => node.view(cx)?.map(Into::into),
            Node::Command(node) => node.view(cx)?.map(Into::into),
            Node::String(node) => node.view(cx)?.map(Into::into),
            Node::Category(node) => node.view(cx).map(Into::into),
        };
        Ok(Container::new(content)
            .width(Length::Fill)
            .style(WithBorder)
            .into())
    }

    pub fn update(
        &mut self,
        msg: Msg,
        cx: &mut ParamsCtxt<impl DeviceControl, impl GenApiCtxt>,
    ) -> Result<()> {
        match (self, msg) {
            (Node::Enumeration(node), Msg::Enumeration(msg)) => node.update(msg, cx)?,
            (Node::Boolean(node), Msg::Bool(msg)) => node.update(msg, cx)?,
            (Node::Float(node), Msg::Float(msg)) => node.update(msg, cx)?,
            (Node::Integer(node), Msg::Integer(msg)) => node.update(msg, cx)?,
            (Node::Command(node), Msg::Command(msg)) => node.update(msg, cx)?,
            (Node::String(node), Msg::String(msg)) => node.update(msg, cx)?,
            (Node::Category(node), Msg::Category(msg)) => node.update(msg, cx)?,
            _ => (),
        }
        Ok(())
    }
}
