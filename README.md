# This crate provides a set of Rust macros that reduce the amount of code when working with async closures.

[![Tests](https://github.com/n08i40k/async-macro/actions/workflows/tests.yml/badge.svg)](https://github.com/n08i40k/async-macro/actions/workflows/tests.yml)

## Installation

Add `async-macro` and `futures-util`* as a dependencies in your `Cargo.toml`:

```toml
[dependencies]
async-macro = { git = "https://github.com/n08i40k/async-macro.git"}
futures-util = "0.3.31"
```

\* - Optional. Only needed when using `async_macro::types::*`.

## Usage

### Closure Macros

* **`async_closure!((arcs...), (args...), async_expr)`**
  This is a macro for building async closures. It returns a closure that produces a `BoxFuture` when called,
  automatically cloning any captured `Arc` variables.

* These are convenient shortcuts:
    * `async_box_closure!`: wraps `async_closure!` in a `Box::new`.
    * `async_arc_closure!`: wraps `async_closure!` in an `Arc::new`.

### Future Type Macros

* **`box_future_type!((ArgTypes...), OutputType)`**
  Expands to `Box<dyn futures_util::future::BoxFuture<'static, OutputType> + Send + Sync>`.
* **`arc_future_type!((ArgTypes...), OutputType)`**
  Same, but wrapped in an `Arc<…>` instead of a `Box<…>`.

### Example: Capturing and Invoking

```rust
use async_macro::{async_box_closure, box_future_type};
use std::sync::Arc;

// Define a boxed future type taking two i32s and returning i32
type CalcFn = box_future_type!((i32, i32), i32);

// Capture shared state (two Arcs) and define an async closure
let shared_a = Arc::new(1);
let shared_b = Arc::new(2);

let calc: CalcFn = async_box_closure!(
    (shared_a, shared_b),        // vars to clone
    (x, y),                      // closure args
    async move {
        (*shared_a + *shared_b) * (x + y)
    }
);

// Invoke the closure asynchronously
let result = calc(3, 4).await; // (1 + 2) * (3 + 4)
```