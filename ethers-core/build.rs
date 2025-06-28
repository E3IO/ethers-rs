use cfg_aliases::cfg_aliases;

fn main() {
    // 告知 cargo 我们使用的自定义 cfg 条件
    println!("cargo:rustc-check-cfg=cfg(wasm)");
    println!("cargo:rustc-check-cfg=cfg(native)");
    println!("cargo:rustc-check-cfg=cfg(unix_like)");
    println!("cargo:rustc-check-cfg=cfg(has_celo)");
    println!("cargo:rustc-check-cfg=cfg(has_optimism)");
    println!("cargo:rustc-check-cfg=cfg(has_legacy)");
    println!("cargo:rustc-check-cfg=cfg(has_macros)");

    cfg_aliases! {
        // 平台检测
        wasm: { target_arch = "wasm32" },
        native: { not(target_arch = "wasm32") },
        unix_like: { any(target_os = "linux", target_os = "macos", target_os = "freebsd", target_os = "openbsd", target_os = "netbsd") },

        // 链特性
        has_celo: { feature = "celo" },
        has_optimism: { feature = "optimism" },
        has_legacy: { feature = "legacy" },

        // 宏特性
        has_macros: { feature = "macros" },
    }
}
