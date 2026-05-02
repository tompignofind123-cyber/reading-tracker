use crate::models::Book;

const RULES: &str = r#"你是一个面向阅读爱好者的对话助手。请严格遵守：

【信息可信度规则】
1. 区分"事实"与"虚构"：作品中的角色、剧情、设定、台词属于"虚构"；作者生平、出版日期、销量、获奖、改编、读者评论等属于"事实"。
2. 涉及"事实"时，必须使用网络搜索工具，并要求至少两个独立来源相互印证；若来源结论不一致，必须明确说明"以下信息存在分歧"并列出各方说法和来源。
3. 涉及"虚构"内容时，以原作内容/用户笔记为准；若用户笔记与网络解读冲突，以用户笔记为准（但可补充"另有 X 解读"）。
4. 找不到可靠来源时，明确说"未找到可靠信息"，禁止编造销量/评分/奖项/链接。
5. 来源 URL 必须出自搜索工具的真实返回，禁止虚构链接。
6. 回答末尾用「来源：」分行列出本轮实际访问的链接（仅在确实使用了搜索时）。
7. 涉及剧透时先简短预警一句再展开。

【风格】
- 用中文回答（除非用户用其他语言）
- 段落清晰，必要时使用 Markdown 列表/小标题
- 不要无意义的客套话
"#;

pub fn build_system_prompt(mode: &str, book: Option<&Book>, library_summary: Option<&str>) -> String {
    let mut s = String::with_capacity(2048);
    s.push_str(RULES);
    s.push_str("\n");

    match mode {
        "book" => {
            if let Some(b) = book {
                s.push_str("【当前书籍】\n");
                s.push_str(&format!("《{}》", b.title));
                if !b.author.is_empty() { s.push_str(&format!(" — 作者：{}", b.author)); }
                s.push_str(&format!("\n分类：{} / 评分：{}/5 / 阅读日期：{}\n", b.category, b.rating, b.date_read));
                if b.word_count > 0 { s.push_str(&format!("字数：{}\n", b.word_count)); }
                if !b.tags.is_empty() { s.push_str(&format!("标签：{}\n", b.tags.join("、"))); }
                if !b.reflection.is_empty() {
                    s.push_str("\n【用户笔记/读后感】\n");
                    s.push_str(&b.reflection);
                    s.push_str("\n");
                }
                s.push_str("\n本次对话默认围绕这本书。如果用户问的是与本书无关的内容，正常回答即可。\n");
            }
        }
        "cross" => {
            s.push_str("【模式】跨书对话——可结合用户书库整体来分析。\n");
            if let Some(sum) = library_summary {
                s.push_str("\n【用户书库摘要】\n");
                s.push_str(sum);
                s.push_str("\n");
            }
        }
        _ => {
            s.push_str("【模式】全局自由对话。\n");
        }
    }

    s
}

pub fn library_summary_text(books: &[Book], max: usize) -> String {
    let mut s = String::new();
    s.push_str(&format!("共 {} 本：\n", books.len()));
    for b in books.iter().take(max) {
        s.push_str(&format!("- 《{}》", b.title));
        if !b.author.is_empty() { s.push_str(&format!(" / {}", b.author)); }
        s.push_str(&format!(" / {} / 评分 {}/5\n", b.category, b.rating));
    }
    if books.len() > max {
        s.push_str(&format!("……（其余 {} 本未列出）\n", books.len() - max));
    }
    s
}
