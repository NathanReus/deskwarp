// These integration tests only run on Windows because the crate's functionality
// is currently Windows-only. They test the public API from a user's perspective.
#![cfg(target_os = "windows")]

use multi_wallpaper::{Wallpaper, WallpaperError, WallpaperStyle};
use serial_test::serial;
use std::path::PathBuf;

/// Create a minimal 1x1 red BMP file in a temporary directory.
/// Returns the path to the file. The file is only created once per directory.
fn create_test_image(blue: u8, green: u8, red: u8) -> PathBuf {
    let dir = std::env::temp_dir().join("multi_wallpaper_tests");
    std::fs::create_dir_all(&dir).unwrap();
    let path = dir.join(format!("test_image_{}_{}_{}.bmp", blue, green, red));
    println!("Creating test image: {:?}", path);
    if !path.exists() {
        // A valid 1x1 24-bit BMP (58 bytes)
        let bmp_data: [u8; 58] = [
            // BMP file header (14 bytes)
            0x42, 0x4D, // "BM"
            0x3A, 0x00, 0x00, 0x00, // file size (58)
            0x00, 0x00, // reserved
            0x00, 0x00, // reserved
            0x36, 0x00, 0x00, 0x00, // offset to pixel data (54)
            // DIB header (40 bytes)
            0x28, 0x00, 0x00, 0x00, // header size (40)
            0x01, 0x00, 0x00, 0x00, // width (1)
            0x01, 0x00, 0x00, 0x00, // height (1)
            0x01, 0x00, // colour planes (1)
            0x18, 0x00, // bits per pixel (24)
            0x00, 0x00, 0x00, 0x00, // compression (none)
            0x04, 0x00, 0x00, 0x00, // image size (4)
            0x00, 0x00, 0x00, 0x00, // x pixels per meter
            0x00, 0x00, 0x00, 0x00, // y pixels per meter
            0x00, 0x00, 0x00, 0x00, // colour in palette
            0x00, 0x00, 0x00, 0x00, // important colours
            // Pixel data (blue, green, red) + 1 byte padding
            blue, green, red,  // pixel colour
            0x00, // row padding
        ];
        std::fs::write(&path, &bmp_data).unwrap();
    }
    path
}

// --------------------------------------------------------------------
// Constructor & monitor info
// --------------------------------------------------------------------

#[test]
#[serial]
fn test_create_manager() {
    let _wp = Wallpaper::new().expect("Wallpaper::new() should succeed");
}

#[test]
#[serial]
fn test_list_monitors_returns_ids() {
    let wp = Wallpaper::new().expect("Wallpaper::new() should succeed");
    let monitors = wp.list_monitors().unwrap();
    assert!(
        !monitors.is_empty(),
        "At least one monitor should be present"
    );
    for id in &monitors {
        assert!(!id.is_empty(), "Monitor ID should not be empty");
    }
}

// --------------------------------------------------------------------
// Set / Get wallpaper for all monitors
// --------------------------------------------------------------------

#[test]
#[serial]
fn test_set_and_get_wallpaper_all_monitors() {
    let image = create_test_image(255, 0, 0);
    let wp = Wallpaper::new().expect("Wallpaper::new() should succeed");

    wp.set(&image, WallpaperStyle::Fill, None).unwrap();
    let got = wp.get(None).unwrap();
    assert_eq!(got, Some(image));
}

#[test]
#[serial]
fn test_set_wallpaper_for_specific_monitor() {
    let image = create_test_image(0, 255, 0);
    let wp = Wallpaper::new().expect("Wallpaper::new() should succeed");
    let monitors = wp.list_monitors().unwrap();
    if monitors.is_empty() {
        return; // Skip if no monitors (should not happen)
    }
    let monitor_id = &monitors[0];

    wp.set(&image, WallpaperStyle::Stretch, Some(monitor_id))
        .unwrap();
    let got = wp.get(Some(monitor_id)).unwrap();
    assert_eq!(got, Some(image));
}

// --------------------------------------------------------------------
// Style manipulation
// --------------------------------------------------------------------

#[test]
#[serial]
fn test_set_and_get_style() {
    let image = create_test_image(0, 0, 255);
    let wp = Wallpaper::new().expect("Wallpaper::new() should succeed");

    // Set image with Span style
    wp.set(&image, WallpaperStyle::Fill, None).unwrap();
    assert_eq!(wp.get_style().unwrap(), WallpaperStyle::Fill);

    // Change style without touching the image
    wp.set_style(WallpaperStyle::Fit).unwrap();
    assert_eq!(wp.get_style().unwrap(), WallpaperStyle::Fit);
}

// --------------------------------------------------------------------
// Error cases
// --------------------------------------------------------------------

#[test]
#[serial]
fn test_set_nonexistent_file_returns_io_error() {
    let wp = Wallpaper::new().expect("Wallpaper::new() should succeed");
    let bad_path = std::env::current_dir()
        .unwrap()
        .join("fake-picture-does-not-exist.jpg");
    let result = wp.set(&bad_path, WallpaperStyle::Fill, None);
    assert!(result.is_err());
    match result.unwrap_err() {
        WallpaperError::Io(_) => {} // expected
        WallpaperError::Os(_) => {} // expected
        other => panic!("Expected Io or Os error, got {:?}", other),
    }
}

#[test]
#[serial]
fn test_invalid_monitor_id_returns_invalid_monitor_error() {
    let image = create_test_image(255, 255, 0);
    let wp = Wallpaper::new().expect("Wallpaper::new() should succeed");
    let result = wp.set(
        &image,
        WallpaperStyle::Fill,
        Some(std::ffi::OsStr::new("INVALID_GUID")),
    );
    assert!(result.is_err());
    if let WallpaperError::InvalidMonitor(id) = result.unwrap_err() {
        assert_eq!(id, "INVALID_GUID");
    } else {
        panic!("Expected InvalidMonitor error");
    }
}
