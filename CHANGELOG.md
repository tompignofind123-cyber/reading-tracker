# Changelog

本项目所有显著变更都记录在此文件中。

格式参考 [Keep a Changelog](https://keepachangelog.com/zh-CN/1.1.0/)，
版本号遵循 [Semantic Versioning](https://semver.org/lang/zh-CN/)。

## [2.0.0] - 2026-05-02

### ✨ 新增
- **AI 深度对话功能**
  - 三种模式：单本书 / 跨书 / 全局
  - 多 AI 提供商热切换：OpenAI 兼容、Anthropic Claude、Google Gemini
  - 模型自带网络搜索（无需第三方 Search Key）
  - 流式响应（Tauri Channel）
  - 来源引用展示（折叠面板）
  - Markdown + GFM 渲染（react-markdown + remark-gfm）
- **每本书对话独立持久化** — 关闭应用后历史完整继承
- **防污染机制**：system prompt 强制区分事实/虚构、要求 ≥2 源交叉验证、禁止编造来源
- **自定义深色标题栏** — 关闭 Windows 默认装饰，整窗深色融合
- **AI 设置弹窗** — provider 切换、base_url、API key（密文显示）、模型、搜索开关

### 🐛 修复
- 书卡 hover 时编辑/删除按钮不再覆盖日期文本

### 🏗️ 架构
- 新增 SQLite 表：`chat_settings`、`chat_sessions`、`chat_messages`
- `book_id` 外键 `ON DELETE SET NULL`（删书保留对话）
- Cargo.toml 新增依赖：`reqwest`、`tokio`、`futures-util`、`async-trait`
- package.json 新增依赖：`react-markdown`、`remark-gfm`
- 新增 `src-tauri/capabilities/default.json` 显式声明窗口控制权限

## [1.0.0] - 2025-04-14

### 初始发布
- 书籍 CRUD（标题、作者、分类、标签、字数、评分、阅读日期、读后感）
- 多维度搜索（关键词、分类、评分区间、字数区间、日期区间）
- 数据导出（JSON / CSV with BOM）
- 阅读概览统计（总数、总字数、平均评分、分类分布）
- SQLite 本地存储 + WAL 模式
- Tauri 2 + React 18 + TypeScript 5

[2.0.0]: ../../releases/tag/v2.0.0
[1.0.0]: ../../releases/tag/v1.0.0
