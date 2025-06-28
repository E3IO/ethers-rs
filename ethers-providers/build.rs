use cfg_aliases::cfg_aliases;

fn main() {
    // 告知 cargo 我们使用的自定义 cfg 条件
    println!("cargo:rustc-check-cfg=cfg(wasm)");
    println!("cargo:rustc-check-cfg=cfg(native)");
    println!("cargo:rustc-check-cfg=cfg(unix_like)");
    println!("cargo:rustc-check-cfg=cfg(has_ws)");
    println!("cargo:rustc-check-cfg=cfg(has_legacy_ws)");
    println!("cargo:rustc-check-cfg=cfg(has_ipc)");
    println!("cargo:rustc-check-cfg=cfg(has_rustls)");
    println!("cargo:rustc-check-cfg=cfg(has_openssl)");
    println!("cargo:rustc-check-cfg=cfg(has_celo)");
    println!("cargo:rustc-check-cfg=cfg(has_optimism)");
    println!("cargo:rustc-check-cfg=cfg(has_legacy)");
    println!("cargo:rustc-check-cfg=cfg(has_tls)");

    cfg_aliases! {
        // 平台检测
        wasm: { target_arch = "wasm32" },
        native: { not(target_arch = "wasm32") },
        unix_like: { any(target_os = "linux", target_os = "macos", target_os = "freebsd", target_os = "openbsd", target_os = "netbsd") },

        // 传输特性
        has_ws: { feature = "ws" },
        has_legacy_ws: { feature = "legacy-ws" },
        has_ipc: { feature = "ipc" },

        // TLS 特性
        has_rustls: { feature = "rustls" },
        has_openssl: { feature = "openssl" },

        // 链特性
        has_celo: { feature = "celo" },
        has_optimism: { feature = "optimism" },
        has_legacy: { feature = "legacy" },

        // 组合特性
        has_tls: { any(has_rustls, has_openssl) },
    }
}
