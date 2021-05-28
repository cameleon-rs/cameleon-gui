use super::util;
use crate::Result;
use cameleon::{
    genapi::{node_kind::EnumerationNode, GenApiCtxt, ParamsCtxt},
    DeviceControl,
};
use cameleon_genapi::EnumEntryNode;
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
    fn new(raw: &EnumEntryNode) -> Self {
        Self {
            name: raw.name().to_string(),
            value: raw.value(),
        }
    }
}

impl fmt::Display for Entry {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.name)
    }
}

#[derive(Debug, Clone)]
pub enum Msg {
    Select(Entry),
}

impl Node {
    pub fn new(
        inner: EnumerationNode,
        ctx: &mut ParamsCtxt<impl DeviceControl, impl GenApiCtxt>,
    ) -> Self {
        Self {
            inner,
            name: inner.as_node().name(ctx).to_string(),
            state: pick_list::State::default(),
            entries: inner
                .entries(ctx)
                .iter()
                .map(|raw| Entry::new(raw))
                .collect(),
        }
    }

    pub fn view(
        &mut self,
        ctx: &mut ParamsCtxt<impl DeviceControl, impl GenApiCtxt>,
    ) -> Result<Element<Msg>> {
        let name = Text::new(&self.name).width(Length::FillPortion(1));
        let value: Element<_> = if self.inner.is_readable(ctx)? {
            let current = self.inner.current_entry(ctx)?;
            let current = Entry::new(current);
            if self.inner.is_writable(ctx)? {
                PickList::new(&mut self.state, &self.entries, Some(current), Msg::Select)
                    .width(Length::FillPortion(1))
                    .into()
            } else {
                Text::new(current.name).width(Length::FillPortion(1)).into()
            }
        } else {
            util::not_available().width(Length::FillPortion(1)).into()
        };
        Ok(Row::new().push(name).push(value).into())
    }

    pub fn update(
        &mut self,
        msg: Msg,
        ctx: &mut ParamsCtxt<impl DeviceControl, impl GenApiCtxt>,
    ) -> Result<()> {
        let Msg::Select(entry) = msg;
        if !self.inner.is_writable(ctx)? {
            return Ok(());
        }
        self.inner.set_entry_by_value(ctx, entry.value)?;
        Ok(())
    }
}
