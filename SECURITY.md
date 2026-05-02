# 安全策略

## 支持版本

| 版本 | 是否接受安全更新 |
|---|---|
| 2.x | ✅ |
| 1.x | ❌ 不再维护 |

## 报告漏洞

如果你发现安全漏洞（特别是涉及到本地数据泄漏、API key 泄漏、远程命令执行等），请 **不要** 在公开 issue 中提交。

请通过 GitHub 的 [Private Vulnerability Reporting](https://docs.github.com/en/code-security/security-advisories/guidance-on-reporting-and-writing-information-about-vulnerabilities/privately-reporting-a-security-vulnerability) 私下提交，或在 issue 中要求私下沟通渠道。

我们会在 7 天内确认收到、30 天内给出修复时间表。

## 已知风险点

- **API key 当前明文存储**于本地 SQLite（`%APPDATA%\com.reading-tracker.app\reading_tracker.db`）。请确保设备只你自己使用。后续版本会迁移到 OS Keyring。
- **AI 调用走第三方**。当你发送对话消息时，内容会经 reqwest 直接发到你配置的 AI 服务商。请遵守对应服务条款，不要把敏感个人信息发给不信任的 endpoint。
