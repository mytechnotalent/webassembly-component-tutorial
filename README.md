# Webassembly Component Tutorial

This tutorial walks you through building, composing, and running a small component-model calculator example. The example uses three components:

- An addition operation (adder)
- A calculator engine (calculator) that imports adder
- A command-line interface (command) that imports calculator and exports `wasi:cli/run`

We'll build each component, compose them into a single runnable component with `wac`, and execute it with `wasmtime`.

---

## Prerequisites

Install these tools on your machine:

- Rust toolchain (rustup + cargo)
  - If needed: https://rustup.rs
- cargo-component
  - `cargo install cargo-component --locked`
- wac CLI
  - `cargo install wac-cli`
- wasmtime (CLI)
  - `curl https://wasmtime.dev/install.sh -sSf | bash`
  - After installing wasmtime you may need to close and reopen your shell so the `wasmtime` binary is on your PATH.

---

## Project layout (where files live)

The example layout:

```
.
├── adder
│   └── (adder crate)
├── calculator
│   └── (calculator crate)
├── command
│   └── (command crate)
└── wit
    ├── adder/world.wit
    └── calculator/world.wit
```

---

## WIT interfaces

Put these two WIT packages under `wit/`:

- wit/adder/world.wit
```wit
package docs:adder@0.1.0;

interface add {
    add: func(x: u32, y: u32) -> u32;
}

world adder {
    export add;
}
```

- wit/calculator/world.wit
```wit
package docs:calculator@0.1.0;

interface calculate {
    enum op {
        add,
    }
    eval-expression: func(op: op, x: u32, y: u32) -> u32;
}

world calculator {
    export calculate;
    import docs:adder/add@0.1.0;
}

world app {
    import calculate;
}
```

These define:
- `adder` world that exports a simple `add` function.
- `calculator` world that exports `calculate` and imports the `adder` world.
- `app` world that imports `calculate` (used by the `command` component).

---

## Create components

### 1) Add component (adder)
Implement the `adder` world in the `adder` crate. This crate should implement the `add` function and be compiled as a component. With `cargo-component` you typically implement the Rust function and build with `cargo component build`.

Example (high-level steps):
```bash
# inside your repo root
cd adder
# implement the add logic in src/lib.rs per the language guide / wit-bindgen generated bindings
cargo component build --release
```

The produced component binary will be at:
```
adder/target/wasm32-wasip1/release/adder.wasm
```

(If you used a debug build, check the `debug` directory.)

### 2) Calculator component
Implement the `calculator` world in the `calculator` crate. It should import the `adder` interface and call it when the `op` is `add`.

High-level steps:
```bash
cd calculator
# ensure calculator crate references the calculator WIT under package.metadata.component.target
# implement the evaluator in src/lib.rs using the generated bindings
cargo component build --release
```

Output:
```
calculator/target/wasm32-wasip1/release/calculator.wasm
```

### 3) Command component
Create a command-line component that exports `wasi:cli/run`. Scaffolding:

```bash
# from repo root
cargo component new command --command
```

In `command/Cargo.toml` point the component target to the app world from the calculator WIT and add the adder WIT as a dependency so the toolchain can resolve the world imports:

```toml
[package.metadata.component.target]
path = "../wit/calculator/world.wit"
world = "app"

[package.metadata.component.target.dependencies]
"docs:adder" = { path = "../wit/adder" }
```

Implement the CLI in `command/src/main.rs`:
- Parse three args: `<x> <y> <op-name>` (e.g., `1 2 add`)
- Convert `op-name` to the `op` enum (support `add`)
- Call `calculate::eval_expression(op, x, y)` (the generated binding)

Then build:
```bash
cd command
cargo component build --release
```

Output:
```
command/target/wasm32-wasip1/release/command.wasm
```

---

## Compose components with wac

Now we plug components together so every import is satisfied and produce one final runnable component.

From the repository root run:

```bash
# compose calculator + adder -> composed.wasm
wac plug calculator/target/wasm32-wasip1/release/calculator.wasm \
  --plug adder/target/wasm32-wasip1/release/adder.wasm \
  -o composed.wasm

# compose command + composed -> final.wasm (this final component will export wasi:cli/run)
wac plug command/target/wasm32-wasip1/release/command.wasm \
  --plug composed.wasm \
  -o final.wasm
```

What these commands do:
- The first `wac plug` satisfies the calculator's import of the adder.
- The second `wac plug` satisfies the command's import of the calculator and yields a component with `wasi:cli/run` export.

---

## Run the composed component

Run the final component using `wasmtime`:

```bash
wasmtime run final.wasm -- 1 2 add
```

Expected output:
```
1 + 2 = 3
```

Notes:
- The `--` separates wasmtime flags from arguments passed to the component.
- Make sure you have a recent `wasmtime` release with component model support.
