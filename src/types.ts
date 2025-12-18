export interface Question {
  id: string;
  stem: string; // Question text (markdown supported)
  code?: string; // Code snippet if applicable
  answers: Answer[];
  explanation?: string; // Correct answer explanation
  distractors?: string; // Why wrong answers are tempting
  // Legacy field for backward compatibility
  content?: string; // Deprecated, use stem + code
}

export interface Answer {
  text: string;
  is_correct: boolean;
  explanation?: string; // Why this answer is correct/incorrect
}

export interface TopicInfo {
  id: string;
  name: string;
  description: string;
  example_count: number;
}

export interface SubjectInfo {
  id: string;
  name: string;
  topic_count: number;
}

export interface GenerationRequest {
  subject: string;
  topics: string[];
  difficulty: string;
  count: number;
  notes: string | null;
  append: boolean;
}
