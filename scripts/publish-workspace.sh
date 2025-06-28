#!/bin/bash

# Ethers-rs 工作空间发布脚本
# 作者：dark (web3自由职业者联盟)
# 解决工作空间依赖版本问题

set -euo pipefail

# 颜色定义
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m'

log_info() {
    echo -e "${BLUE}[INFO]${NC} $1"
}

log_success() {
    echo -e "${GREEN}[SUCCESS]${NC} $1"
}

log_warning() {
    echo -e "${YELLOW}[WARNING]${NC} $1"
}

log_error() {
    echo -e "${RED}[ERROR]${NC} $1"
}

# 获取当前版本
get_version() {
    grep '^version = ' Cargo.toml | head -1 | sed 's/version = "\(.*\)"/\1/'
}

# 发布单个包（使用 --allow-dirty 绕过依赖检查）
publish_package() {
    local package=$1
    local retry_count=0
    local max_retries=3
    
    log_info "发布包: $package"
    
    while [ $retry_count -lt $max_retries ]; do
        if cargo publish -p "$package" --allow-dirty; then
            log_success "$package 发布成功"
            return 0
        else
            retry_count=$((retry_count + 1))
            log_warning "$package 发布失败，重试 $retry_count/$max_retries"
            if [ $retry_count -lt $max_retries ]; then
                log_info "等待 30 秒后重试..."
                sleep 30
            fi
        fi
    done
    
    log_error "$package 发布失败，已重试 $max_retries 次"
    return 1
}

# 等待包在 crates.io 上可用
wait_for_package() {
    local package=$1
    local version=$2
    local timeout=300
    local elapsed=0
    
    log_info "等待 $package-$version 在 crates.io 上可用..."
    
    while [ $elapsed -lt $timeout ]; do
        if cargo search "$package" --limit 1 | grep -q "$package.*$version"; then
            log_success "$package-$version 已在 crates.io 上可用"
            return 0
        fi
        
        sleep 10
        elapsed=$((elapsed + 10))
        echo -n "."
    done
    
    echo
    log_warning "$package-$version 在 $timeout 秒内未在 crates.io 上可用，但继续发布"
    return 0
}

main() {
    log_info "开始 Ethers-rs 工作空间发布流程"
    
    VERSION=$(get_version)
    log_info "当前版本: $VERSION"
    
    # 确认发布
    echo
    log_warning "即将发布 ethers-rs v$VERSION 到 crates.io"
    log_warning "注意：将使用 --allow-dirty 标志绕过依赖版本检查"
    read -p "确认继续？(y/N): " -n 1 -r
    echo
    if [[ ! $REPLY =~ ^[Yy]$ ]]; then
        log_info "发布已取消"
        exit 0
    fi
    
    # 检查构建
    log_info "检查构建状态..."
    if ! cargo check --workspace --quiet; then
        log_error "构建失败，请修复后再发布"
        exit 1
    fi
    log_success "构建检查通过"
    
    # 定义发布顺序
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
    
    log_info "开始按顺序发布 ${#PACKAGES[@]} 个包..."
    echo
    
    # 发布每个包
    for i in "${!PACKAGES[@]}"; do
        package="${PACKAGES[$i]}"
        echo "----------------------------------------"
        log_info "[$((i+1))/${#PACKAGES[@]}] 发布 $package"
        
        if publish_package "$package"; then
            # 对于关键依赖包，等待其在 crates.io 上可用
            if [[ "$package" == "ethers-core" ]]; then
                wait_for_package "$package" "$VERSION"
            else
                # 其他包等待较短时间
                log_info "等待 30 秒让 crates.io 更新索引..."
                sleep 30
            fi
        else
            log_error "包 $package 发布失败"
            echo
            log_info "你可以手动发布剩余的包："
            for j in $(seq $i $((${#PACKAGES[@]} - 1))); do
                echo "  cargo publish -p ${PACKAGES[$j]} --allow-dirty"
            done
            exit 1
        fi
        
        echo
    done
    
    echo "========================================"
    log_success "🎉 所有包发布成功！"
    log_info "发布的包："
    for package in "${PACKAGES[@]}"; do
        echo "  ✓ $package v$VERSION"
    done
    
    echo
    log_info "验证发布："
    echo "  cargo search ethers"
    echo "  https://crates.io/crates/ethers"
    
    log_success "Ethers-rs v$VERSION 发布完成！"
}

# 处理命令行参数
case "${1:-}" in
    --help|-h)
        echo "Ethers-rs 工作空间发布脚本"
        echo
        echo "用法:"
        echo "  $0                 # 发布所有包"
        echo "  $0 --help         # 显示此帮助信息"
        echo
        echo "特性:"
        echo "  - 使用 --allow-dirty 绕过依赖版本检查"
        echo "  - 按正确顺序发布包"
        echo "  - 自动重试机制"
        echo "  - 等待包索引更新"
        exit 0
        ;;
    "")
        main
        ;;
    *)
        log_error "未知参数: $1"
        exit 1
        ;;
esac 