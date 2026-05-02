# 截图占位目录

把这些文件替换为应用实际截图（建议 1200×800 或更高，PNG 格式，文件大小 < 500 KB）：

- `list.png` — 阅读列表页（主界面）
- `detail.png` — 书籍详情页
- `chat.png` — AI 对话面板（建议带消息流和来源链接展示）
- `settings.png` — AI 设置弹窗

截图后可以用 [TinyPNG](https://tinypng.com/) 压缩，或用 ImageMagick：

```bash
magick input.png -resize 1200x -quality 90 output.png
```
