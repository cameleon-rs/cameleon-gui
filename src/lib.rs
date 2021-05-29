use context::CameraId;
use derive_more::From;
use iced::{
    executor, Application, Clipboard, Color, Column, Command, Container, Element, Row, Subscription,
};
use std::borrow::Cow;

mod camera;
mod context;
mod control;
mod convert;
mod features;
mod frame;
mod genapi;
mod scanner;
mod selector;
mod style;

#[derive(Default)]
pub struct App {
    ctx: context::Context,
    control: control::Control,
    selector: selector::Selector,
    scanner: scanner::Scanner,
    features: features::Features,
    frame: frame::Frame,
    debug: bool,
}

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("cameleon error: {0}")]
    CameleonError(#[from] cameleon::CameleonError),

    #[error("stream error: {0}")]
    StreamError(#[from] cameleon::StreamError),

    #[error("control error: {0}")]
    ControlError(#[from] cameleon::ControlError),

    #[error("genapi error: {0}")]
    GenApiError(#[from] cameleon_genapi::GenApiError),

    #[error("failed conversion: {0}")]
    ConversionError(#[from] convert::Error),

    #[error("not found: {0:?}")]
    NotFound(CameraId),

    #[error("internal error: {0}")]
    InternelError(Cow<'static, str>),
}

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, From)]
pub enum Msg {
    Control(control::Msg),
    Selector(selector::Msg),
    Scanner(scanner::Msg),
    Features(features::Msg),
    Frame(frame::Msg),
}

#[derive(Debug, Default)]
pub struct Flags {
    pub debug: bool,
}

impl Application for App {
    type Message = Msg;
    type Executor = executor::Default;
    type Flags = Flags;

    fn new(flags: Self::Flags) -> (Self, Command<Self::Message>) {
        let mut app = Self {
            debug: flags.debug,
            ..Default::default()
        };
        app.update_scanner(scanner::Msg::Scan);
        (app, Command::none())
    }

    fn title(&self) -> String {
        "Cameleon".to_string()
    }

    fn view(&mut self) -> Element<'_, Self::Message> {
        let control = self.control.view(&self.ctx).map(Into::into);
        let selector = self.selector.view(&self.ctx).map(Into::into);
        let scanner = self.scanner.view(&mut self.ctx).map(Into::into);
        let features = self.features.view(&mut self.ctx).map(Into::into);
        let frame = self.frame.view(&self.ctx).map(Into::into);

        let content: Element<_> = Column::new()
            .push(control)
            .push(
                Row::new()
                    .push(
                        Column::new()
                            .max_width(400)
                            .push(selector)
                            .push(scanner)
                            .push(features),
                    )
                    .push(frame),
            )
            .into();
        if self.debug {
            Container::new(content.explain(Color::from_rgb8(200, 30, 30))).into()
        } else {
            Container::new(content).into()
        }
    }

    fn update(
        &mut self,
        message: Self::Message,
        _clipboard: &mut Clipboard,
    ) -> Command<Self::Message> {
        match message {
            Msg::Control(msg) => self.update_control(msg),
            Msg::Selector(msg) => self.update_selector(msg),
            Msg::Scanner(msg) => self.update_scanner(msg),
            Msg::Frame(msg) => self.update_frame(msg),
            Msg::Features(msg) => self.update_features(msg),
        }
    }

    fn subscription(&self) -> Subscription<Self::Message> {
        Subscription::batch(vec![
            self.frame.subscription().map(Into::into),
            self.scanner.subscription().map(Into::into),
        ])
    }
}

impl App {
    fn update_control(&mut self, msg: control::Msg) -> Command<Msg> {
        match self.control.update(msg, &mut self.ctx) {
            Ok(out) => match out {
                control::OutMsg::Open(id) => self.update_features(features::Msg::Load(id)),
                control::OutMsg::StartStreaming(_id, receiver) => {
                    self.update_frame(frame::Msg::Attach(receiver))
                }
                control::OutMsg::StopStreaming(_) => self.update_frame(frame::Msg::Detach),
                control::OutMsg::Close(_) | control::OutMsg::None => Command::none(),
            },
            Err(err) => {
                tracing::error!("{}", err);
                Command::none()
            }
        }
    }

    fn update_selector(&mut self, msg: selector::Msg) -> Command<Msg> {
        if let Err(err) = self.selector.update(msg, &mut self.ctx) {
            tracing::error!("{}", err)
        };
        Command::none()
    }

    fn update_scanner(&mut self, msg: scanner::Msg) -> Command<Msg> {
        match self.scanner.update(msg, &mut self.ctx) {
            Ok(msg) => match msg {
                scanner::OutMsg::SyncIds => Command::batch(vec![
                    self.update_selector(selector::Msg::SyncIds),
                    self.update_features(features::Msg::SyncIds),
                ]),
                scanner::OutMsg::None => Command::none(),
            },
            Err(err) => {
                tracing::error!("{}", err);
                Command::none()
            }
        }
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
        if let Err(err) = self.features.update(msg, &mut self.ctx) {
            tracing::error!("{}", err);
        }
        Command::none()
    }
}
