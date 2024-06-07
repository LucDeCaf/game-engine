use crate::traits::Component;

#[derive(Debug)]
pub struct Position {
    pub x: f64,
    pub y: f64,
}

impl Component for Position {}
