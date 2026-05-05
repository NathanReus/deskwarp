# multi-wallpaper

A cross‑platform Rust library for managing desktop wallpapers.

Currently implemented for Windows; other platforms will follow.

## Usage

```rust
use multi_wallpaper::Wallpaper;

let wallpaper = Wallpaper::new();
let monitors = wallpaper.list_monitors();
wallpaper.set("path/to/image.jpg", multi_wallpaper::Style::Fill, None)?;
```

## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.
