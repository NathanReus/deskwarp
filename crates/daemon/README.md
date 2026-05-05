# DeskWarp daemon

The background service that controls wallpaper changes.  
It runs a tray icon, an HTTP server (axum), and integrates with the
`multi-wallpaper` library.

## Current state

The daemon starts, shows a system‑tray icon, and exposes a health endpoint
at `http://127.0.0.1:<port>/api/health`. The port is written to
`daemon_port` inside the user’s config directory.

## Running

```bash
cargo run -p deskwarp-daemon
```

Set `RUST_LOG=info` (or `deskwarp_daemon=debug`) for log output.

## License

Same dual‑licensing as the workspace.
