use cfg_aliases::cfg_aliases;

fn main() {
    // 告知 cargo 我们使用的自定义 cfg 条件
    println!("cargo:rustc-check-cfg=cfg(wasm)");
    println!("cargo:rustc-check-cfg=cfg(native)");
    println!("cargo:rustc-check-cfg=cfg(unix_like)");
    println!("cargo:rustc-check-cfg=cfg(hw_ledger)");
    println!("cargo:rustc-check-cfg=cfg(hw_trezor)");
    println!("cargo:rustc-check-cfg=cfg(hw_yubi)");
    println!("cargo:rustc-check-cfg=cfg(cloud_aws)");
    println!("cargo:rustc-check-cfg=cfg(any_hw)");
    println!("cargo:rustc-check-cfg=cfg(has_celo)");
    println!("cargo:rustc-check-cfg=cfg(has_optimism)");

    cfg_aliases! {
        // 平台检测
        wasm: { target_arch = "wasm32" },
        native: { not(target_arch = "wasm32") },
        unix_like: { any(target_os = "linux", target_os = "macos", target_os = "freebsd", target_os = "openbsd", target_os = "netbsd") },

        // 硬件钱包特性
        hw_ledger: { all(feature = "ledger", native) },
        hw_trezor: { all(feature = "trezor", native) },
        hw_yubi: { all(feature = "yubihsm", native) },

        // 云签名器特性
        cloud_aws: { all(feature = "aws", native) },

        // 链特性
        has_celo: { feature = "celo" },
        has_optimism: { feature = "optimism" },

        // 组合特性
        any_hw: { any(hw_ledger, hw_trezor, hw_yubi) },
    }
}
