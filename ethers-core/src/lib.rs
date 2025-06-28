#![doc = include_str!("../README.md")]
#![deny(rustdoc::broken_intra_doc_links)]
#![cfg_attr(not(target_arch = "wasm32"), deny(unused_crate_dependencies))]
#![cfg_attr(docsrs, feature(doc_cfg, doc_auto_cfg))]

// 编译时检查互斥特性（测试时跳过，因为 --all-features 会同时启用所有特性）
#[cfg(all(feature = "celo", feature = "optimism", not(test)))]
compile_error!("features 'celo' and 'optimism' are mutually exclusive");

pub mod types;

pub mod abi;

/// Various utilities
pub mod utils;

/// Platform compatibility utilities
pub mod compat;

/// Feature flag management and compatibility utilities
pub mod features;

#[cfg(feature = "macros")]
pub mod macros;

// re-export rand to avoid potential confusion when there's rand version mismatches
pub use rand;

// re-export k256
pub use k256;
