# CrossTerm

This backend relies on the `crossterm` crate to provide a terminal abstraction layer. It supports various terminal features such as mouse input, keyboard input, and screen manipulation.

For clipboard operations, it uses the `copypaste` crate's built-in clipboard functionality for UNIX-like systems, and windows API for Windows systems. 

## Limitations

Some flickering issues may be seen when using this backend (in particular for old terminals). This is due to the way `crossterm` handles terminal output and input. If you experience flickering, consider using a different backend or adjusting your terminal settings.

## Usage

This backend is not enabled by default. To enable it, you need to compile your application with the feature `CROSSTERM` enabled in your `Cargo.toml` file:

```toml
[dependencies]
appcui = { version = "*", features = ["CROSSTERM"] }
```

Then, you can create your application using the `App::with_backend` method, specifying the `Type::CrossTerm` backend:

```rust
use appcui::prelude::*;

fn main() -> Result<(), appcui::system::Error> {
    let app = App::with_backend(appcui::backend::Type::CrossTerm).build()?;
    // build your application here
    Ok(())
}
```