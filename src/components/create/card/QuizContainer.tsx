import styled from '@emotion/styled';
import QuestionItem from './question/QuestionItem';
import { Quiz as QuizType, Option } from '../../../types/card';

interface QuizContainerProps {
  questions: QuizType;
  setQuestions: (questions: QuizType) => void;
}

export default function QuizContainer({ questions, setQuestions }: QuizContainerProps) {
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
      <QuestionItem
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
