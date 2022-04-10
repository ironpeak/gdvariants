use gdnative::api::*;
use gdnative::prelude::*;

use gdvariants::collections::HashMap;

#[derive(NativeClass, Default)]
#[inherit(Node)]
pub struct ExampleHashMapProperty {
    #[property]
    players: HashMap<i64, String>,
}

impl ExampleHashMapProperty {
    fn new(_owner: &Node) -> Self {
        Self::default()
    }
}

#[methods]
impl ExampleHashMapProperty {
    #[export]
    fn _ready(&self, _owner: &Node) {
        godot_print!("HashMap:");
        for (id, name) in &self.players {
            godot_print!("Hello, {} - {}!", id, name);
        }
        godot_print!("Hello, world!");
    }
}
