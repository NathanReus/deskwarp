use std::cell::RefCell;
use windows::Win32::Foundation::{S_FALSE, S_OK};
use windows::Win32::System::Com::{
    COINIT_APARTMENTTHREADED, COINIT_DISABLE_OLE1DDE, CoInitializeEx, CoUninitialize,
};

use crate::error::WallpaperError;

/// Represents an apartment (thread) that has been initialised for COM usage.
///
/// This is a wrapper around the COM initialization API, providing a convenient
/// RAII interface for managing the apartment's lifecycle.
struct ComApartment {
    owns_init: bool,
}

impl ComApartment {
    pub fn new() -> Result<Self, WallpaperError> {
        let hr = unsafe { CoInitializeEx(None, COINIT_APARTMENTTHREADED | COINIT_DISABLE_OLE1DDE) };

        match hr {
            S_OK => Ok(Self { owns_init: true }),
            S_FALSE => Ok(Self { owns_init: false }), // already initialised on this thread
            _ => Err(WallpaperError::Os(format!("CoInitializeEx failed: {}", hr))),
        }
    }
}

/// Drops the COM apartment, uninitialising it if it owns the initialisation.
impl Drop for ComApartment {
    fn drop(&mut self) {
        if self.owns_init {
            unsafe {
                CoUninitialize();
            }
        }
    }
}

thread_local! {
    static COM: RefCell<Option<ComApartment>> = RefCell::new(None);
}

/// Ensures that the calling thread has a COM apartment (STA) initialised.
fn ensure_sta() -> Result<(), WallpaperError> {
    COM.with(|cell| {
        let mut slot = cell.borrow_mut();

        if slot.is_none() {
            *slot = Some(ComApartment::new()?);
        }

        Ok(())
    })
}

/// Initialises COM for the calling thread (STA apartment).
///
/// This is a convenience wrapper around `ensure_sta` that provides a `new`
/// method for use with the `Com` struct.
pub struct Com;

impl Com {
    /// Initialises COM for the calling thread (STA apartment).
    ///
    /// This is a convenience wrapper around `ensure_sta` that provides a `new`
    /// method for use with the `Com` struct.
    pub fn new() -> Result<Self, WallpaperError> {
        ensure_sta()?;
        Ok(Self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_com_init_succeeds() {
        let result = Com::new();
        assert!(result.is_ok());
    }

    #[test]
    fn test_com_init_idempotent() {
        let result1 = Com::new();
        let result2 = Com::new();
        assert!(result1.is_ok());
        assert!(result2.is_ok());
    }
}
