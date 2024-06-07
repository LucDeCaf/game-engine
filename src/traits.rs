use crate::app::App;
use std::any::Any;

pub trait Component: Any + 'static {}

pub trait Plugin {
    fn attach(&self, app: &mut App);
}
