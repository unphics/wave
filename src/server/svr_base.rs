use std::collections::HashMap;

pub struct svr_base {
    name: String,
}

impl svr_base {
    pub fn new(name: String) -> svr_base {
        svr_base{name: name}
    }
    pub fn name(&self)  -> String {
        self.name.clone()
    }
}