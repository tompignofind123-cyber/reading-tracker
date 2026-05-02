import React, { useEffect, useState } from "react";
import { getCurrentWindow } from "@tauri-apps/api/window";

export const TitleBar: React.FC = () => {
  const win = getCurrentWindow();
  const [maxed, setMaxed] = useState(false);

  useEffect(() => {
    let unlisten: (() => void) | undefined;
    (async () => {
      setMaxed(await win.isMaximized());
      unlisten = await win.onResized(async () => {
        setMaxed(await win.isMaximized());
      });
    })();
    return () => { unlisten?.(); };
  }, []);

  return (
    <div className="titlebar" data-tauri-drag-region>
      <div className="titlebar-text" data-tauri-drag-region>
        <span className="titlebar-icon">📖</span>
        <span>阅读记录管理器</span>
      </div>
      <div className="titlebar-controls">
        <button
          className="tb-btn"
          title="最小化"
          onClick={() => win.minimize()}
        >
          <svg width="10" height="10" viewBox="0 0 10 10"><rect x="1" y="4.5" width="8" height="1" fill="currentColor"/></svg>
        </button>
        <button
          className="tb-btn"
          title={maxed ? "还原" : "最大化"}
          onClick={() => win.toggleMaximize()}
        >
          {maxed ? (
            <svg width="10" height="10" viewBox="0 0 10 10">
              <rect x="2" y="2" width="6" height="6" fill="none" stroke="currentColor" strokeWidth="1"/>
              <rect x="3.5" y="0.5" width="6" height="6" fill="none" stroke="currentColor" strokeWidth="1"/>
            </svg>
          ) : (
            <svg width="10" height="10" viewBox="0 0 10 10"><rect x="1" y="1" width="8" height="8" fill="none" stroke="currentColor" strokeWidth="1"/></svg>
          )}
        </button>
        <button
          className="tb-btn tb-close"
          title="关闭"
          onClick={() => win.close()}
        >
          <svg width="10" height="10" viewBox="0 0 10 10">
            <line x1="1.5" y1="1.5" x2="8.5" y2="8.5" stroke="currentColor" strokeWidth="1.2"/>
            <line x1="8.5" y1="1.5" x2="1.5" y2="8.5" stroke="currentColor" strokeWidth="1.2"/>
          </svg>
        </button>
      </div>
    </div>
  );
};
