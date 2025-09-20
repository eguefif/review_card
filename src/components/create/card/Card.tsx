import QuizContainer from './QuizContainer';
import CardContentEditor from './CardContentEditor';
import { Quiz } from '../../../types/card';

import styled from '@emotion/styled';

interface CardProps {
  content: string;
  setContent: (content: string) => void;
  quiz: Quiz;
  setQuiz: (quiz: Quiz) => void;
}

export default function Card({ content, setContent, quiz, setQuiz }: CardProps) {
  return (
    <>
      <Result>
        <CardContentEditor content={content} setContent={setContent} />
      </Result>
      {quiz ? (
        <QuizContainer
          questions={quiz}
          setQuestions={setQuiz} />
        ) : (
        ''
      )}
    </>
  );
}

const CardForm = styled.form`
  display: flex;
  flex-direction: column;
  justify-content: start;
  align-items: center;
  gap: 16px;
`;

const Result = styled.div`
  width: 100%;
`;

const ContentInput = styled.textarea``;
