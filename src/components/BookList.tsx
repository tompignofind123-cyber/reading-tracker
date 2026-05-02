import React from "react";
import { Book, CATEGORIES } from "../types";
import { StarRating } from "./StarRating";

interface Props {
  books: Book[];
  total: number;
  selectedId?: number;
  onSelect: (book: Book) => void;
  onEdit: (book: Book) => void;
  onDelete: (id: number) => void;
}

function formatWordCount(n: number): string {
  if (n >= 10000) return (n / 10000).toFixed(1) + "万";
  if (n >= 1000) return (n / 1000).toFixed(1) + "千";
  return n.toString();
}

export const BookList: React.FC<Props> = ({ books, total, selectedId, onSelect, onEdit, onDelete }) => {
  if (books.length === 0) {
    return (
      <div className="book-list-empty">
        <div className="empty-icon">📚</div>
        <p>还没有记录</p>
        <p className="hint">点击右上角"添加作品"开始记录吧</p>
      </div>
    );
  }

  return (
    <div className="book-list">
      <div className="list-header">
        <span className="total">共 {total} 条记录</span>
      </div>
      {books.map((book) => (
        <div
          key={book.id}
          className={`book-card ${selectedId === book.id ? "selected" : ""}`}
          onClick={() => onSelect(book)}
        >
          <div className="book-card-header">
            <span className={`category-badge ${book.category}`}>
              {CATEGORIES[book.category] || book.category}
            </span>
            <span className="date">{book.date_read}</span>
          </div>
          <h3 className="book-title">{book.title}</h3>
          {book.author && <p className="book-author">{book.author}</p>}
          <div className="book-card-footer">
            <StarRating rating={book.rating} readonly />
            {book.word_count > 0 && (
              <span className="word-count">{formatWordCount(book.word_count)}字</span>
            )}
          </div>
          {book.tags.length > 0 && (
            <div className="book-tags">
              {book.tags.map((tag) => (
                <span key={tag} className="tag-sm">{tag}</span>
              ))}
            </div>
          )}
          <div className="book-card-actions" onClick={(e) => e.stopPropagation()}>
            <button className="btn-icon" title="编辑" onClick={() => onEdit(book)}>✏️</button>
            <button className="btn-icon" title="删除" onClick={() => book.id && onDelete(book.id)}>🗑️</button>
          </div>
        </div>
      ))}
    </div>
  );
};
