//! Feature flag management and compatibility utilities
//!
//! This module provides utilities for managing feature flags and ensuring
//! compatibility across different configurations.

/// Chain-specific feature detection
pub mod chain {
    /// Check if Celo support is enabled
    pub const fn is_celo_enabled() -> bool {
        cfg!(has_celo)
    }

    /// Check if Optimism support is enabled
    pub const fn is_optimism_enabled() -> bool {
        cfg!(has_optimism)
    }

    /// Check if legacy transaction support is enabled
    pub const fn is_legacy_enabled() -> bool {
        cfg!(has_legacy)
    }

    /// Get the current chain configuration as a string
    pub fn chain_config() -> &'static str {
        if is_celo_enabled() {
            "celo"
        } else if is_optimism_enabled() {
            "optimism"
        } else {
            "ethereum"
        }
    }
}

/// Platform-specific feature detection
pub mod platform {
    /// Check if we're running on WASM
    pub const fn is_wasm() -> bool {
        cfg!(wasm)
    }

    /// Check if we're running on a native platform
    pub const fn is_native() -> bool {
        cfg!(native)
    }

    /// Check if we're on a Unix-like system
    pub const fn is_unix_like() -> bool {
        cfg!(unix_like)
    }

    /// Check if we're on Windows
    pub const fn is_windows() -> bool {
        cfg!(windows)
    }
}

/// Macro support detection
pub mod macros {
    /// Check if macro support is enabled
    pub const fn is_enabled() -> bool {
        cfg!(has_macros)
    }
}

/// Compile-time feature validation
///
/// This function performs compile-time checks to ensure feature combinations
/// are valid. It should be called early in the application lifecycle.
pub const fn validate_features() -> Result<(), &'static str> {
    // These checks are already done at compile time via compile_error!
    // but we provide runtime validation as well
    if chain::is_celo_enabled() && chain::is_optimism_enabled() {
        return Err("Celo and Optimism features are mutually exclusive");
    }

    Ok(())
}

/// Get a summary of enabled features
pub fn feature_summary() -> String {
    let mut features = Vec::new();

    features.push(format!("platform: {}", if platform::is_wasm() { "wasm" } else { "native" }));

    features.push(format!("chain: {}", chain::chain_config()));

    if macros::is_enabled() {
        features.push("macros: enabled".to_string());
    }

    format!("ethers-rs features: [{}]", features.join(", "))
}
