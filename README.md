# AVR Stack

This crate provides helper functions for stack analysis on AVR microcontrollers.

- **Stack Usage Estimation:** Estimate the amount of unused stack space.

## Usage

This crate provides two main items:

- `init_stack_pattern!()`:
  A macro to initialize the stack with a known pattern.
  This should be called once in your application's main crate.
- `estimate_unused_stack_space()`:
  A function that returns an estimate of the number of unused bytes on the stack.

## Stack Usage Estimation

The `init_stack_pattern!` macro defines a function that runs in the `.init4` section of the AVR's program memory.
This function executes before `main` and fills the entire stack with a pattern.

The `estimate_unused_stack_space()` function then walks the stack from its end towards its beginning, counting the number of bytes that still contain the initialization pattern.
This provides an estimate of how much stack space has never been touched.

## License

This project is licensed under either of

- Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.