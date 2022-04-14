use gdnative::api::*;
use gdnative::prelude::*;

use gdvariants::collections::HashSet;

#[derive(NativeClass, Default)]
#[inherit(Node)]
pub struct ExampleHashSetProperty {
    #[property]
    players: HashSet<String>,
}

impl ExampleHashSetProperty {
    fn new(_owner: &Node) -> Self {
        Self::default()
    }
}

#[methods]
impl ExampleHashSetProperty {
    #[export]
    fn _ready(&self, _owner: &Node) {
        godot_print!("HashSet:");
        for name in &self.players {
            godot_print!("Hello, {}!", name);
        }
        godot_print!("Hello, world!");
    }
}
