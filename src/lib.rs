use std::collections::{BTreeMap, HashMap, HashSet};
use std::time::Duration;

use anyhow::Result;
use cameleon::{
    self,
    genapi::{DefaultGenApiCtxt, ParamsCtxt},
    u3v,
};
use iced::{
    button, executor, time, Application, Clipboard, Column, Command, Container, Element, Row,
    Subscription,
};
use tracing::trace;

mod camera;
mod control;
mod convert;
mod frame;
mod genapi;
mod selector;
mod style;

use camera::{Camera, CameraId};

#[derive(Default)]
pub struct App {
    cameras: HashMap<CameraId, Camera>,
    selected: Option<CameraId>,
    selector: selector::Selector,
    control: control::Control,
    genapis: HashMap<CameraId, genapi::GenApi>,
    frame: frame::Frame,
}

#[derive(Debug, Clone)]
pub enum Msg {
    // Camera control
    Open,
    Close,
    StartStreaming,
    StopStreaming,
    // Camera selector
    Refresh,
    Selected(CameraId),
    // GenAPI
    GenApi(genapi::Msg),
    // Frame
    UpdateFrame,
}

macro_rules! view_genapi {
    ($selected: expr, $genapis: expr) => {
        if let Some(selected) = $selected {
            Row::new()
        } else {
            Row::new()
        }
    };
}

macro_rules! check {
    ($res: expr) => {
        if let Err(err) = $res {
            tracing::error!("{}", err)
        }
    };
}

impl Application for App {
    type Message = Msg;
    type Executor = executor::Default;
    type Flags = ();

    fn new(_flags: Self::Flags) -> (Self, Command<Self::Message>) {
        let mut app = Self::default();
        check!(app.refresh());
        (app, Command::none())
    }

    fn title(&self) -> String {
        "Cameleon".to_string()
    }

    fn view(&mut self) -> Element<'_, Self::Message> {
        let control = self.control.view(self.selected_state());
        let selector = self.selector.view(self.selected);
        let genapi = view_genapi!(self.selected, self.genapis);
        let frame = self.frame.view();

        Column::new()
            .push(control)
            .push(
                Row::new()
                    .push(Column::new().max_width(400).push(selector).push(genapi))
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
            Msg::Open => {
                if let Some(cam) = self.selected_mut() {
                    check!(cam.raw.open());
                    check!(cam.raw.load_context());
                }
            }
            Msg::Close => {
                if let Some(cam) = self.selected_mut() {
                    check!(cam.raw.close());
                }
            }
            Msg::StartStreaming => {
                if let Some(selected) = self.selected_mut() {
                    match selected.raw.start_streaming(1) {
                        Ok(receiver) => self.frame.attach(receiver),
                        Err(err) => tracing::error!("{}", err),
                    }
                }
            }
            Msg::StopStreaming => {
                if let Some(cam) = self.selected_mut() {
                    check!(cam.raw.stop_streaming());
                    self.frame.detach();
                }
            }
            Msg::Refresh => check!(self.refresh()),
            Msg::Selected(info) => self.selected = Some(info),
            Msg::GenApi(msg) => {
                if let Some(camera) = self.selected_mut() {
                    let params_ctxt = camera.raw.params_ctxt();
                };

                if let Some(selected) = self.selected {
                    if let Some(genapi) = self.genapis.get_mut(&selected) {
                        genapi.update(msg)
                    }
                }
            }
            Msg::UpdateFrame => {
                check!(self.frame.update())
            }
        }
        Command::none()
    }

    fn subscription(&self) -> Subscription<Self::Message> {
        time::every(Duration::from_millis(100)).map(|_| Msg::UpdateFrame)
    }
}

impl App {
    fn refresh(&mut self) -> Result<()> {
        let raws: Vec<cameleon::Camera<u3v::ControlHandle, u3v::StreamHandle, DefaultGenApiCtxt>> =
            u3v::enumerate_cameras().unwrap(); // TODO: Add error handling
        trace!("Detected {} cameras", raws.len());
        let mut exists: HashSet<CameraId> = self.cameras.iter().map(|(id, _)| *id).collect();
        for raw in raws {
            let camera = Camera::new(raw)?;
            if exists.contains(&camera.id) {
                exists.remove(&camera.id);
                continue;
            }
            self.selector
                .options
                .insert(camera.id, (camera.name.clone(), button::State::new()));
            self.cameras.insert(camera.id, camera);
        }
        for removed in exists {
            // TODO: close and/or stop streaming
            self.selector.options.remove(&removed);
            self.cameras.remove(&removed);
        }

        // Drop selected if it removed
        if let Some(selected) = self.selected {
            if !self.cameras.contains_key(&selected) {
                self.selected = None
            }
        }

        // Randomly selected if it is `None`
        if self.selected.is_none() && self.cameras.len() > 0 {
            self.selected = Some(*self.cameras.keys().next().unwrap());
        }

        Ok(())
    }

    fn selected(&self) -> Option<&Camera> {
        if let Some(selected) = self.selected {
            self.cameras.get(&selected)
        } else {
            None
        }
    }

    fn selected_mut(&mut self) -> Option<&mut Camera> {
        if let Some(selected) = self.selected {
            self.cameras.get_mut(&selected)
        } else {
            None
        }
    }

    fn selected_state(&self) -> Option<camera::State> {
        self.selected().map(|cam| cam.state())
    }
}
