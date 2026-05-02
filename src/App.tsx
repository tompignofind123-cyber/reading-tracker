import { useState, useCallback, useEffect, useMemo, useRef } from "react";
import { invoke } from "@tauri-apps/api/core";
import { Book, SearchParams, CATEGORIES } from "./types";
import { SearchBar } from "./components/SearchBar";
import { BookList } from "./components/BookList";
import { BookForm } from "./components/BookForm";
import { BookDetail } from "./components/BookDetail";
import { ExportButton } from "./components/ExportButton";
import { ChatPanel } from "./components/ChatPanel";
import { ChatSettings } from "./components/ChatSettings";
import { TitleBar } from "./components/TitleBar";
import "./App.css";

type View = "list" | "detail" | "form" | "chat";

function App() {
  const [books, setBooks] = useState<Book[]>([]);
  const [total, setTotal] = useState(0);
  const [searchParams, setSearchParams] = useState<SearchParams>({});
  const [view, setView] = useState<View>("list");
  const [selectedBook, setSelectedBook] = useState<Book | undefined>();
  const [editingBook, setEditingBook] = useState<Book | undefined>();
  const [deleteConfirm, setDeleteConfirm] = useState<number | null>(null);
  const [chatBook, setChatBook] = useState<Book | undefined>();
  const [showSettings, setShowSettings] = useState(false);
  const searchParamsRef = useRef(searchParams);
  searchParamsRef.current = searchParams;

  const loadBooks = useCallback(async (params?: SearchParams) => {
    const p = params || searchParamsRef.current;
    const result: [Book[], number] = await invoke("get_books", { params: p });
    setBooks(result[0]);
    setTotal(result[1]);
  }, []);

  useEffect(() => { loadBooks(); }, []);

  const stats = useMemo(() => {
    const avgRating = books.length > 0
      ? books.filter(b => b.rating > 0).reduce((s, b) => s + b.rating, 0) / Math.max(books.filter(b => b.rating > 0).length, 1)
      : 0;
    const totalWords = books.reduce((s, b) => s + b.word_count, 0);
    const categoryCounts: Record<string, number> = {};
    for (const b of books) {
      categoryCounts[b.category] = (categoryCounts[b.category] || 0) + 1;
    }
    return { avgRating, totalWords, categoryCounts };
  }, [books]);

  const handleSearch = (params: SearchParams) => {
    setSearchParams(params);
    loadBooks(params);
  };

  const handleSelect = (book: Book) => {
    if (view === "chat") {
      setChatBook(book);
    } else {
      setSelectedBook(book);
      setView("detail");
    }
  };

  const handleAdd = () => {
    setEditingBook(undefined);
    setView("form");
  };

  const handleEdit = (book: Book) => {
    setEditingBook(book);
    setView("form");
  };

  const handleSave = async (book: Book) => {
    if (book.id) {
      await invoke("update_book", { book });
    } else {
      await invoke("add_book", { book });
    }
    setView("list");
    setEditingBook(undefined);
    await loadBooks();
  };

  const handleDelete = async (id: number) => {
    setDeleteConfirm(id);
  };

  const confirmDelete = async () => {
    if (deleteConfirm === null) return;
    await invoke("delete_book", { id: deleteConfirm });
    if (selectedBook?.id === deleteConfirm) {
      setSelectedBook(undefined);
      setView("list");
    }
    setDeleteConfirm(null);
    await loadBooks();
  };

  const formatWords = (n: number) => {
    if (n >= 100000000) return (n / 100000000).toFixed(1) + "亿";
    if (n >= 10000) return (n / 10000).toFixed(1) + "万";
    return n.toLocaleString();
  };

  return (
    <div className="app">
      <TitleBar />
      <header className="app-header">
        <h1>阅读记录</h1>
        <div className="header-actions">
          <button
            className={`btn-tab ${view === "chat" ? "active" : ""}`}
            onClick={() => setView(view === "chat" ? "list" : "chat")}
          >💬 AI 对话</button>
          <button className="btn-tab" title="AI 设置" onClick={() => setShowSettings(true)}>⚙ AI 设置</button>
          <ExportButton />
          <button className="btn-primary" onClick={handleAdd}>+ 添加作品</button>
        </div>
      </header>

      <main className="app-main">
        <div className="sidebar">
          <SearchBar onSearch={handleSearch} />
          <BookList
            books={books}
            total={total}
            selectedId={view === "chat" ? chatBook?.id : selectedBook?.id}
            onSelect={handleSelect}
            onEdit={handleEdit}
            onDelete={handleDelete}
          />
        </div>

        <div className="content">
          {view === "form" && (
            <BookForm
              book={editingBook}
              onSave={handleSave}
              onCancel={() => { setView(selectedBook ? "detail" : "list"); }}
            />
          )}
          {view === "detail" && selectedBook && (
            <BookDetail
              book={selectedBook}
              onEdit={() => handleEdit(selectedBook)}
              onClose={() => { setSelectedBook(undefined); setView("list"); }}
              onChat={() => { setChatBook(selectedBook); setView("chat"); }}
            />
          )}
          {view === "chat" && (
            <ChatPanel
              contextBook={chatBook}
              onClearContextBook={() => setChatBook(undefined)}
            />
          )}
          {view === "list" && (
            <div className="welcome-panel">
              {books.length === 0 ? (
                <>
                  <div className="welcome-icon">📖</div>
                  <h2>开始记录你的阅读旅程</h2>
                  <p>点击"添加作品"来记录第一本</p>
                </>
              ) : (
                <div className="stats-panel">
                  <h2 className="stats-title">阅读概览</h2>
                  <div className="stats-grid">
                    <div className="stat-card">
                      <span className="stat-value">{total}</span>
                      <span className="stat-label">作品总数</span>
                    </div>
                    <div className="stat-card">
                      <span className="stat-value">{formatWords(stats.totalWords)}</span>
                      <span className="stat-label">总字数</span>
                    </div>
                    <div className="stat-card">
                      <span className="stat-value">{stats.avgRating > 0 ? stats.avgRating.toFixed(1) : "-"}</span>
                      <span className="stat-label">平均评分</span>
                    </div>
                  </div>
                  <div className="stats-categories">
                    <h3>分类分布</h3>
                    <div className="category-bars">
                      {Object.entries(stats.categoryCounts)
                        .sort((a, b) => b[1] - a[1])
                        .map(([cat, count]) => (
                          <div key={cat} className="category-bar-row">
                            <span className={`category-badge ${cat}`}>{CATEGORIES[cat] || cat}</span>
                            <div className="bar-track">
                              <div
                                className={`bar-fill ${cat}`}
                                style={{ width: `${(count / total) * 100}%` }}
                              />
                            </div>
                            <span className="bar-count">{count}</span>
                          </div>
                        ))}
                    </div>
                  </div>
                  <p className="stats-hint">选择左侧作品查看详情</p>
                </div>
              )}
            </div>
          )}
        </div>
      </main>

      {deleteConfirm !== null && (
        <div className="modal-overlay" onClick={() => setDeleteConfirm(null)}>
          <div className="modal" onClick={(e) => e.stopPropagation()}>
            <h3>确认删除</h3>
            <p>确定要删除这条记录吗？此操作不可恢复。</p>
            <div className="modal-actions">
              <button className="btn-ghost" onClick={() => setDeleteConfirm(null)}>取消</button>
              <button className="btn-danger" onClick={confirmDelete}>删除</button>
            </div>
          </div>
        </div>
      )}

      {showSettings && <ChatSettings onClose={() => setShowSettings(false)} />}
    </div>
  );
}

export default App;
