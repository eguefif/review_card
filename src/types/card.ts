export interface Option {
  id: string;
  answer: string;
}

export interface Question {
  id: string;
  question: string;
  options: Option[];
}

export type Quiz = Question[];

export interface Card {
  content: string;
  questions: Quiz;
}