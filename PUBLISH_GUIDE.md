# Ethers-rs 发布指南

本指南提供了发布 ethers-rs 包到 crates.io 的完整流程。

## 🚀 快速开始

### 1. 准备环境
```bash
# 登录 crates.io
cargo login

# 检查当前状态
cargo check --workspace
```

### 2. 更新版本号
编辑 `Cargo.toml`：
```toml
[workspace.package]
version = "2.0.15"  # 更新版本号
```

以及工作空间依赖：
```toml
[workspace.dependencies]
ethers-core = { version = "2.0.15", path = "ethers-core", default-features = false }
ethers-providers = { version = "2.0.15", path = "ethers-providers", default-features = false }
# ... 更新所有包的版本
```

### 3. 发布（推荐方式）
```bash
./scripts/publish-workspace.sh
```

## 📝 详细流程

### 发布前检查清单

- [ ] 所有代码已提交到 Git
- [ ] 构建正常：`cargo check --workspace`
- [ ] 测试通过：`cargo test --workspace`（可选）
- [ ] 版本号已更新
- [ ] 已登录 crates.io：`cargo login`

### 发布脚本选择

| 脚本 | 适用场景 | 特点 |
|------|----------|------|
| `publish-workspace.sh` | **推荐** | 解决工作空间依赖问题，使用 `--allow-dirty` |
| `publish.sh` | 完整检查 | 包含所有预检查，但可能遇到依赖版本问题 |
| `quick-publish.sh` | 快速发布 | 简化流程，适合熟悉用户 |

### 发布顺序

脚本会自动按照依赖关系发布：

```
ethers-core
├── ethers-contract-abigen
├── ethers-contract-derive
├── ethers-providers
├── ethers-signers
├── ethers-solc
├── ethers-etherscan
├── ethers-addressbook
├── ethers-middleware
├── ethers-contract
└── ethers (主包)
```

## 🛠️ 故障排除

### 常见错误及解决方案

#### 1. 依赖版本错误
```
failed to select a version for ethers-core
```

**解决方案：**
```bash
# 使用工作空间脚本
./scripts/publish-workspace.sh

# 或手动发布
cargo publish -p ethers-core --allow-dirty
```

#### 2. 包已存在
```
crate ethers@2.0.15 already exists
```

**解决方案：** 更新版本号到未使用的版本

#### 3. 认证失败
```
authentication required
```

**解决方案：** 重新登录
```bash
cargo login
```

#### 4. 网络超时
脚本有自动重试机制，会重试最多 3 次

### 手动发布流程

如果脚本失败，可以手动发布：

```bash
# 按顺序发布每个包
cargo publish -p ethers-core --allow-dirty
sleep 30  # 等待索引更新

cargo publish -p ethers-contract-abigen --allow-dirty
sleep 30

cargo publish -p ethers-contract-derive --allow-dirty
sleep 30

# ... 继续其他包
```

## 📊 发布后验证

### 1. 检查包是否可用
```bash
cargo search ethers
cargo info ethers
```

### 2. 测试安装
```bash
cargo new test-project
cd test-project
cargo add ethers
cargo check
```

### 3. 创建 Git 标签
```bash
git tag v2.0.15
git push origin v2.0.15
```

## 📚 版本管理策略

### 语义化版本控制

- **MAJOR.MINOR.PATCH** (例如: 2.0.15)
- **MAJOR**: 不兼容的 API 更改
- **MINOR**: 向后兼容的功能添加
- **PATCH**: 向后兼容的错误修复

### 发布频率

- **补丁版本**: 根据需要（错误修复）
- **次要版本**: 每月或每季度（新功能）
- **主要版本**: 每年或重大架构更改

## 🔒 安全注意事项

1. **代码审查**: 确保所有更改都经过审查
2. **测试覆盖**: 运行完整测试套件
3. **依赖检查**: 确认所有依赖都是最新且安全的
4. **版本验证**: 确认版本号正确
5. **发布权限**: 只有授权人员可以发布

## 📋 发布检查清单

### 发布前
- [ ] 代码审查完成
- [ ] 所有测试通过
- [ ] 文档更新
- [ ] 版本号更新
- [ ] CHANGELOG.md 更新
- [ ] Git 状态干净

### 发布中
- [ ] 使用正确的发布脚本
- [ ] 监控发布过程
- [ ] 处理任何错误

### 发布后
- [ ] 验证包可用性
- [ ] 创建 Git 标签
- [ ] 更新文档
- [ ] 发布公告
- [ ] 监控社区反馈

## 📞 获取帮助

如果遇到问题：

1. 查看脚本帮助：`./scripts/publish-workspace.sh --help`
2. 查看 Cargo 文档：https://doc.rust-lang.org/cargo/
3. 查看 crates.io 指南：https://doc.rust-lang.org/cargo/reference/publishing.html
4. 提交 Issue 到项目仓库

---

**作者**: dark - web3自由职业者联盟  
**最后更新**: 2024年 