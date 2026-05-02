import React from "react";
import { ChatSession } from "../chat-types";

interface Props {
  sessions: ChatSession[];
  activeId?: number;
  onSelect: (s: ChatSession) => void;
  onNew: () => void;
  onDelete: (id: number) => void;
}

const MODE_LABEL: Record<string, string> = {
  book: "本书",
  cross: "跨书",
  global: "全局",
};

export const SessionList: React.FC<Props> = ({ sessions, activeId, onSelect, onNew, onDelete }) => {
  return (
    <div className="session-list">
      <div className="session-list-header">
        <span>对话历史</span>
        <button className="btn-icon" title="新建对话" onClick={onNew}>＋</button>
      </div>
      {sessions.length === 0 ? (
        <div className="session-empty">还没有对话</div>
      ) : (
        <div className="session-items">
          {sessions.map((s) => (
            <div
              key={s.id}
              className={`session-item ${activeId === s.id ? "active" : ""}`}
              onClick={() => onSelect(s)}
            >
              <div className="session-row">
                <span className={`session-mode mode-${s.mode}`}>{MODE_LABEL[s.mode] || s.mode}</span>
                <span className="session-title" title={s.title}>{s.title}</span>
              </div>
              <div className="session-meta">
                <span>{s.updated_at?.slice(5, 16)}</span>
                <button
                  className="btn-icon-sm"
                  title="删除"
                  onClick={(e) => { e.stopPropagation(); s.id && onDelete(s.id); }}
                >🗑</button>
              </div>
            </div>
          ))}
        </div>
      )}
    </div>
  );
};
