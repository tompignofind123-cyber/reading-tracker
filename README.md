<div align="center">

# 📖 Reading Tracker · 阅读记录管理器

**一款带 AI 对话能力的本地化阅读记录工具 · Windows 桌面**

围绕你读过的每一本书与 AI 深度交流 · 上下文独立持久化 · 多源交叉验证防止信息污染

[![Tauri](https://img.shields.io/badge/Tauri-2.x-24C8DB?logo=tauri&logoColor=white)](https://tauri.app)
[![React](https://img.shields.io/badge/React-18-61DAFB?logo=react&logoColor=white)](https://react.dev)
[![Rust](https://img.shields.io/badge/Rust-edition%202021-CE422B?logo=rust&logoColor=white)](https://www.rust-lang.org)
[![TypeScript](https://img.shields.io/badge/TypeScript-5-3178C6?logo=typescript&logoColor=white)](https://www.typescriptlang.org)
[![SQLite](https://img.shields.io/badge/SQLite-bundled-003B57?logo=sqlite&logoColor=white)](https://www.sqlite.org)
[![License](https://img.shields.io/badge/License-MIT-yellow.svg)](./LICENSE)
[![Platform](https://img.shields.io/badge/Platform-Windows%2010%2F11%20x64-0078D4?logo=windows&logoColor=white)](#)

[特性](#-特性) · [截图](#-截图) · [安装](#-安装) · [AI 配置](#-ai-配置) · [使用指南](#-使用指南) · [技术栈](#-技术栈) · [开发](#-本地开发)

</div>

---

## ✨ 特性

### 📚 阅读记录核心
- **轻量本地存储** — 全部数据落地 SQLite，开机即用，零账号、零云依赖
- **多维度记录** — 标题 / 作者 / 分类 / 标签 / 字数 / 评分 / 阅读日期 / 读后感
- **强力筛选** — 关键词、分类、评分区间、字数区间、日期区间联合查询
- **导出友好** — 一键导出 JSON / CSV（含 BOM，Excel 直开）

### 🤖 AI 深度对话（v2.0 新增）
- **三种对话模式自由切换**
  - 📖 **本书模式** — 自动注入书的标题、作者、评分、你的笔记，让 AI "懂这本书"
  - 📚 **跨书模式** — 把整个书库摘要发给 AI，问"我读过的小说有哪些反复出现的主题"
  - 🌐 **全局模式** — 自由对话 + 联网搜索，问什么都行
- **每本书的对话独立持久化** — 关掉应用再打开，历史完整继承，能像看老朋友一样接着聊
- **多家提供商热切换** — OpenAI 兼容（含 DeepSeek / Kimi / OpenRouter / 自部署）/ Anthropic Claude / Google Gemini
- **模型自带网络搜索** — 直接调用 `web_search_preview` / `web_search_20250305` / `google_search`，无需第三方搜索 Key
- **来源链接折叠展示** — 每条回答附引用源 URL，点击直达原文

### 🛡️ 防污染机制（核心亮点）
精心设计的 system prompt 强制 AI 遵守：
- ✅ **区分事实与虚构** — 角色台词、剧情属于"虚构"；作者生平、销量、获奖属于"事实"
- ✅ **事实必须 ≥ 2 个独立来源交叉验证** — 不一致时显式标注「来源存在分歧」并列出各方说法
- ✅ **虚构内容以原作 / 用户笔记为准** — 网络解读冲突时优先你的私人笔记
- ✅ **找不到来源就说"未找到"** — 禁止编造销量、评分、奖项、URL
- ✅ **来源 URL 必须真实** — 禁止凭空虚构链接

### 🎨 视觉与交互
- **完全自定义深色标题栏** — 关闭 Windows 默认装饰，整窗融入暗色主题
- **流畅流式响应** — Tauri Channel 实时推送 token，所见即所得
- **Markdown + GFM 渲染** — 表格、代码块、引用、链接全支持

---

## 📸 截图

> 把下面的占位图片替换为你自己的截图（建议 1200×800 或更高清晰度）

| 阅读列表 | 书籍详情 |
|:---:|:---:|
| ![list](docs/screenshots/list.png) | ![detail](docs/screenshots/detail.png) |

| AI 对话 · 单本书模式 | AI 设置 |
|:---:|:---:|
| ![chat](docs/screenshots/chat.png) | ![settings](docs/screenshots/settings.png) |

---

## 📦 安装

### 方式一：下载 Release（推荐）
前往 [Releases](../../releases) 页面下载最新版安装包：

| 平台 | 文件 | 说明 |
|---|---|---|
| Windows 10/11 (x64) | `阅读记录管理器_x.y.z_x64-setup.exe` | NSIS 安装包，双击运行向导即可 |

> 当前仅适配 Windows。macOS / Linux 用户欢迎提 issue 或 PR。

### 方式二：从源码构建
见 [本地开发](#-本地开发) 章节。

---

## 🤖 AI 配置

打开应用 → 顶部 **「⚙ AI 设置」** 按钮 → 选择提供商 → 填好保存。

### OpenAI 兼容（最灵活）
适用于：OpenAI 官方、DeepSeek、Kimi、OpenRouter、Together、自部署 vLLM 等

| 服务 | base_url | 推荐模型 | 联网搜索 |
|---|---|---|---|
| OpenAI 官方 | `https://api.openai.com/v1` | `gpt-4o-mini` / `gpt-4o-search-preview` | ✅ 需选择 search 模型 |
| DeepSeek | `https://api.deepseek.com/v1` | `deepseek-chat` / `deepseek-reasoner` | ❌ 不支持，请关掉 |
| Moonshot Kimi | `https://api.moonshot.cn/v1` | `moonshot-v1-32k` | ❌ 关掉 |
| OpenRouter | `https://openrouter.ai/api/v1` | `openai/gpt-4o-mini` 等 | ⚠️ 看具体路由 |

### Anthropic Claude（搜索能力最强）
| 字段 | 值 |
|---|---|
| base_url | `https://api.anthropic.com` |
| model | `claude-sonnet-4-6` 或 `claude-haiku-4-6` |
| 联网搜索 | ✅ 原生 `web_search_20250305` 工具 |

### Google Gemini（性价比之选）
| 字段 | 值 |
|---|---|
| base_url | `https://generativelanguage.googleapis.com/v1beta` |
| model | `gemini-2.5-flash` 或 `gemini-2.5-pro` |
| 联网搜索 | ✅ 原生 `google_search` grounding |

> 💡 **小提示**：API key 当前以明文形式保存在本地 SQLite，请确保设备只你自己用。后续版本会加 OS Keyring 加密。

---

## 📖 使用指南

### 添加一本书
顶栏 **「+ 添加作品」** → 填写标题、作者、分类、标签、字数、评分、阅读日期、读后感 → 保存

### 单本书 AI 对话
1. 点击列表中的某本书 → 进入详情页
2. 点击 **「💬 和这本书聊聊」**
3. AI 会自动加载你的笔记作为上下文，可以问：
   - "主角的核心动机是什么？我笔记里写得对吗？"
   - "这本书与作者其他作品的风格差异在哪？"
   - "学术界对这本书的主流解读有几种？"

### 跨书模式
顶栏 **「💬 AI 对话」** → 切到 **「📚 跨书」** 模式：
- "我读过的轻小说里反复出现的母题是什么？"
- "按我的评分判断，我可能喜欢哪类未读的书？"

### 全局模式
切到 **「🌐 全局」** 模式 + 启用搜索：
- "《三体》英文版在 2024 年有什么新的精装本？"
- "村上春树最近一次访谈说了什么？"

### 上下文继承
**同一本书的所有对话会话都保存**：
- 关闭应用 → 重新打开 → 进入同一本书的「和这本书聊聊」
- 左侧会话列表展示所有历史对话，点击即可恢复完整聊天记录
- 想新开话题？点会话列表右上角的 **＋**

---

## 🏗️ 技术栈

| 层级 | 技术 |
|---|---|
| 桌面壳 | [Tauri 2](https://tauri.app) — Rust + 系统 webview，体积小、内存低 |
| 前端框架 | [React 18](https://react.dev) + [TypeScript 5](https://www.typescriptlang.org) |
| 构建工具 | [Vite 6](https://vitejs.dev) |
| 后端语言 | [Rust 2021](https://www.rust-lang.org) |
| 数据库 | [SQLite](https://www.sqlite.org)（通过 [rusqlite](https://github.com/rusqlite/rusqlite) bundled，无需额外安装） |
| HTTP 客户端 | [reqwest](https://github.com/seanmonstar/reqwest) + [rustls](https://github.com/rustls/rustls) |
| 异步运行时 | [tokio](https://tokio.rs) |
| Markdown 渲染 | [react-markdown](https://github.com/remarkjs/react-markdown) + [remark-gfm](https://github.com/remarkjs/remark-gfm) |

### 项目结构
```
reading-tracker/
├── src/                          # React 前端
│   ├── components/
│   │   ├── BookList.tsx          # 书籍列表
│   │   ├── BookDetail.tsx        # 书籍详情
│   │   ├── BookForm.tsx          # 新增/编辑表单
│   │   ├── SearchBar.tsx         # 搜索过滤器
│   │   ├── StarRating.tsx        # 星级评分
│   │   ├── ExportButton.tsx      # 导出 JSON/CSV
│   │   ├── TitleBar.tsx          # 自定义深色标题栏
│   │   ├── ChatPanel.tsx         # AI 对话主面板
│   │   ├── ChatSettings.tsx      # AI 配置弹窗
│   │   ├── SessionList.tsx       # 会话历史列表
│   │   └── MessageBubble.tsx     # 消息气泡 + 来源
│   ├── App.tsx
│   ├── App.css
│   ├── types.ts                  # 书籍相关类型
│   └── chat-types.ts             # AI 对话相关类型
│
├── src-tauri/                    # Rust 后端
│   ├── src/
│   │   ├── lib.rs                # Tauri 入口 / 命令注册
│   │   ├── db.rs                 # SQLite 连接 + 表初始化
│   │   ├── models.rs             # Book / SearchParams
│   │   ├── commands.rs           # 书籍 CRUD 命令
│   │   └── chat/                 # AI 对话模块
│   │       ├── mod.rs
│   │       ├── models.rs         # ChatSettings / Session / Message / Event
│   │       ├── db.rs             # 对话相关 CRUD
│   │       ├── prompt.rs         # 防污染 system prompt 模板
│   │       ├── commands.rs       # 流式 send_chat_message 等
│   │       └── providers/        # 三家 AI 适配器
│   │           ├── mod.rs        # ChatProvider trait
│   │           ├── openai.rs     # OpenAI 兼容 SSE
│   │           ├── anthropic.rs  # Claude messages SSE
│   │           └── gemini.rs     # Gemini streamGenerateContent
│   ├── capabilities/             # Tauri 2 权限声明
│   ├── icons/                    # 应用图标
│   ├── Cargo.toml
│   └── tauri.conf.json
│
├── package.json
├── tsconfig.json
└── vite.config.ts
```

### 数据库 Schema
```sql
-- 书籍主表
books(id, title, author, category, tags, word_count, rating, date_read, reflection, created_at, updated_at)

-- AI 全局设置（单行）
chat_settings(id=1, provider, base_url, api_key, model, enable_search, extra_json)

-- 对话会话
chat_sessions(id, book_id → books.id, mode, title, created_at, updated_at)

-- 对话消息
chat_messages(id, session_id → chat_sessions.id, role, content, sources_json, created_at)
```

> 删除一本书时，对应 chat_sessions 的 `book_id` 自动 SET NULL（保留对话历史不丢失）

---

## 🛠 本地开发

### 环境要求
- **Windows 10 / 11 x64**（当前仅支持 Windows）
- [Node.js](https://nodejs.org) ≥ 18
- [Rust](https://rustup.rs) ≥ 1.77（stable）
- WebView2（Win11 自带；Win10 由 Tauri 安装时自动拉起）
- Visual Studio Build Tools（含 C++ 工作负载）—— 安装 Rust 时通常会引导

### 启动开发服务器
```powershell
git clone https://github.com/<your-username>/reading-tracker.git
cd reading-tracker

npm install
npm run tauri dev          # 首次启动会编译 Rust，约 1–3 分钟
```

### 构建发行版
```powershell
npm run tauri build
# 产物：
#   src-tauri\target\release\reading-tracker.exe                          (单可执行)
#   src-tauri\target\release\bundle\nsis\阅读记录管理器_*_x64-setup.exe   (NSIS 安装包)
```

### 类型检查
```powershell
npm run build              # 包含 tsc --noEmit + vite build
cargo check --manifest-path src-tauri\Cargo.toml
```

---

## 🔐 数据与隐私

- ✅ **完全本地化** — 所有书籍记录、对话历史、API key 仅存于本机 SQLite
- ✅ **不上传任何遥测** — 应用本身不连接任何"主控"服务器
- ⚠️ **AI 调用走第三方** — 当你发送消息时，对话内容会经 reqwest 直接发送给你配置的 AI 服务商（OpenAI/Anthropic/Google 等），请遵守对应服务条款
- ⚠️ **API key 当前明文存储** — 在 `%APPDATA%\com.reading-tracker.app\reading_tracker.db`，后续版本会迁移到 Windows Credential Manager 加密

---

## 🗺️ 路线图

- [ ] API key 用 OS Keyring（Windows Credential Manager）加密存储
- [ ] 对话超 token 时滚动总结压缩历史
- [ ] 支持自定义 system prompt 模板
- [ ] 引入 Tavily / Brave 作为可选搜索源（不依赖模型自带）
- [ ] 对话导出为 Markdown / JSON
- [ ] 多语言 UI（i18n）
- [ ] Logseq / Obsidian 双向同步
- [ ] macOS / Linux 适配（社区有需要再做）

---

## 🤝 贡献

欢迎 issue、PR、想法！

```bash
# Fork + clone 后
git checkout -b feat/your-feature
# 提交前请确保
npm run build
cargo check --manifest-path src-tauri/Cargo.toml
git push origin feat/your-feature
# 然后开 PR
```

提交信息推荐 [Conventional Commits](https://www.conventionalcommits.org)：`feat:` `fix:` `docs:` `refactor:` `chore:`

---

## 📜 协议

[MIT License](./LICENSE) © 2026 Reading Tracker Contributors

---

## 🙏 致谢

- [Tauri](https://tauri.app) — 让 Rust + 系统 webview 写桌面应用如此优雅
- [Anthropic](https://anthropic.com) / [OpenAI](https://openai.com) / [Google](https://ai.google.dev) — 强大的 AI 与原生搜索能力
- 所有为开源生态贡献力量的人 ❤️

<div align="center">

如果这个项目对你有帮助，欢迎点一颗 ⭐ Star，是对作者最大的鼓励～

</div>
