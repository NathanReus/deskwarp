use ico::IconDir;
use std::{env, fs, io::Cursor, path::PathBuf};

fn main() {
    // Extract RGBA from ICO for tray icon
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    let icon_path = PathBuf::from("assets/images/icon.ico");
    let icon_bytes =
        fs::read(&icon_path).expect("Failed to read icon.ico for tray icon extraction");
    let icon_dir = IconDir::read(Cursor::new(&icon_bytes)).expect("Failed to parse icon.ico");

    let entry = icon_dir
        .entries()
        .iter()
        .find(|e| e.width() == 32 && e.height() == 32)
        .expect("ICO file contains no entry for 32x32 icon");

    let rgba_image = entry.decode().expect("Failed to decode 32x32 icon");
    let rgba_bytes = rgba_image.rgba_data();

    fs::write(out_dir.join("icon_32_rgba.bin"), rgba_bytes)
        .expect("Failed to write icon_32_rgba.bin");

    // Windows resource embedding
    #[cfg(target_os = "windows")]
    {
        let mut res = tauri_winres::WindowsResource::new();
        res.set_icon("assets/images/icon.ico");
        res.set("ProductName", "DeskWarp");
        res.set("FileDescription", "DeskWarp");
        res.set("LegalCopyright", "© 2026 Nathan Reus");
        res.set("OriginalFilename", "deskwarp-daemon.exe");
        res.set("InternalName", "deskwarp-daemon");
        res.set("FileVersion", env!("CARGO_PKG_VERSION"));
        res.set("ProductVersion", env!("CARGO_PKG_VERSION"));

        // The manifest is needed for proper DPI awareness, etc.
        res.set_manifest_file("build/win_manifest.xml");

        if let Err(e) = res.compile() {
            eprintln!("windows resource compilation failed: {e}");
            std::process::exit(1);
        }
    }
}
