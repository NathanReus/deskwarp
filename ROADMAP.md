# DeskWarp Roadmap

This document tracks progress toward the first public release of DeskWarp, a Windows system‑tray wallpaper manager with an HTTP API and optional GUI.

**Target:** Windows only (Linux and macOS will be added in future releases).

---

## 1. Project Structure & Tooling

- [x] Cargo workspace with crates: `multi-wallpaper`, `api-models`, `daemon`, `gui`
- [x] Build script for daemon (embed icon, static VCRuntime, Windows resource metadata)
- [x] WiX installer skeleton (`Makefile.toml` tasks for `msi`, `installer`)
- [x] Populate WiX source files to produce a working MSI
- [ ] CI/CD pipeline (build, test, package)
- [ ] Sign executables and installer

---

## 2. Multi‑Wallpaper Library (`multi-wallpaper`)

- [x] `WallpaperManager` trait defining `set`, `get`, `set_style`, `get_style`, `list_monitors`
- [x] Windows implementation via COM `IDesktopWallpaper`
- [x] `WallpaperStyle` enum with `Center`, `Tile`, `Stretch`, `Fit`, `Fill`, `Span`
- [x] Windows COM apartment initialisation (STA) – thread‑local RAII
- [x] Error types (`WallpaperError`, conversions from OS/COM/Io)
- [x] Linux and macOS stubs (return `Unsupported`)
- [ ] Unit tests for Windows implementation (mock COM if possible)

---

## 3. API Models (`api-models`)

- [x] Request/response types: `Monitor`, `CurrentWallpaper`, `SetWallpaperRequest`, `SetStyleRequest`, `StyleResponse`, `MonitorList`
- [x] `WallpaperStyle` serialised/deserialised as lowercase strings
- [ ] Types for image management: `ImageSource` (single file / folder), `ImageList`, `ImageRotationSettings`
- [ ] Types for triggers: `TimeTrigger`, `EventTrigger`, `TriggerConfig`
- [ ] Types for profiles: `Profile`, `CreateProfileRequest`, `UpdateProfileRequest`, `ProfileSummary`
- [ ] Types for daemon status: `AppStatus` (active profile, next change time, etc.)

---

## 4. Daemon Core (`daemon`)

### 4.1 Tray & Process Lifecycle
- [x] System‑tray icon using `winit` / `tray-icon`
- [x] Basic tray menu: “Next wallpaper” (stub), “Open GUI”, “Exit”
- [x] Tray icon embedded from `assets/images/icon.ico` (32×32)
- [ ] Allow toggling startup behaviour (registry `Run` key)
- [ ] Single‑instance enforcement (prevent multiple daemons)
- [ ] Graceful shutdown: save state, remove tray icon explicitly before exit?
- [x] Logging to file

### 4.2 Configuration
- [x] Basic `Config` structure with `gui_path`
- [x] Load from TOML file next to the daemon binary
- [ ] Save config to TOML file
- [ ] Expand `Config` to include:
  - Data directory path (profile/image storage)
  - Default profile
  - Autostart behaviour
- [ ] Config file validation and migration on version change
- [ ] Watch config file for external changes and hot‑reload

### 4.3 HTTP Server (API endpoints)
- [x] `GET /api/health`
- [x] `GET /api/monitors`
- [x] `GET /api/wallpaper?monitor=...`
- [x] `POST /api/wallpaper`
- [x] `GET /api/style`
- [x] `PUT /api/style`
- [x] All wallpaper commands forwarded to main thread via `EventLoopProxy` and `oneshot`
- [ ] Image management endpoints:
  - `POST /api/images` – add image source (file or folder)
  - `DELETE /api/images/:id`
  - `GET /api/images` – list configured images
  - `GET /api/images/next` – get next image (random/sequential)
- [ ] Profile management endpoints:
  - `POST /api/profiles`
  - `GET /api/profiles`
  - `GET /api/profiles/:id`
  - `PUT /api/profiles/:id`
  - `DELETE /api/profiles/:id`
  - `POST /api/profiles/:id/activate`
- [ ] Trigger management endpoints (per profile):
  - `POST /api/profiles/:id/triggers`
  - `DELETE /api/triggers/:trigger_id`
  - `GET /api/profiles/:id/triggers`
- [ ] Endpoint to manually advance to next wallpaper: `POST /api/next`
- [ ] Endpoint to get daemon status: `GET /api/status`

### 4.4 Wallpaper Command Processing
- [x] `WallpaperCommand` enum with `ListMonitors`, `GetWallpaper`, `GetStyle`, `SetStyle`, `SetWallpaper`
- [ ] Add commands for image rotation, profile switching, scheduling, etc.
- [ ] Next wallpaper selection logic (respect per‑monitor rotation settings)

---

## 5. Image Management

- [ ] Scan directories for image files (`.jpg`, `.jpeg`, `.png`, `.bmp`, `.gif`, etc.)
- [ ] Recursive folder scanning (configurable depth? optional)
- [ ] Maintain an in‑memory list of image paths per profile/source
- [ ] Rotation strategies:
  - [ ] Random
  - [ ] Sequential
  - [ ] Ordered by filename/date
- [ ] Remember last displayed image per monitor (to continue sequence)
- [ ] Handle missing / deleted images gracefully (skip, log warning)
- [ ] Image pre‑caching / thumbnail generation? (optional for future)

---

## 6. Scheduling & Triggers

- [ ] Time‑based triggers:
  - [ ] Interval trigger (every N minutes/hours)
  - [ ] Specific time of day (“at 08:00”)
  - [ ] Day‑of‑week / date restrictions
- [ ] Event‑based triggers:
  - [ ] On user logon / session unlock
  - [ ] On application launch (detect specific `.exe` via WMI or polling)
  - [ ] On system idle / resume
  - [ ] On network change / location change? (future)
- [ ] Per‑monitor trigger association (each monitor can have its own triggers)
- [ ] Scheduler engine within the daemon:
  - Timer management (tokio intervals or Windows thread timers)
  - Event hooks (logon notification, process creation watcher)
- [ ] Cooldown / deduplication to avoid rapid changes

---

## 7. Profile System

- [ ] Profile data structure: name, list of image sources, default style, triggers, schedule
- [ ] Active profile tracking (which profile is currently applied)
- [ ] Profile scheduling:
  - [ ] Time range activation (e.g. 09:00–17:00 for “Work” profile)
  - [ ] Day‑of‑week schedules
  - [ ] Conflicts resolution (priority, last match wins)
- [ ] Manual profile switching (from tray menu, API, or GUI)
- [ ] Persistence: save profiles as TOML/JSON files in data directory
- [ ] Migration from old config format (if any)

---

## 8. GUI Application (`gui` crate)

- [ ] Basic `iced` window
- [ ] Communication with daemon via HTTP API (on `127.0.0.1` dynamic port)
- [ ] Screens / tabs:
  - [ ] **Dashboard** – active profile, current wallpapers, next change info
  - [ ] **Profile Editor** – create, edit, delete profiles; set schedules
  - [ ] **Image Sources** – add folders/files, reorder, preview thumbnails
  - [ ] **Trigger Configuration** – add/edit time and event triggers per monitor
  - [ ] **Settings** – daemon behaviour, autostart, logging
- [ ] Open GUI from tray menu (already wired) – ensure port discovery
- [ ] Graceful handling when daemon is not running (show connection error or force start daemon)
- [ ] Minimise to tray? (keep GUI as a separate window that can be closed without affecting daemon)

---

## 9. Installer & Deployment

- [x] Fully populated WiX project with all binaries, assets, and shortcuts
- [x] MSI installs:
  - Daemon executable (registered for auto‑start)
  - GUI executable
  - Uninstall support
  - Start Menu shortcuts (GUI, maybe daemon restart script)
  - File associations? (e.g. `.dwprofile`)
- [ ] Thin executable wrapper around MSI (e.g. `Burn`)
- [ ] Code signing for executables and installer
- [ ] Auto‑updater mechanism?

---

## 10. Documentation

- [x] `README.md` with project overview, features, and build instructions
- [ ] Wiki / User guide (how to install, configure profiles, use the API)
- [ ] API documentation (Swagger/OpenAPI or simple markdown table)
- [ ] Developer guide (architecture, how to add a new platform, how the event loop works)
- [ ] Changelog

---

## 11. Polish & Testing

- [ ] Error handling audit: all unwraps/expects replaced with proper error propagation
- [ ] Consistent logging (`tracing`)
- [ ] Integration tests for API endpoints
- [ ] Stress testing (many images, rapid profile switches)
- [ ] Memory/performance profiling (idle CPU usage)
- [ ] Windows 10 and 11 compatibility checks
- [ ] Multi‑DPI / high‑DPI monitor support
- [ ] Accessibility (keyboard navigation in GUI)

---

**Next immediate actions:**  
1. Define the extended configuration and API models (image sources, profiles, triggers).  
2. Implement image folder scanning and next‑image rotation logic.  
3. Add profile scheduling and trigger engine.  
4. Extend the HTTP API with the new endpoints.  
5. Build a minimal GUI for profile management.
