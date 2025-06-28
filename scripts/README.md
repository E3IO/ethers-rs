# Ethers-rs 发布脚本

这个目录包含用于发布 ethers-rs 包到 crates.io 的脚本。

## 脚本说明

### 🚀 `publish-workspace.sh` - 工作空间发布脚本（推荐）

**专门为工作空间设计的发布脚本**，解决依赖版本问题：
- 使用 `--allow-dirty` 绕过依赖版本检查
- 按正确顺序发布包
- 自动重试机制
- 等待包索引更新

```bash
# 查看帮助
./scripts/publish-workspace.sh --help

# 发布所有包
./scripts/publish-workspace.sh
```

### 🔧 `publish.sh` - 完整发布脚本

功能最全面的发布脚本，包含：
- 完整的预检查（git状态、构建、测试）
- 登录状态验证
- 干运行检查
- 重试机制
- 等待包索引更新
- 详细的日志输出

```bash
# 查看帮助
./scripts/publish.sh --help

# 干运行检查（推荐先运行）
./scripts/publish.sh --dry-run

# 完整发布流程
./scripts/publish.sh
```

### ⚡ `quick-publish.sh` - 快速发布脚本

简化版本，适合熟悉流程的用户：
- 基本检查
- 快速发布
- 简洁输出

```bash
./scripts/quick-publish.sh
```

## 推荐发布流程

### 方案一：使用工作空间脚本（推荐）

```bash
# 1. 更新版本号
# 编辑 Cargo.toml 中的 [workspace.package] version

# 2. 检查构建
cargo check --workspace

# 3. 提交更改
git add .
git commit -m "准备发布 v2.0.15"

# 4. 发布
./scripts/publish-workspace.sh
```

### 方案二：使用完整脚本

```bash
# 1. 干运行检查
./scripts/publish.sh --dry-run

# 2. 如果干运行成功，进行实际发布
./scripts/publish.sh
```

## 发布前准备

### 1. 登录 crates.io
```bash
cargo login
```

### 2. 确保代码状态良好
```bash
# 检查构建
cargo check --workspace

# 运行测试（可选）
cargo test --workspace

# 提交所有更改
git add .
git commit -m "准备发布 v2.0.15"
git push
```

### 3. 更新版本号

编辑 `Cargo.toml` 中的工作空间版本：
```toml
[workspace.package]
version = "2.0.15"  # 更新版本号
```

同时更新工作空间依赖中的版本：
```toml
[workspace.dependencies]
ethers-core = { version = "2.0.15", path = "ethers-core", default-features = false }
# ... 其他包也要更新
```

## 发布顺序

脚本会按照以下依赖关系顺序发布包：

1. **ethers-core** - 核心类型和工具
2. **ethers-contract-abigen** - ABI 生成器
3. **ethers-contract-derive** - 过程宏
4. **ethers-providers** - 网络提供者
5. **ethers-signers** - 签名器
6. **ethers-solc** - Solidity 编译器集成
7. **ethers-etherscan** - Etherscan API
8. **ethers-addressbook** - 地址簿
9. **ethers-middleware** - 中间件
10. **ethers-contract** - 智能合约交互
11. **ethers** - 主包（聚合所有功能）

## 故障排除

### 工作空间依赖版本问题

如果遇到 "failed to select a version" 错误：

```bash
# 使用工作空间脚本（推荐）
./scripts/publish-workspace.sh

# 或手动发布单个包
cargo publish -p ethers-core --allow-dirty
```

### 发布失败

如果某个包发布失败：
1. 检查错误信息
2. 修复问题
3. 从失败的包开始重新发布

```bash
# 单独发布某个包
cargo publish -p ethers-core --allow-dirty

# 或使用脚本的干运行检查
./scripts/publish.sh --dry-run
```

### 常见问题

**Q: "already exists" 错误**
A: 该版本已存在，需要更新版本号

**Q: "authentication required" 错误**
A: 需要重新登录 `cargo login`

**Q: "dependency not found" 错误**
A: 前置依赖包还未在 crates.io 上可用，等待几分钟后重试

**Q: "failed to select a version" 错误**
A: 工作空间依赖版本问题，使用 `--allow-dirty` 标志或工作空间脚本

**Q: 网络超时**
A: 脚本有重试机制，会自动重试最多3次

## 验证发布

发布完成后，可以通过以下方式验证：

```bash
# 搜索包
cargo search ethers

# 查看包信息
cargo info ethers

# 在新项目中测试
cargo new test-ethers
cd test-ethers
cargo add ethers
cargo check
```

## 发布后操作

1. **创建 Git 标签**
```bash
git tag v2.0.15
git push origin v2.0.15
```

2. **更新文档**
- 更新 CHANGELOG.md
- 更新 README.md 中的版本信息

3. **发布公告**
- GitHub Release
- 社交媒体
- 技术博客

## 安全注意事项

- ⚠️ 确保在发布前彻底测试代码
- ⚠️ 不要发布包含敏感信息的版本
- ⚠️ 确认版本号正确，发布后无法撤回
- ⚠️ 发布前检查所有依赖关系

## 作者

**dark** - web3自由职业者联盟

如有问题，请提交 Issue 或联系维护者。 