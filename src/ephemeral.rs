
#[derive(Clone)]
pub struct Id<T: Clone>{
    lookup: T,
}

impl<T: Clone> Id<T>{
    pub fn new(serial: T) -> Id<T>{
        Id{
            lookup: serial,
        }
    }

    pub fn get_id(&self) -> T{
        self.lookup.clone()
    }
}