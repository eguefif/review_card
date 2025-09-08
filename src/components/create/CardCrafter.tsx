import { useState, useEffect } from 'react';
import styled from '@emotion/styled';
import { invoke } from '@tauri-apps/api/core';
j
import { marked } from 'marked';

import { Questions } from './Questions';

export default function CardCrafter() {
  const [result, setResult] = useState('');
  const [prompt, setPrompt] = useState('');
  const [topic, setTopic] = useState('');
  const [questions, setQuestions] = useState('');

  async function promptAi() {
    const [ result, questions ] = await invoke('prompt_ai', { topic: topic });
    setResult(marked(result));
    setQuestions(questions);
  }

  function submitOnEnterKey(event) {
    if (event.key === 'Enter') {
      promptAi();
    }
  }

=======
  function updatePrompt(e: React.ChangeEvent<HTMLTextAreaElement>) {
    setTopic(e.target.value);
  }

  return (
    <Wrapper>
      <h1>Create Your Review Card</h1>
      <CardForm action={promptAi}>
        <Result>
          {result ? result : ''}
        </Result>
        {questions ? <Questions questions={questions} /> : ''}
        <ContentWrapper>
          <Label focus>Prompt</Label>
          <Prompt name="prompt"
            value={prompt}
            onChange={updatePrompt}
            placeHolder="My review card is about"
            onKeyDown={submitOnEnterKey}
          />
        </ContentWrapper>
      </CardForm>
    </Wrapper>
  )
}

const Result = styled.div`
  width: 100%;
`;

const Wrapper = styled.div`
  width: 75%;
  margin: auto;
  display: flex;
  flex-direction: column;
  justify-content: start;
  align-items: center;
`;

const CardForm = styled.form`
  width: 50%;
  display: flex;
  flex-direction: column;
  justify-content: start;
  align-items: center;
  gap: 16px;
`;

const Label = styled.label`
`;

const ContentWrapper = styled.div`
  width: 100%;
  display: flex;
  flex-direction: column;
  justify-content: start;
  align-items: center;
  gap: 8px;
`;

const Prompt = styled.textarea`
  width: 100%;
  height: 100px;
`;

const SubmitButton = styled.button`
`
