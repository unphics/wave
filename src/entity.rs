use crate::component::component;

use std::collections::HashMap;

struct entity {
    id: u64,
    components: HashMap<String, component>,
}

impl entity {
    fn new(id: u64) -> Self {
        return entity{id, components: HashMap::new()}
    }
    fn add_component(&mut self, component: component) {
        self.components.insert(component.name.clone(), component);
    }
    fn get_component(&self, name: String) -> Option<&component>{
        self.components.get(name.as_str())
    }
}