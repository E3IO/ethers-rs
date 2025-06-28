//! Platform compatibility utilities
//!
//! This module provides utilities and type definitions that help handle
//! differences between WASM and native platforms.

/// Platform-specific re-exports and utilities
pub mod platform {
    #[cfg(native)]
    pub mod native {
        //! Native platform utilities

        /// Re-export common native-only types
        pub use std::net::{TcpListener, TcpStream};
        pub use std::process::{Child, Command};

        /// Check if we're running on a native platform
        pub const fn is_native() -> bool {
            true
        }

        /// Check if we're running on WASM
        pub const fn is_wasm() -> bool {
            false
        }
    }

    #[cfg(wasm)]
    pub mod wasm {
        //! WASM platform utilities

        /// Check if we're running on a native platform  
        pub const fn is_native() -> bool {
            false
        }

        /// Check if we're running on WASM
        pub const fn is_wasm() -> bool {
            true
        }

        /// Placeholder for native-only functionality in WASM
        pub fn native_only_error(feature: &str) -> ! {
            panic!("{} is not available in WASM environment", feature);
        }
    }

    // Re-export the current platform's utilities
    #[cfg(native)]
    pub use native::*;

    #[cfg(wasm)]
    pub use wasm::*;
}

/// Error type for platform compatibility issues
#[derive(Debug, thiserror::Error)]
pub enum CompatError {
    #[error("Feature '{feature}' is not available on WASM")]
    WasmUnsupported { feature: String },

    #[error("Feature '{feature}' is not available on native platforms")]
    NativeUnsupported { feature: String },
}

/// Result type for platform-specific operations
pub type CompatResult<T> = Result<T, CompatError>;
