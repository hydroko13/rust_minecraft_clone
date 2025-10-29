pub enum HandleEnum {
    Uninitialized,
    Initialized(u32),
}

pub struct Handle {
    pub handle: HandleEnum,
}

impl Handle {
    pub fn new() -> Self {
        Handle {
            handle: HandleEnum::Uninitialized
        }
    }
    pub fn store(&mut self, val: u32) {
        self.handle = HandleEnum::Initialized(val);
    }
    
    pub fn read(&self) -> Option<u32> {
        match self.handle {
            HandleEnum::Initialized(val) => Option::Some(val),
            HandleEnum::Uninitialized => Option::None
        }
    }



}




