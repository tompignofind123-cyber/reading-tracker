import React from "react";
import { Book, CATEGORIES } from "../types";
import { StarRating } from "./StarRating";

interface Props {
  book: Book;
  onEdit: () => void;
  onClose: () => void;
  onChat: () => void;
}

export const BookDetail: React.FC<Props> = ({ book, onEdit, onClose, onChat }) => {
  return (
    <div className="book-detail">
      <div className="detail-header">
        <button className="btn-ghost" onClick={onClose}>← 返回列表</button>
        <div className="detail-header-actions">
          <button className="btn-secondary" onClick={onChat}>💬 和这本书聊聊</button>
          <button className="btn-primary" onClick={onEdit}>编辑</button>
        </div>
      </div>
      <div className="detail-content">
        <span className={`category-badge ${book.category}`}>
          {CATEGORIES[book.category] || book.category}
        </span>
        <h2>{book.title}</h2>
        {book.author && <p className="detail-author">作者：{book.author}</p>}
        <div className="detail-meta">
          <div className="meta-item">
            <span className="meta-label">评分</span>
            <StarRating rating={book.rating} readonly />
          </div>
          <div className="meta-item">
            <span className="meta-label">阅读日期</span>
            <span>{book.date_read}</span>
          </div>
          {book.word_count > 0 && (
            <div className="meta-item">
              <span className="meta-label">字数</span>
              <span>{book.word_count.toLocaleString()}</span>
            </div>
          )}
        </div>
        {book.tags.length > 0 && (
          <div className="detail-tags">
            {book.tags.map((tag) => (
              <span key={tag} className="tag">{tag}</span>
            ))}
          </div>
        )}
        {book.reflection && (
          <div className="detail-reflection">
            <h3>读后感</h3>
            <div className="reflection-text">{book.reflection}</div>
          </div>
        )}
      </div>
    </div>
  );
};
