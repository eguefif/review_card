import styled from '@emotion/styled';
import InputQuiz from './InputQuiz';

interface Question {
  question: string;
  options: string[];
}

export default function EditableQuizs({ questions, updateQuiz }) {
  let content = questions.map((question) => {
    return (
      <InputQuiz
        question={question.question}
        options={question.options}
        updateQuiz={updateQuiz}
        id={question.id}
        key={question.id}
      />
    );
  });

  return content;
}

const Option = styled.li``;
