mod ethers_crate;
pub use ethers_crate::*;

/// 简化 async_trait 在不同平台上的使用
/// 在 WASM 上使用 ?Send，在其他平台上正常使用
#[macro_export]
macro_rules! async_trait_maybe_send {
    ($($tt:tt)*) => {
        #[cfg_attr(target_arch = "wasm32", async_trait::async_trait(?Send))]
        #[cfg_attr(not(target_arch = "wasm32"), async_trait::async_trait)]
        $($tt)*
    };
}
