import EditableQuizs from './EditableQuizs';
import CardContentEditor from './CardContentEditor';
import { Question } from '../../../types/card';

import styled from '@emotion/styled';

interface CardProps {
  content: string;
  setContent: (content: string) => void;
  questions: Question[];
  setQuestions: (questions: Question[]) => void;
}

export default function Card({ content, setContent, questions, setQuestions }: CardProps) {
  return (
    <>
      <Result>
        <CardContentEditor content={content} setContent={setContent} />
      </Result>
      {questions ? (
        <EditableQuizs
          questions={questions}
          setQuestions={setQuestions} />
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
