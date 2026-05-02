import React, { useEffect, useRef, useState } from "react";
import { invoke, Channel } from "@tauri-apps/api/core";
import { Book } from "../types";
import {
  ChatEvent, ChatMessage, ChatMode, ChatSession, ChatSource,
} from "../chat-types";
import { SessionList } from "./SessionList";
import { MessageBubble } from "./MessageBubble";

interface Props {
  contextBook?: Book;
  onClearContextBook: () => void;
}

const MODE_OPTIONS: { value: ChatMode; label: string; desc: string }[] = [
  { value: "book", label: "📖 单本书", desc: "围绕一本书聊（自动注入书籍信息和你的笔记）" },
  { value: "cross", label: "📚 跨书", desc: "结合你的书库整体分析" },
  { value: "global", label: "🌐 全局", desc: "自由对话，可联网搜索" },
];

export const ChatPanel: React.FC<Props> = ({ contextBook, onClearContextBook }) => {
  const [mode, setMode] = useState<ChatMode>(contextBook ? "book" : "global");
  const [sessions, setSessions] = useState<ChatSession[]>([]);
  const [activeSession, setActiveSession] = useState<ChatSession | null>(null);
  const [messages, setMessages] = useState<ChatMessage[]>([]);
  const [streamingText, setStreamingText] = useState<string>("");
  const [streamingSources, setStreamingSources] = useState<ChatSource[]>([]);
  const [isStreaming, setIsStreaming] = useState(false);
  const [input, setInput] = useState("");
  const [error, setError] = useState<string>("");
  const scrollRef = useRef<HTMLDivElement>(null);

  // 切换模式时调整 contextBook 联动
  useEffect(() => {
    if (mode !== "book" && contextBook) {
      onClearContextBook();
    }
  }, [mode]);

  useEffect(() => {
    if (contextBook && mode !== "book") {
      setMode("book");
    }
  }, [contextBook]);

  const loadSessions = async () => {
    const filter = mode === "book" && contextBook?.id
      ? { bookId: contextBook.id, mode: "book" }
      : { bookId: null, mode };
    const list = await invoke<ChatSession[]>("list_chat_sessions", filter);
    setSessions(list);
    return list;
  };

  // 模式或当前书变化时刷新会话列表
  useEffect(() => {
    (async () => {
      try {
        const list = await loadSessions();
        // 如果当前激活会话不在新列表里，清空
        if (activeSession && !list.some((s) => s.id === activeSession.id)) {
          setActiveSession(null);
          setMessages([]);
        }
      } catch (e) {
        setError(String(e));
      }
    })();
  }, [mode, contextBook?.id]);

  const loadMessages = async (sessionId: number) => {
    const msgs = await invoke<ChatMessage[]>("get_session_messages", { sessionId });
    setMessages(msgs);
  };

  const onSelectSession = async (s: ChatSession) => {
    setActiveSession(s);
    setError("");
    await loadMessages(s.id!);
  };

  const onNewSession = async () => {
    const newId = await invoke<number>("create_chat_session", {
      bookId: mode === "book" ? contextBook?.id ?? null : null,
      mode,
      title: null,
    });
    const list = await loadSessions();
    const created = list.find((s) => s.id === newId);
    if (created) {
      setActiveSession(created);
      setMessages([]);
    }
  };

  const onDeleteSession = async (id: number) => {
    if (!confirm("删除这个对话吗？")) return;
    await invoke("delete_chat_session", { id });
    if (activeSession?.id === id) {
      setActiveSession(null);
      setMessages([]);
    }
    await loadSessions();
  };

  const ensureSession = async (): Promise<ChatSession> => {
    if (activeSession) return activeSession;
    const newId = await invoke<number>("create_chat_session", {
      bookId: mode === "book" ? contextBook?.id ?? null : null,
      mode,
      title: null,
    });
    const list = await loadSessions();
    const created = list.find((s) => s.id === newId)!;
    setActiveSession(created);
    return created;
  };

  const onSend = async () => {
    const content = input.trim();
    if (!content || isStreaming) return;
    setInput("");
    setError("");
    const session = await ensureSession();

    // 乐观追加 user 消息
    const optimistic: ChatMessage = {
      session_id: session.id!,
      role: "user",
      content,
      sources: [],
    };
    setMessages((m) => [...m, optimistic]);
    setStreamingText("");
    setStreamingSources([]);
    setIsStreaming(true);

    const channel = new Channel<ChatEvent>();
    let buffer = "";
    const localSources: ChatSource[] = [];
    channel.onmessage = (e) => {
      if (e.type === "delta") {
        buffer += e.data.text;
        setStreamingText(buffer);
      } else if (e.type === "source") {
        if (!localSources.some((s) => s.url === e.data.url)) {
          localSources.push({ url: e.data.url, title: e.data.title });
          setStreamingSources([...localSources]);
        }
      } else if (e.type === "error") {
        setError(e.data.message);
      } else if (e.type === "done") {
        // 在 finally 中重新加载
      }
    };

    try {
      await invoke("send_chat_message", {
        sessionId: session.id,
        content,
        onEvent: channel,
      });
      await loadMessages(session.id!);
      await loadSessions();
    } catch (e) {
      setError(String(e));
      // 失败保留乐观 user 消息（DB 已写入），重载消息列表
      try { await loadMessages(session.id!); } catch {}
    } finally {
      setIsStreaming(false);
      setStreamingText("");
      setStreamingSources([]);
    }
  };

  // 自动滚动到底部
  useEffect(() => {
    if (scrollRef.current) {
      scrollRef.current.scrollTop = scrollRef.current.scrollHeight;
    }
  }, [messages, streamingText]);

  const onKeyDown = (e: React.KeyboardEvent<HTMLTextAreaElement>) => {
    if (e.key === "Enter" && !e.shiftKey && !e.nativeEvent.isComposing) {
      e.preventDefault();
      onSend();
    }
  };

  return (
    <div className="chat-panel">
      <div className="chat-sidebar">
        <div className="chat-mode-switcher">
          {MODE_OPTIONS.map((opt) => (
            <button
              key={opt.value}
              className={`mode-btn ${mode === opt.value ? "active" : ""}`}
              onClick={() => setMode(opt.value)}
              title={opt.desc}
            >
              {opt.label}
            </button>
          ))}
        </div>
        {mode === "book" && (
          <div className="context-book-card">
            {contextBook ? (
              <>
                <div className="ctx-label">当前讨论</div>
                <div className="ctx-title">《{contextBook.title}》</div>
                {contextBook.author && <div className="ctx-author">{contextBook.author}</div>}
                <button className="btn-ghost btn-sm" onClick={onClearContextBook}>切换书籍</button>
              </>
            ) : (
              <div className="ctx-empty">从左侧书籍点击「和这本书聊聊」选择一本</div>
            )}
          </div>
        )}
        <SessionList
          sessions={sessions}
          activeId={activeSession?.id}
          onSelect={onSelectSession}
          onNew={onNewSession}
          onDelete={onDeleteSession}
        />
      </div>

      <div className="chat-main">
        <div className="chat-messages" ref={scrollRef}>
          {messages.length === 0 && !isStreaming && (
            <div className="chat-empty">
              <div className="welcome-icon">💬</div>
              <h3>开始对话</h3>
              <p className="hint">
                {mode === "book" && (contextBook ? `就《${contextBook.title}》聊点什么` : "请先选择一本书")}
                {mode === "cross" && "可以问：「我读过的小说里反复出现的主题是什么？」"}
                {mode === "global" && "自由对话，开启搜索时可以问最新事实问题"}
              </p>
            </div>
          )}
          {messages.map((m, i) => (
            <MessageBubble key={m.id ?? `tmp-${i}`} message={m} />
          ))}
          {isStreaming && (
            <MessageBubble
              message={{
                session_id: activeSession?.id ?? 0,
                role: "assistant",
                content: streamingText,
                sources: streamingSources,
              }}
              streaming
            />
          )}
        </div>

        {error && <div className="chat-error">{error}</div>}

        <div className="chat-input-row">
          <textarea
            value={input}
            onChange={(e) => setInput(e.target.value)}
            onKeyDown={onKeyDown}
            placeholder={
              mode === "book" && !contextBook
                ? "请先选择一本书…"
                : "输入消息（Enter 发送，Shift+Enter 换行）"
            }
            disabled={isStreaming || (mode === "book" && !contextBook)}
            rows={2}
          />
          <button
            className="btn-primary"
            onClick={onSend}
            disabled={isStreaming || !input.trim() || (mode === "book" && !contextBook)}
          >
            {isStreaming ? "生成中…" : "发送"}
          </button>
        </div>
      </div>
    </div>
  );
};
