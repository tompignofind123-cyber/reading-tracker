import React, { useState } from "react";
import { SearchParams, CATEGORIES } from "../types";

interface Props {
  onSearch: (params: SearchParams) => void;
}

export const SearchBar: React.FC<Props> = ({ onSearch }) => {
  const [keyword, setKeyword] = useState("");
  const [category, setCategory] = useState("");
  const [minRating, setMinRating] = useState("");
  const [minWordCount, setMinWordCount] = useState("");
  const [maxWordCount, setMaxWordCount] = useState("");
  const [dateFrom, setDateFrom] = useState("");
  const [dateTo, setDateTo] = useState("");
  const [expanded, setExpanded] = useState(false);

  const doSearch = () => {
    onSearch({
      keyword: keyword || undefined,
      category: category || undefined,
      min_rating: minRating ? parseInt(minRating) : undefined,
      min_word_count: minWordCount ? parseInt(minWordCount) : undefined,
      max_word_count: maxWordCount ? parseInt(maxWordCount) : undefined,
      date_from: dateFrom || undefined,
      date_to: dateTo || undefined,
    });
  };

  const reset = () => {
    setKeyword("");
    setCategory("");
    setMinRating("");
    setMinWordCount("");
    setMaxWordCount("");
    setDateFrom("");
    setDateTo("");
    onSearch({});
  };

  return (
    <div className="search-bar">
      <div className="search-main">
        <input
          type="text"
          placeholder="搜索标题或作者..."
          value={keyword}
          onChange={(e) => setKeyword(e.target.value)}
          onKeyDown={(e) => e.key === "Enter" && doSearch()}
        />
        <button className="btn-primary" onClick={doSearch}>搜索</button>
        <button className="btn-ghost" onClick={() => setExpanded(!expanded)}>
          {expanded ? "收起筛选" : "高级筛选"}
        </button>
      </div>
      {expanded && (
        <div className="search-filters">
          <div className="filter-row">
            <label>
              分类
              <select value={category} onChange={(e) => setCategory(e.target.value)}>
                <option value="">全部</option>
                {Object.entries(CATEGORIES).map(([k, v]) => (
                  <option key={k} value={k}>{v}</option>
                ))}
              </select>
            </label>
            <label>
              最低评分
              <select value={minRating} onChange={(e) => setMinRating(e.target.value)}>
                <option value="">不限</option>
                {[1, 2, 3, 4, 5].map((r) => (
                  <option key={r} value={r}>{r} 星</option>
                ))}
              </select>
            </label>
          </div>
          <div className="filter-row">
            <label>
              字数范围
              <div className="range-input">
                <input type="number" placeholder="最少" value={minWordCount} onChange={(e) => setMinWordCount(e.target.value)} />
                <span>-</span>
                <input type="number" placeholder="最多" value={maxWordCount} onChange={(e) => setMaxWordCount(e.target.value)} />
              </div>
            </label>
          </div>
          <div className="filter-row">
            <label>
              日期范围
              <div className="range-input">
                <input type="date" value={dateFrom} onChange={(e) => setDateFrom(e.target.value)} />
                <span>至</span>
                <input type="date" value={dateTo} onChange={(e) => setDateTo(e.target.value)} />
              </div>
            </label>
          </div>
          <div className="filter-actions">
            <button className="btn-ghost" onClick={reset}>重置</button>
            <button className="btn-primary" onClick={doSearch}>应用筛选</button>
          </div>
        </div>
      )}
    </div>
  );
};
