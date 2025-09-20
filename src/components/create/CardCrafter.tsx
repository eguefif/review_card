import { useState, useEffect } from 'react';
import styled from '@emotion/styled';
import { invoke } from '@tauri-apps/api/core';
import { Card as CardType, Question } from '../../types/card';

import Card from './Card';
import Prompt from './Prompt';

export default function CardCrafter() {
  const [error, setError] = useState('');
  const [content, setContent] = useState('');
  const [prompt, setPrompt] = useState('');
  const [topic, setTopic] = useState('');
  const [questions, setQuestions] = useState<Question[]>([]);

  async function promptAi() {
    try {
      const [result, questions] = await invoke('prompt', { topic: topic });
      setContent(result);
      setQuestions(questions);
    } catch (error) {
      console.log(error);
      setError("There was a problem with the Anthropic API. Retry later");
    }
  }

  function onPromptSubmit(event: React.ChangeEvent<HTMLTextAreaElement>) {
    if (event.key === 'Enter') {
      try {
        promptAi();
      } catch (error) {
        console.log(error);
        setError("There was a problem with the Anthropic API. Retry later");
      }
    }
  }

  function updatePrompt(e: React.ChangeEvent<HTMLTextAreaElement>) {
    setTopic(e.target.value);
  }

  async function persistData() {
    console.log('saving...');
    const card: CardType = { content, questions };
    let result = await invoke('save_card', { card });
  }

  return (
    <Wrapper>
      <h1>Create Your Review Card</h1>

      <ContentWrapper>
        <PromptWrapper>
          <Prompt
            submit={onPromptSubmit}
            setPrompt={updatePrompt}
            prompt={topic}
          />
          {content ? <SaveButton onClick={persistData}>Save</SaveButton> : ''}
        </PromptWrapper>

        <CardWrapper>
          {
            error.length === 0
              ? <Card
                content={content}
                setContent={setContent}
                questions={questions}
                setQuestions={setQuestions}
              />
              : <ErrorMessage>{error}</ErrorMessage>
          }
        </CardWrapper>
      </ContentWrapper>
    </Wrapper>
  );
}

const Wrapper = styled.div`
  width: 75%;
  margin: auto;
  display: flex;
  flex-direction: column;
  justify-content: start;
  align-items: center;
`;

const ContentWrapper = styled.div`
  width: 100%;
  display: flex;
  flex-direction: row;
  justify-content: center;
  align-items: start;
  gap: 64px;
`;

const PromptWrapper = styled.div`
  display: flex;
  flex: 1;
  flex-direction: column;
  justify-content: start;
  align-items: center;
  gap: 8px;
`;

const CardWrapper = styled.div`
  flex: 3;
`;

const SaveButton = styled.button``;

const ErrorMessage = styled.h3`
  text-align: center;
  color: red;
`;
