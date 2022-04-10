use gdnative::prelude::*;

mod nodes;

// Function that registers all exposed classes to Godot
fn init(handle: InitHandle) {
    // Register the new `HelloWorld` type we just declared.
    handle.add_class::<nodes::ExampleHashMapProperty>();
    handle.add_class::<nodes::ExampleVecProperty>();
}

// Macro that creates the entry-points of the dynamic library.
godot_init!(init);
