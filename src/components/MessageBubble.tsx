import React from "react";
import ReactMarkdown from "react-markdown";
import remarkGfm from "remark-gfm";
import { ChatMessage } from "../chat-types";

interface Props {
  message: ChatMessage;
  streaming?: boolean;
}

export const MessageBubble: React.FC<Props> = ({ message, streaming }) => {
  const isUser = message.role === "user";
  const isAssistant = message.role === "assistant";
  const sources = message.sources || [];

  return (
    <div className={`msg-row ${isUser ? "msg-user" : "msg-assistant"}`}>
      <div className="msg-bubble">
        {isAssistant ? (
          <div className="msg-content markdown-body">
            <ReactMarkdown remarkPlugins={[remarkGfm]}>
              {message.content || (streaming ? "▍" : "")}
            </ReactMarkdown>
            {streaming && <span className="streaming-cursor">▍</span>}
          </div>
        ) : (
          <div className="msg-content">
            {message.content.split("\n").map((line, i) => (
              <div key={i}>{line || "\u00A0"}</div>
            ))}
          </div>
        )}
        {sources.length > 0 && (
          <details className="msg-sources">
            <summary>来源 · {sources.length}</summary>
            <ul>
              {sources.map((s, i) => (
                <li key={i}>
                  <a href={s.url} target="_blank" rel="noreferrer">
                    {s.title || s.url}
                  </a>
                </li>
              ))}
            </ul>
          </details>
        )}
      </div>
    </div>
  );
};
