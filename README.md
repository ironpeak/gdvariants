# gdvariants

Rust std library collections wrapper that implements the godot-rust variant traits for convenience when using [godot-rust](https://github.com/godot-rust/godot-rust).

Traits implemented on top of the standard library implementation:

- [Export](https://docs.rs/gdnative/0.10.0/gdnative/export/trait.Export.html) required for the [property](https://godot-rust.github.io/book/rust-binding/properties.html) attribute.
- [FromVariant](https://docs.rs/gdnative/0.10.0/gdnative/core_types/trait.FromVariant.html) required for converting Godot types to Rust types.
- [ToVariant](https://docs.rs/gdnative/0.10.0/gdnative/core_types/trait.ToVariant.html) required for converting Rust types to Godot types.
- [Borrow](https://doc.rust-lang.org/std/borrow/trait.Borrow.html) for borrowing the base standard library type.
- [BorrowMut](https://doc.rust-lang.org/std/borrow/trait.BorrowMut.html) for borrowing the base standard library type as a mutable reference.
- [From](https://doc.rust-lang.org/std/convert/trait.From.html) for converting standard library types to gdvariant types.

## Types

- HashMap
- HashSet
- Vec

## Usage

Add this to your `Cargo.toml`:

~~~toml
gdvariants = "*"
~~~

Read the [godot-rust book](https://godot-rust.github.io/book/getting-started.html) for information on how to setup a Godot project that uses Rust.

### Property

~~~rust
use gdnative::api::*;
use gdnative::prelude::*;

use gdvariants::collections::HashMap;

#[derive(NativeClass, Default)]
#[inherit(Node)]
pub struct ExampleHashMapProperty {
    #[property]
    players: HashMap<i64, String>,
}
~~~

### Networking

~~~rust
use gdnative::api::*;
use gdnative::prelude::*;

use gdvariants::collections::HashMap;

...

fn send_data(&mut self, owner: &Node) {
    let mut data: HashMap<i64, i64> = HashMap::new();
    data.insert(1, 0);

    for player in self.players.keys() {
        owner.rpc_id(*player, "data", &[data.to_variant()]);
    }
}
~~~

~~~rust
use gdnative::api::*;
use gdnative::prelude::*;

use gdvariants::collections::HashMap;

...

fn receive_data(&mut self, owner: &Node, data: HashMap<i64, i64>) {
    let mut data: HashMap<i64, i64> = HashMap::new();
    data.insert(1, 0);

    for player in self.players.keys() {
        owner.rpc_id(*player, "data", &[data.to_variant()]);
    }
}
~~~

## Crate Features

* serde: enables deserialize and serialize for collections.
