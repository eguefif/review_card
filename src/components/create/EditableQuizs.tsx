import styled from '@emotion/styled';
import InputQuiz from './InputQuiz';

interface Question {
  id: string;
  question: string;
  options: string[];
}

interface EditableQuizsProps {
  questions: Question[];
  setQuestions: (questions: Question[]) => void;
}

export default function EditableQuizs({ questions, setQuestions }: EditableQuizsProps) {
  function updateQuiz(id: string, newQuestion: string, options: string[]) {
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

const Option = styled.li``;
