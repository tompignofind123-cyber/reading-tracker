export type ChatProviderName = "openai" | "anthropic" | "gemini";

export interface ChatSettings {
  provider: ChatProviderName;
  base_url: string;
  api_key: string;
  model: string;
  enable_search: boolean;
  extra_json: string;
}

export type ChatMode = "book" | "cross" | "global";

export interface ChatSession {
  id?: number;
  book_id?: number | null;
  mode: ChatMode;
  title: string;
  created_at?: string;
  updated_at?: string;
}

export interface ChatSource {
  url: string;
  title?: string | null;
}

export interface ChatMessage {
  id?: number;
  session_id: number;
  role: "user" | "assistant" | "system";
  content: string;
  sources: ChatSource[];
  created_at?: string;
}

export type ChatEvent =
  | { type: "delta"; data: { text: string } }
  | { type: "source"; data: { url: string; title?: string | null } }
  | { type: "error"; data: { message: string } }
  | { type: "done"; data: { message_id: number } };

export const PROVIDER_PRESETS: Record<ChatProviderName, { base_url: string; default_model: string; label: string; tip: string }> = {
  openai: {
    base_url: "https://api.openai.com/v1",
    default_model: "gpt-4o-mini",
    label: "OpenAI 兼容",
    tip: "可改 base_url 接入 DeepSeek / Kimi / OpenRouter 等。开启搜索需要支持 web_search_preview 的模型。",
  },
  anthropic: {
    base_url: "https://api.anthropic.com",
    default_model: "claude-sonnet-4-6",
    label: "Anthropic Claude",
    tip: "原生支持 web_search 工具，搜索质量高。",
  },
  gemini: {
    base_url: "https://generativelanguage.googleapis.com/v1beta",
    default_model: "gemini-2.5-flash",
    label: "Google Gemini",
    tip: "原生支持 google_search grounding。",
  },
};
