import EditableQuizs from './EditableQuizs';
import CardContentEditor from './CardContentEditor';

import styled from '@emotion/styled';

export default function Card(
  { content, setContent, questions, updateQuiz}
) {
  return (
    <>
      <Result>
        <CardContentEditor content={content} setContent={setContent} />
      </Result>
      { questions
          ? <EditableQuizs
            questions={questions} 
            updateQuiz={updateQuiz}
          />
          : ''
      }
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
