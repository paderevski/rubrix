export interface Question {
  id: string;
  text: string; // Question text in markdown format (may include code blocks)
  answers: Answer[];
  explanation?: string; // Correct answer explanation
  distractors?: string; // Why wrong answers are tempting
  subject?: string;
  topics?: string[];
}

// Question bank (rich) entries
export interface QuestionBankEntry {
  id: string;
  text: string;
  options: QuestionBankOption[];
  explanation: string;
  difficulty: string;
  cognitive_level: string;
  topics: string[];
  skills: string[];
  distractors: DistractorInfo;
}

export interface QuestionBankOption {
  id: string;
  text: string;
  is_correct: boolean;
}

export interface DistractorInfo {
  common_mistakes: CommonMistake[];
  common_errors: string[];
}

export interface CommonMistake {
  option_id: string;
  misconception: string;
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
