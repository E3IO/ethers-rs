use cfg_aliases::cfg_aliases;

fn main() {
    cfg_aliases! {
        // 平台检测
        wasm: { target_arch = "wasm32" },
        native: { not(target_arch = "wasm32") },

        // TLS 特性
        has_rustls: { feature = "rustls" },
        has_openssl: { feature = "openssl" },

        // 链特性
        has_celo: { feature = "celo" },
        has_optimism: { feature = "optimism" },

        // 外部服务特性
        has_etherscan: { feature = "etherscan" },

        // 组合特性
        has_tls: { any(has_rustls, has_openssl) },
    }
}
