# Memory Safety in Rust

**Level 3**

This file discusses how Rust's ownership and type system ensure memory safety without relying on a garbage collector. Learn how Rust prevents common bugs like dangling pointers and data races.

## How Rust Ensures Memory Safety

1. Ownership System: Each value in Rust has a unique owner. When the owner goes out of scope, the value is automatically deallocated.
2. Borrowing & Reference: Borrowing prevents multiple mutable references that could cause data races. Immutable and mutable references enforce safe concurrent access.
3. Lifetimes: Lifetimes ensure references are always valid, preventing use-after-free and dangling pointers.
4. Safe Concurrency: Rust’s type system enforces thread safety at compile time. The `Send` and `Sync` traits ensure safe data sharing across threads.

- [Back to Language Features](features.md)
