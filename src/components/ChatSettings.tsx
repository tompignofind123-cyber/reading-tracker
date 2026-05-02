import React, { useEffect, useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import { ChatSettings as Settings, ChatProviderName, PROVIDER_PRESETS } from "../chat-types";

interface Props {
  onClose: () => void;
}

export const ChatSettings: React.FC<Props> = ({ onClose }) => {
  const [s, setS] = useState<Settings | null>(null);
  const [showKey, setShowKey] = useState(false);
  const [saving, setSaving] = useState(false);
  const [err, setErr] = useState<string>("");

  useEffect(() => {
    (async () => {
      try {
        const cur = await invoke<Settings>("get_chat_settings");
        setS(cur);
      } catch (e) {
        setErr(String(e));
      }
    })();
  }, []);

  if (!s) {
    return (
      <div className="modal-overlay" onClick={onClose}>
        <div className="modal" onClick={(e) => e.stopPropagation()}>
          <h3>AI 设置</h3>
          {err ? <p className="error-text">{err}</p> : <p>加载中…</p>}
        </div>
      </div>
    );
  }

  const onProviderChange = (p: ChatProviderName) => {
    const preset = PROVIDER_PRESETS[p];
    setS({
      ...s,
      provider: p,
      base_url: preset.base_url,
      model: s.model && s.provider === p ? s.model : preset.default_model,
    });
  };

  const onSave = async () => {
    setSaving(true);
    setErr("");
    try {
      await invoke("save_chat_settings", { settings: s });
      onClose();
    } catch (e) {
      setErr(String(e));
    } finally {
      setSaving(false);
    }
  };

  const preset = PROVIDER_PRESETS[s.provider];

  return (
    <div className="modal-overlay" onClick={onClose}>
      <div className="modal modal-wide" onClick={(e) => e.stopPropagation()}>
        <h3>AI 设置</h3>
        <div className="form-grid">
          <label>提供商</label>
          <div className="provider-tabs">
            {(Object.keys(PROVIDER_PRESETS) as ChatProviderName[]).map((p) => (
              <button
                key={p}
                className={`provider-tab ${s.provider === p ? "active" : ""}`}
                onClick={() => onProviderChange(p)}
                type="button"
              >
                {PROVIDER_PRESETS[p].label}
              </button>
            ))}
          </div>

          <label>Base URL</label>
          <input
            type="text"
            value={s.base_url}
            onChange={(e) => setS({ ...s, base_url: e.target.value })}
            placeholder={preset.base_url}
          />

          <label>模型</label>
          <input
            type="text"
            value={s.model}
            onChange={(e) => setS({ ...s, model: e.target.value })}
            placeholder={preset.default_model}
          />

          <label>API Key</label>
          <div className="key-row">
            <input
              type={showKey ? "text" : "password"}
              value={s.api_key}
              onChange={(e) => setS({ ...s, api_key: e.target.value })}
              placeholder="sk-..."
            />
            <button type="button" className="btn-ghost" onClick={() => setShowKey((v) => !v)}>
              {showKey ? "隐藏" : "显示"}
            </button>
          </div>

          <label>启用网络搜索</label>
          <label className="switch-row">
            <input
              type="checkbox"
              checked={s.enable_search}
              onChange={(e) => setS({ ...s, enable_search: e.target.checked })}
            />
            <span>使用模型内置 web search 工具</span>
          </label>
        </div>

        <p className="setting-tip">{preset.tip}</p>

        {err && <p className="error-text">{err}</p>}

        <div className="modal-actions">
          <button className="btn-ghost" onClick={onClose} disabled={saving}>取消</button>
          <button className="btn-primary" onClick={onSave} disabled={saving}>
            {saving ? "保存中…" : "保存"}
          </button>
        </div>
      </div>
    </div>
  );
};
