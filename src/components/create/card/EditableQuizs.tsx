import styled from '@emotion/styled';
import InputQuiz from './InputQuiz';
import { Question, Option } from '../../../types/card';

interface EditableQuizsProps {
  questions: Question[];
  setQuestions: (questions: Question[]) => void;
}

export default function EditableQuizs({ questions, setQuestions }: EditableQuizsProps) {
  function updateQuiz(id: string, newQuestion: string, options: Option[]) {
    const newQuestions = questions.map((question) => {
      if (question.id == id) {
        return { ...question, question: newQuestion, options };
      }
      return question;
    });
    setQuestions(newQuestions);
  }

  function deleteQuestion(id: string) {
    setQuestions(questions.filter((question) => question.id != id));
  }

  let content = questions.map((question) => {
    return (
      <InputQuiz
        question={question.question}
        options={question.options}
        updateQuiz={updateQuiz}
        deleteQuestion={deleteQuestion}
        id={question.id}
        key={question.id}
      />
    );
  });

  return content;
}
