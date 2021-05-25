use super::util;
use cameleon::{
    genapi::{node_kind::EnumerationNode, GenApiCtxt, ParamsCtxt},
    DeviceControl,
};
use cameleon_genapi::{EnumEntryNode, NodeStore};
use iced::{pick_list, Element, Length, PickList, Row, Text};
use std::fmt;

pub struct Node {
    inner: EnumerationNode,
    name: String,
    state: pick_list::State<Entry>,
    entries: Vec<Entry>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Entry {
    name: String,
    value: i64,
}

impl Entry {
    pub fn new(node: &EnumEntryNode, cx: &ParamsCtxt<impl DeviceControl, impl GenApiCtxt>) -> Self {
        let base = node.node_base();
        let name = base
            .display_name()
            .unwrap_or_else(|| cx.ctxt.node_store().name_by_id(base.id()).unwrap())
            .to_string();
        let value = node.value();
        Self { name, value }
    }
}

impl fmt::Display for Entry {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.name)
    }
}

#[derive(Debug, Clone)]
pub enum Msg {
    Selected(Entry),
    Ignore(Entry),
}

impl Node {
    pub fn new(
        inner: EnumerationNode,
        cx: &mut ParamsCtxt<impl DeviceControl, impl GenApiCtxt>,
    ) -> Self {
        Self {
            inner,
            name: inner.as_node().name(cx).to_string(),
            state: pick_list::State::default(),
            entries: inner
                .entries(cx)
                .iter()
                .map(|raw| Entry::new(raw, cx))
                .collect(),
        }
    }

    pub fn view(
        &mut self,
        cx: &mut ParamsCtxt<impl DeviceControl, impl GenApiCtxt>,
    ) -> Element<Msg> {
        let name = Text::new(&self.name).width(Length::FillPortion(1));
        let value: Element<_> = if self.inner.is_readable(cx).unwrap() {
            let current = self.inner.current_entry(cx).unwrap();
            let value = current.value();
            let current = self.inner.current_entry(cx).unwrap().node_base().id();
            let ns = cx.node_store();
            let name = ns.name_by_id(current).unwrap();
            let node = current.as_inode_kind(ns).unwrap().node_base_precise();
            let display_name = node.display_name();
            let name = display_name.unwrap_or(name).to_string();
            let current = Entry { name, value };
            if self.inner.is_writable(cx).unwrap() {
                PickList::new(&mut self.state, &self.entries, Some(current), Msg::Selected)
                    .width(Length::FillPortion(1))
                    .into()
            } else {
                PickList::new(&mut self.state, &self.entries, Some(current), Msg::Ignore)
                    .width(Length::FillPortion(1))
                    .into()
            }
        } else {
            util::not_available().width(Length::FillPortion(1)).into()
        };
        Row::new().push(name).push(value).into()
    }

    pub fn update(&mut self, msg: Msg, cx: &mut ParamsCtxt<impl DeviceControl, impl GenApiCtxt>) {
        match msg {
            Msg::Selected(entry) => {
                if !self.inner.is_writable(cx).unwrap() {
                    return;
                }
                self.inner.set_entry_by_value(cx, entry.value).unwrap();
            }
            Msg::Ignore(_) => (),
        }
    }
}
