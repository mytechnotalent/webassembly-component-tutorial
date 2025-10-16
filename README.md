# WebAssembly Component Tutorial

This tutorial walks you through building, composing, and running a small WebAssembly component-model calculator example. The example uses four components:

- An addition operation (adder)
- A subtraction operation (subtractor)
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
├── subtractor
│   └── (subtractor crate)
└── wit
  ├── adder/world.wit
  ├── calculator/world.wit
  └── subtractor/world.wit
```

---

## WIT interfaces

Put these WIT packages under `wit/`:

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

- wit/subtractor/world.wit
```wit
package docs:subtractor@0.1.0;

interface subtract {
    subtract: func(x: u32, y: u32) -> u32;
}

world subtractor {
    export subtract;
}
```

- wit/calculator/world.wit
```wit
package docs:calculator@0.1.0;

interface calculate {
    enum op {
        add,
        subtract,
    }
    eval-expression: func(op: op, x: u32, y: u32) -> u32;
}

world calculator {
    export calculate;
    import docs:adder/add@0.1.0;
    import docs:subtractor/subtract@0.1.0;
}

world app {
    import calculate;
}
```

These define:
- `adder` world that exports a simple `add` function.
- `subtractor` world that exports a simple `subtract` function.
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

### 2) Subtract component (subtractor)

Implement the `subtractor` world in the `subtractor` crate. This crate should implement the `subtract` function and be compiled as a component. With `cargo-component` you typically implement the Rust function and build with `cargo component build`.

High-level steps:
```bash
cd subtractor
# implement the subtract logic in src/lib.rs per the language guide / wit-bindgen generated bindings
cargo component build --release
```

Output:
```
subtractor/target/wasm32-wasip1/release/subtractor.wasm
```

### 3) Calculator component

Implement the `calculator` world in the `calculator` crate. It should import the `adder` interface and call it when the `op` is `add`.

### 4) Calculator component
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

### 5) Command component
Implement the `app` world in the `command` crate. This component should import the `calculator` interface and export `wasi:cli/run` to create a command-line interface.

High-level steps:
```bash
cd command
# ensure command crate references the calculator WIT under package.metadata.component.target
# implement the CLI logic in src/main.rs using the generated bindings
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
# compose calculator + adder + subtractor -> composed.wasm
wac plug calculator/target/wasm32-wasip1/release/calculator.wasm \
  --plug adder/target/wasm32-wasip1/release/adder.wasm \
  --plug subtractor/target/wasm32-wasip1/release/subtractor.wasm \
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
wasmtime run final.wasm -- 5 3 subtract
```

Expected output:
```
1 + 2 = 3
5 - 3 = 2
```

Notes:
- The `--` separates wasmtime flags from arguments passed to the component.
- Make sure you have a recent `wasmtime` release with component model support.
