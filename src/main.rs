use cameleon_gui::App;
use iced::{Application, Settings};

fn main() {
    tracing_subscriber::fmt()
        .pretty()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .init();
    App::run(Settings::default()).unwrap();
}
