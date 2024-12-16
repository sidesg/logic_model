#[derive(PartialEq)]
#[derive(Debug)]
pub struct World {
    id: u64,
    c: Option<String>
}

impl World {
    pub fn derive_world(&self) -> World {
        World {
            id: (self.id +1).clone(),
            c: self.c.clone()
        }
    }

    pub fn first_world() -> World {
        World {
            id: 0,
            c: None
        }
    }
}