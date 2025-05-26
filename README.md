# WARP
WARP (WebAssembly Application Runtime using Packages) is a Rust-first toolkit for building modular WebAssembly components with WIT and cargo component. It enables fast, portable development of composable WASM apps.

## Install
```bash
./scaffold-component.sh
```

## Code `src/lib.rs`
```rust
#[allow(warnings)]
mod bindings;

use bindings::Guest;

struct Component;

impl Guest for Component {
    /// Say hello!
    fn hello_world() -> String {
        "Hello, World!".to_string()
    }
}

bindings::export!(Component with_types_in bindings);
```

## Code `wit/world.wit`
```
package component:hello-world;

/// An example world for the component to target.
world example {
    export hello-world: func() -> string;
}
```

<br>

## License
[Apache License, Version 2.0](https://www.apache.org/licenses/LICENSE-2.0)
