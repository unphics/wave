
pub struct role {
    id: i32,
    name: String,
}

impl role {
    pub fn new(id: i32, name: String) -> Self {
        return role {
            id: id,
            name: name
        };
    }
    pub fn on_create(&mut self) {
        
    }
}