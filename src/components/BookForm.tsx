import React, { useState, useEffect } from "react";
import { Book, CATEGORIES, emptyBook } from "../types";
import { StarRating } from "./StarRating";

interface Props {
  book?: Book;
  onSave: (book: Book) => void;
  onCancel: () => void;
}

export const BookForm: React.FC<Props> = ({ book, onSave, onCancel }) => {
  const [form, setForm] = useState<Book>(book || emptyBook());
  const [tagInput, setTagInput] = useState("");

  useEffect(() => {
    setForm(book || emptyBook());
    setTagInput("");
  }, [book]);

  const set = <K extends keyof Book>(key: K, val: Book[K]) =>
    setForm((f) => ({ ...f, [key]: val }));

  const addTag = () => {
    const tag = tagInput.trim();
    if (tag && !form.tags.includes(tag)) {
      set("tags", [...form.tags, tag]);
    }
    setTagInput("");
  };

  const removeTag = (tag: string) => {
    set("tags", form.tags.filter((t) => t !== tag));
  };

  const handleSubmit = (e: React.FormEvent) => {
    e.preventDefault();
    if (!form.title.trim()) return;
    onSave(form);
  };

  return (
    <form className="book-form" onSubmit={handleSubmit}>
      <h2>{book?.id ? "编辑作品" : "添加新作品"}</h2>

      <label className="form-field">
        <span className="required">标题</span>
        <input type="text" value={form.title} onChange={(e) => set("title", e.target.value)} required />
      </label>

      <label className="form-field">
        <span>作者</span>
        <input type="text" value={form.author} onChange={(e) => set("author", e.target.value)} />
      </label>

      <div className="form-row">
        <label className="form-field">
          <span className="required">分类</span>
          <select value={form.category} onChange={(e) => set("category", e.target.value)}>
            {Object.entries(CATEGORIES).map(([k, v]) => (
              <option key={k} value={k}>{v}</option>
            ))}
          </select>
        </label>

        <label className="form-field">
          <span>字数</span>
          <input type="number" min="0" value={form.word_count || ""} onChange={(e) => set("word_count", parseInt(e.target.value) || 0)} />
        </label>
      </div>

      <div className="form-row">
        <label className="form-field">
          <span className="required">阅读日期</span>
          <input type="date" value={form.date_read} onChange={(e) => set("date_read", e.target.value)} required />
        </label>

        <div className="form-field">
          <span>评分</span>
          <StarRating rating={form.rating} onChange={(r) => set("rating", r)} />
        </div>
      </div>

      <div className="form-field">
        <span>标签</span>
        <div className="tag-input-area">
          <div className="tags-display">
            {form.tags.map((tag) => (
              <span key={tag} className="tag" onClick={() => removeTag(tag)}>
                {tag} &times;
              </span>
            ))}
          </div>
          <div className="tag-input-row">
            <input
              type="text"
              placeholder="输入标签后按回车"
              value={tagInput}
              onChange={(e) => setTagInput(e.target.value)}
              onKeyDown={(e) => { if (e.key === "Enter") { e.preventDefault(); addTag(); } }}
            />
            <button type="button" className="btn-ghost btn-sm" onClick={addTag}>添加</button>
          </div>
        </div>
      </div>

      <label className="form-field">
        <span>读后感</span>
        <textarea
          rows={8}
          placeholder="写下你的读后感..."
          value={form.reflection}
          onChange={(e) => set("reflection", e.target.value)}
        />
      </label>

      <div className="form-actions">
        <button type="button" className="btn-ghost" onClick={onCancel}>取消</button>
        <button type="submit" className="btn-primary">保存</button>
      </div>
    </form>
  );
};
