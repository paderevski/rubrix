export interface Question {
  id: string;
  content: string;  // Full markdown content (question text + code blocks + tables)
  answers: Answer[];
}

export interface Answer {
  text: string;
  is_correct: boolean;
}

export interface TopicInfo {
  id: string;
  name: string;
  description: string;
  example_count: number;
}

export interface GenerationRequest {
  topics: string[];
  difficulty: string;
  count: number;
  notes: string | null;
}
