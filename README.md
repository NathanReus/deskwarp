# DeskWarp

DeskWarp is a cross‑platform desktop wallpaper manager.  
It lets you set, schedule, and rotate wallpapers across multiple monitors using a
lightweight daemon with a tray icon and an HTTP API.

## Structure

- [**multi-wallpaper**](crates/multi-wallpaper) – the core platform‑abstraction library
- [**api-models**](crates/api-models) – shared request/response types for the HTTP API
- [**daemon**](crates/daemon) – the tray‑icon daemon and HTTP server
- [**gui**](crates/gui) – the optional graphical configuration interface (coming soon)

## Building

The project is a Cargo workspace. Run from the workspace root:

```bash
cargo build
```

The daemon binary will be at `target/debug/deskwarp-daemon`.

## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.
