# 贡献指南

感谢你愿意为 Reading Tracker 贡献力量！🎉

## 🐛 报告 Bug

1. 在 [Issues](../../issues) 检查是否已有相同问题
2. 使用 **Bug Report** 模板提交，附上：
   - 操作系统 + 版本
   - 应用版本
   - 复现步骤
   - 截图或日志（如可能）

## 💡 提需求

使用 **Feature Request** 模板，描述：
- 当前痛点
- 你期望的解决方案
- 替代方案（如有）

## 🔧 提交代码

### 流程
```bash
git clone https://github.com/<your-username>/reading-tracker.git
cd reading-tracker
npm install

git checkout -b feat/short-description
# ... 写代码 ...

# 提交前必查
npm run build                                       # tsc + vite build
cargo check --manifest-path src-tauri/Cargo.toml    # Rust 类型检查
cargo clippy --manifest-path src-tauri/Cargo.toml   # Rust lint（可选但建议）

git commit -m "feat: short description"
git push origin feat/short-description
```

然后开 Pull Request，并在描述里：
- 关联相关 issue（`Closes #123`）
- 简述改动动机
- 截图（如果是 UI 改动）

### 提交信息规范
遵循 [Conventional Commits](https://www.conventionalcommits.org/zh-hans/v1.0.0/)：

| 前缀 | 用于 |
|---|---|
| `feat:` | 新功能 |
| `fix:` | 修 bug |
| `docs:` | 文档 |
| `style:` | 格式（不影响代码逻辑） |
| `refactor:` | 重构（既不是新功能也不是修 bug） |
| `perf:` | 性能优化 |
| `test:` | 测试 |
| `chore:` | 构建、CI、依赖等杂项 |

### 代码风格
- **TypeScript**：函数式组件 + hooks，避免 class 组件
- **Rust**：遵循 `rustfmt` 默认风格；新模块尽量独立、低耦合
- **CSS**：使用 `App.css` 顶部已有的 CSS 变量（`var(--primary)` 等），不引入新色板
- **无注释代码**：除非逻辑非显而易见，否则不写注释；让代码本身说话

### AI 适配器
新增 AI 提供商：
1. 在 `src-tauri/src/chat/providers/` 新建 `<name>.rs`，实现 `ChatProvider` trait
2. 在 `providers/mod.rs::get_provider` 加分支
3. 在 `src/chat-types.ts::PROVIDER_PRESETS` 加预设
4. 在 README 的 [AI 配置](./README.md#-ai-配置) 章节加表格行

## 🌐 翻译 / i18n

目前应用 UI 是中文。欢迎 PR 把字符串提取到 i18n 字典并补充其他语言。

## 📜 协议

提交即视为同意你的贡献以 [MIT License](./LICENSE) 协议发布。
