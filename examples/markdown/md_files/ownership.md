# Ownership & Borrowing 

**Level 2**

This file explains Rust's ownership model, a core concept that guarantees memory safety and eliminates many common programming errors. Understand how ownership, borrowing, and lifetimes work in Rust.

## Ownership Rules

Rust enforces strict ownership rules:

| Rule | Description |
|------|------------|
| **Each value in Rust has a single owner** | A value can only have one owner at a time. |
| **When the owner goes out of scope, the value is dropped** | Rust automatically cleans up memory when a value’s owner is no longer needed. |
| **Ownership can be transferred (moved) or borrowed** | You can transfer ownership or allow temporary access via borrowing. |

## Code Example: Ownership and Borrowing

```
fn main() {
    let s1 = String::from("Hello");
    let s2 = s1; // Ownership moves to s2, s1 is invalid

    // println!("{}", s1); // Error: s1 is no longer valid

    let s3 = String::from("Rust");
    let s4 = &s3; // Borrowing s3, s3 is still valid

    println!("s3: {}, s4: {}", s3, s4); // Works fine
}
```
- [Back to Introduction](introduction.md)