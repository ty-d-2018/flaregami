use super::terrain::{
    Material,
    sensory::Vector,
};

use std::collections::HashMap;

pub struct Blueprint{
    points: HashMap::<usize, Box::<dyn Vector>>,
}

impl Blueprint{
    pub fn new() -> Blueprint{
        Blueprint{
            points: HashMap::new(),
        }
    }
}