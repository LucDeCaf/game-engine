use crate::app::App;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SystemRuntime {
    Startup,
    Update,
}

pub struct Interface {
    app: &'static App,
}
