use crate::scene::physics::obj::obj;
pub struct world {
    objs: Vec<*mut obj>,
}
impl world {
    pub fn new() -> Self {
        return world {
            objs: Vec::new(),
        };
    }
    pub fn add_obj(&mut self, obj: *mut obj) {
        self.objs.push(obj);
    }
    pub fn simulate(&mut self, delta: f32) {
        
    }
}