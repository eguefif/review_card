import Questions from './Questions';
import CardContentEditor from './CardContentEditor';

import styled from '@emotion/styled';

export default function Card({ content, setContent, questions }) {
  return (
    <>
      <Result>
        <CardContentEditor content={content} setContent={setContent} />
      </Result>
      {questions ? <Questions questions={questions} /> : ''}
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
