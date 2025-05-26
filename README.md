# WARP
WARP (WebAssembly Application Runtime using Packages) is a Rust-first toolkit for building modular WebAssembly components with WIT and cargo component. It enables fast, portable development of composable WASM apps.

<br>

## Purpose
WARP empowers developers to build portable, composable WebAssembly applications by leveraging the Component Model, WIT interfaces, and Rust as a first-class implementation language. The goal is to simplify the creation of modular Wasm components that communicate through clearly defined interfaces—eliminating the need for unsafe pointer manipulation or runtime glue.

At the core of WARP is the idea that components should be reusable, language-agnostic units of logic. Each component defines and/or consumes interfaces using WIT (WebAssembly Interface Types), which describe functions, records, enums, and resources in a language-neutral format. These interfaces are grouped into worlds, which represent a complete set of expected imports and exports for a component. Projects are organized into packages, which are versioned collections of WIT definitions and implementations.

WARP streamlines the development process using cargo component for scaffolding, binding generation, and building, and wasmtime for running and composing components. By following the WARP structure, developers can create scalable applications with strong interface contracts and the flexibility to mix and match components from different packages, languages, or runtimes—all with Rust at the helm.

<br>

## Install
```bash
./scaffold-component.sh
```

<br>

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
