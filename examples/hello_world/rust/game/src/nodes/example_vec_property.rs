use gdnative::api::*;
use gdnative::prelude::*;

use gdvariants::vec::Vec;

#[derive(NativeClass, Default)]
#[inherit(Node)]
pub struct ExampleVecProperty {
    #[property]
    players: Vec<String>,
}

impl ExampleVecProperty {
    fn new(_owner: &Node) -> Self {
        Self::default()
    }
}

#[methods]
impl ExampleVecProperty {
    #[export]
    fn _ready(&self, _owner: &Node) {
        godot_print!("Vec:");
        for name in &self.players {
            godot_print!("Hello, {}!", name);
        }
        godot_print!("Hello, world!");
    }
}
