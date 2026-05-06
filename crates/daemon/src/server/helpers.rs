use deskwarp_api_models::WallpaperStyle as ApiWallpaperStyle;
use multi_wallpaper::WallpaperStyle as MultiWallpaperStyle;

pub fn into_multi_style(api: ApiWallpaperStyle) -> MultiWallpaperStyle {
    match api {
        ApiWallpaperStyle::Center => MultiWallpaperStyle::Center,
        ApiWallpaperStyle::Tile => MultiWallpaperStyle::Tile,
        ApiWallpaperStyle::Stretch => MultiWallpaperStyle::Stretch,
        ApiWallpaperStyle::Fit => MultiWallpaperStyle::Fit,
        ApiWallpaperStyle::Fill => MultiWallpaperStyle::Fill,
        ApiWallpaperStyle::Span => MultiWallpaperStyle::Span,
    }
}

pub fn into_api_style(multi: MultiWallpaperStyle) -> ApiWallpaperStyle {
    match multi {
        MultiWallpaperStyle::Center => ApiWallpaperStyle::Center,
        MultiWallpaperStyle::Tile => ApiWallpaperStyle::Tile,
        MultiWallpaperStyle::Stretch => ApiWallpaperStyle::Stretch,
        MultiWallpaperStyle::Fit => ApiWallpaperStyle::Fit,
        MultiWallpaperStyle::Fill => ApiWallpaperStyle::Fill,
        MultiWallpaperStyle::Span => ApiWallpaperStyle::Span,
    }
}
