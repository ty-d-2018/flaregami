use std::collections::HashMap;

use super::craft::teaching::{ Blueprint };

pub struct Ancestor{
    wisdom: HashMap::<String, Blueprint>,
}

impl Ancestor{
    pub fn new() -> Ancestor{
        Ancestor{
            wisdom: HashMap::new(),
        }
    }
    pub fn speak(&self){}
}