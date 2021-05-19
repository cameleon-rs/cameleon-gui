use cameleon::{
    genapi::{GenApiCtxt, ParamsCtxt},
    DeviceControl,
};
use derive_more::From;
use iced::{scrollable, Element, Scrollable};

mod boolean;
mod category;
mod command;
mod enumeration;
mod float;
mod integer;
mod node;
mod string;
mod util;

#[derive(Debug, From)]
pub enum Msg {
    Category(usize, category::Msg),
}

pub struct GenApi {
    categories: Vec<category::Node>,
    scrollable: scrollable::State,
}

impl GenApi {
    pub fn new<T: DeviceControl, U: GenApiCtxt>(ctxt: &mut ParamsCtxt<T, U>) -> Self {
        let root = ctxt.node("Root").unwrap().as_category(ctxt).unwrap();
        let categories = root
            .nodes(ctxt)
            .into_iter()
            .map(|node| category::Node::new(node.as_category(ctxt).unwrap(), ctxt))
            .collect();
        Self {
            categories,
            scrollable: scrollable::State::new(),
        }
    }

    pub fn view<'a, T: DeviceControl, U: GenApiCtxt>(
        &'a mut self,
        ctx: &mut ParamsCtxt<T, U>,
    ) -> Element<'a, Msg> {
        self.categories
            .iter_mut()
            .enumerate()
            .fold(Scrollable::new(&mut self.scrollable), |column, (i, cat)| {
                column.push(cat.view(ctx).map(move |msg| Msg::Category(i, msg)))
            })
            .into()
    }

    pub fn update<T: DeviceControl, U: GenApiCtxt>(
        &mut self,
        msg: Msg,
        ctx: &mut ParamsCtxt<T, U>,
    ) {
        match msg {
            Msg::Category(i, msg) => self.categories[i].update(msg, ctx),
        }
    }
}
