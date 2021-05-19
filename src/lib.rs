use derive_more::From;
use iced::{executor, Application, Clipboard, Column, Command, Element, Row, Subscription};

mod camera;
mod context;
mod control;
mod convert;
mod features;
mod frame;
mod genapi;
mod selector;
mod style;

#[derive(Default)]
pub struct App {
    ctx: context::Context,
    selector: selector::Selector,
    control: control::Control,
    features: features::Features,
    frame: frame::Frame,
}

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Cameleon error: {0}")]
    CameleonError(#[from] cameleon::CameleonError),

    #[error("stream error: {0}")]
    StreamError(#[from] cameleon::StreamError),

    #[error("control error: {0}")]
    ControlError(#[from] cameleon::ControlError),

    #[error("failed conversion")]
    FailedConversion,
}

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, From)]
pub enum Msg {
    Control(control::Msg),
    Selector(selector::Msg),
    Features(features::Msg),
    Frame(frame::Msg),
}

impl Application for App {
    type Message = Msg;
    type Executor = executor::Default;
    type Flags = ();

    fn new(_flags: Self::Flags) -> (Self, Command<Self::Message>) {
        let mut app = Self::default();
        app.ctx.refresh();
        (app, Command::none())
    }

    fn title(&self) -> String {
        "Cameleon".to_string()
    }

    fn view(&mut self) -> Element<'_, Self::Message> {
        let control = self.control.view(&self.ctx).map(Into::into);
        let selector = self.selector.view(&self.ctx).map(Into::into);
        let features = self.features.view(&mut self.ctx).map(Into::into);
        let frame = self.frame.view(&self.ctx).map(Into::into);

        Column::new()
            .push(control)
            .push(
                Row::new()
                    .push(Column::new().max_width(400).push(selector).push(features))
                    .push(frame),
            )
            .into()
    }

    #[tracing::instrument(skip(self, _clipboard), level = "trace")]
    fn update(
        &mut self,
        message: Self::Message,
        _clipboard: &mut Clipboard,
    ) -> Command<Self::Message> {
        match message {
            Msg::Control(msg) => self.update_control(msg),
            Msg::Selector(msg) => self.update_selector(msg),
            Msg::Frame(msg) => self.update_frame(msg),
            Msg::Features(msg) => self.update_features(msg),
        }
    }

    fn subscription(&self) -> Subscription<Self::Message> {
        Subscription::batch(vec![
            self.frame.subscription().map(Into::into),
            self.selector.subscription().map(Into::into),
        ])
    }
}

impl App {
    fn update_control(&mut self, msg: control::Msg) -> Command<Msg> {
        match self.control.update(msg, &mut self.ctx) {
            Ok(Some(out)) => match out {
                control::OutMsg::Frame(msg) => self.update_frame(msg),
            },
            Ok(None) => Command::none(),
            Err(err) => {
                tracing::error!("{}", err);
                Command::none()
            }
        }
    }

    fn update_selector(&mut self, msg: selector::Msg) -> Command<Msg> {
        self.selector.update(msg, &mut self.ctx).map(Into::into)
    }

    fn update_frame(&mut self, msg: frame::Msg) -> Command<Msg> {
        match self.frame.update(msg, &mut self.ctx) {
            Ok(cmd) => cmd.map(Into::into),
            Err(err) => {
                tracing::error!("{}", err);
                Command::none()
            }
        }
    }

    fn update_features(&mut self, msg: features::Msg) -> Command<Msg> {
        self.features.update(msg, &mut self.ctx);
        Command::none()
    }
}
