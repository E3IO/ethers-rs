use cfg_aliases::cfg_aliases;

fn main() {
    cfg_aliases! {
        // 平台检测
        wasm: { target_arch = "wasm32" },
        native: { not(target_arch = "wasm32") },

        // 核心特性
        has_providers: { feature = "providers" },
        has_abigen: { feature = "abigen" },
        has_abigen_online: { feature = "abigen-online" },

        // TLS 特性
        has_rustls: { feature = "rustls" },
        has_openssl: { feature = "openssl" },

        // 链特性
        has_celo: { feature = "celo" },
        has_optimism: { feature = "optimism" },
        has_legacy: { feature = "legacy" },

        // 组合特性
        has_tls: { any(has_rustls, has_openssl) },
        has_codegen: { any(has_abigen, has_abigen_online) },
    }
}
