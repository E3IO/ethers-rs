#!/bin/bash

# Ethers-rs 发布脚本
# 作者：dark (web3自由职业者联盟)
# 按照依赖关系顺序发布所有包到 crates.io

set -euo pipefail

# 颜色定义
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# 日志函数
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

# 检查是否已登录 crates.io
check_cargo_login() {
    log_info "检查 cargo 登录状态..."
    if ! cargo owner --list ethers-core &>/dev/null; then
        log_error "未登录 crates.io，请先运行: cargo login"
        exit 1
    fi
    log_success "已登录 crates.io"
}

# 检查工作目录是否干净
check_git_status() {
    log_info "检查 git 状态..."
    if [[ -n $(git status --porcelain) ]]; then
        log_warning "工作目录有未提交的更改"
        echo "未提交的文件："
        git status --short
        read -p "是否继续发布？(y/N): " -n 1 -r
        echo
        if [[ ! $REPLY =~ ^[Yy]$ ]]; then
            log_info "发布已取消"
            exit 0
        fi
    fi
    log_success "Git 状态检查通过"
}

# 运行测试
run_tests() {
    log_info "运行测试套件..."
    if ! cargo test --workspace --quiet; then
        log_error "测试失败，请修复后再发布"
        exit 1
    fi
    log_success "所有测试通过"
}

# 检查构建
check_build() {
    log_info "检查构建状态..."
    if ! cargo check --workspace --quiet; then
        log_error "构建失败，请修复后再发布"
        exit 1
    fi
    log_success "构建检查通过"
}

# 发布单个包
publish_package() {
    local package=$1
    local retry_count=0
    local max_retries=3
    
    log_info "发布包: $package"
    
    # 先进行干运行检查
    log_info "对 $package 进行干运行检查..."
    if ! cargo publish --dry-run -p "$package" --quiet; then
        log_error "$package 干运行失败"
        return 1
    fi
    
    # 实际发布
    while [ $retry_count -lt $max_retries ]; do
        if cargo publish -p "$package" --quiet; then
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
    local timeout=300  # 5分钟超时
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

# 获取当前版本
get_version() {
    grep '^version.workspace = true' ethers-core/Cargo.toml > /dev/null
    if [ $? -eq 0 ]; then
        # 使用工作空间版本
        grep '^version = ' Cargo.toml | head -1 | sed 's/version = "\(.*\)"/\1/'
    else
        # 从 ethers-core 获取版本
        grep '^version = ' ethers-core/Cargo.toml | sed 's/version = "\(.*\)"/\1/'
    fi
}

# 主发布流程
main() {
    log_info "开始 Ethers-rs 发布流程"
    
    # 获取版本信息
    VERSION=$(get_version)
    log_info "当前版本: $VERSION"
    
    # 确认发布
    echo
    log_warning "即将发布 ethers-rs v$VERSION 到 crates.io"
    read -p "确认继续？(y/N): " -n 1 -r
    echo
    if [[ ! $REPLY =~ ^[Yy]$ ]]; then
        log_info "发布已取消"
        exit 0
    fi
    
    # 预检查
    check_cargo_login
    check_git_status
    check_build
    
    # 可选：运行测试（可能很慢）
    read -p "是否运行完整测试套件？(y/N): " -n 1 -r
    echo
    if [[ $REPLY =~ ^[Yy]$ ]]; then
        run_tests
    fi
    
    # 定义发布顺序（按依赖关系）
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
    for package in "${PACKAGES[@]}"; do
        echo "----------------------------------------"
        
        if publish_package "$package"; then
            # 对于基础包，等待其在 crates.io 上可用
            if [[ "$package" == "ethers-core" || "$package" == "ethers-contract-abigen" ]]; then
                wait_for_package "$package" "$VERSION"
            else
                # 其他包等待较短时间
                log_info "等待 30 秒让 crates.io 更新索引..."
                sleep 30
            fi
        else
            log_error "包 $package 发布失败，停止发布流程"
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

# 脚本帮助信息
show_help() {
    echo "Ethers-rs 发布脚本"
    echo
    echo "用法:"
    echo "  $0                 # 交互式发布所有包"
    echo "  $0 --help         # 显示此帮助信息"
    echo "  $0 --dry-run      # 仅进行干运行检查"
    echo
    echo "环境要求:"
    echo "  - 已登录 crates.io (cargo login)"
    echo "  - Git 工作目录干净"
    echo "  - 所有测试通过"
    echo
    echo "发布顺序:"
    echo "  1. ethers-core"
    echo "  2. ethers-contract-abigen"
    echo "  3. ethers-contract-derive"
    echo "  4. ethers-providers"
    echo "  5. ethers-signers"
    echo "  6. ethers-solc"
    echo "  7. ethers-etherscan"
    echo "  8. ethers-addressbook"
    echo "  9. ethers-middleware"
    echo "  10. ethers-contract"
    echo "  11. ethers"
}

# 仅干运行
dry_run() {
    log_info "执行干运行检查..."
    
    VERSION=$(get_version)
    log_info "当前版本: $VERSION"
    
    check_build
    
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
    
    log_info "检查所有包的发布准备状态..."
    
    for package in "${PACKAGES[@]}"; do
        log_info "检查 $package..."
        if cargo publish --dry-run -p "$package" --quiet; then
            log_success "$package 准备就绪"
        else
            log_error "$package 检查失败"
            exit 1
        fi
    done
    
    log_success "所有包都准备好发布！"
}

# 处理命令行参数
case "${1:-}" in
    --help|-h)
        show_help
        exit 0
        ;;
    --dry-run)
        dry_run
        exit 0
        ;;
    "")
        main
        ;;
    *)
        log_error "未知参数: $1"
        show_help
        exit 1
        ;;
esac 