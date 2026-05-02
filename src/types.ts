export interface Book {
  id?: number;
  title: string;
  author: string;
  category: string;
  tags: string[];
  word_count: number;
  rating: number;
  date_read: string;
  reflection: string;
  created_at?: string;
  updated_at?: string;
}

export interface SearchParams {
  keyword?: string;
  category?: string;
  min_rating?: number;
  max_rating?: number;
  min_word_count?: number;
  max_word_count?: number;
  date_from?: string;
  date_to?: string;
  page?: number;
  page_size?: number;
}

export const CATEGORIES: Record<string, string> = {
  manga: "漫画",
  novel: "小说",
  light_novel: "轻小说",
  web_novel: "网络小说",
  other: "其他",
};

export function emptyBook(): Book {
  return {
    title: "",
    author: "",
    category: "novel",
    tags: [],
    word_count: 0,
    rating: 0,
    date_read: new Date().toISOString().split("T")[0],
    reflection: "",
  };
}
