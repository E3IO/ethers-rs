#!/bin/bash

# Ethers-rs 快速发布脚本
# 作者：dark (web3自由职业者联盟)

set -e

echo "🚀 Ethers-rs 快速发布脚本"
echo "=========================="

# 获取版本
VERSION=$(grep '^version = ' Cargo.toml | head -1 | sed 's/version = "\(.*\)"/\1/')
echo "📦 当前版本: $VERSION"

# 发布顺序
PACKAGES=(
    "ethers-core"
    "ethers-contract-abigen" 
    "ethers-contract-derive"
    "ethers-providers"
    "ethers-signers"
    "ethers-solc"
    "ethers-etherscan"
    "ethers-addressbook"
    "ethers-middleware"
    "ethers-contract"
    "ethers"
)

echo "📋 将按顺序发布 ${#PACKAGES[@]} 个包"

# 确认
read -p "确认发布到 crates.io？(y/N): " -n 1 -r
echo
if [[ ! $REPLY =~ ^[Yy]$ ]]; then
    echo "❌ 发布已取消"
    exit 0
fi

# 快速检查
echo "🔍 检查构建状态..."
cargo check --workspace --quiet

echo "🚀 开始发布..."

# 发布每个包
for i in "${!PACKAGES[@]}"; do
    package="${PACKAGES[$i]}"
    echo "[$((i+1))/${#PACKAGES[@]}] 发布 $package..."
    
    if cargo publish -p "$package" --quiet; then
        echo "✅ $package 发布成功"
        if [[ $i -lt $((${#PACKAGES[@]} - 1)) ]]; then
            echo "⏳ 等待 20 秒..."
            sleep 20
        fi
    else
        echo "❌ $package 发布失败"
        exit 1
    fi
done

echo
echo "🎉 所有包发布完成！"
echo "🔗 查看: https://crates.io/crates/ethers" 