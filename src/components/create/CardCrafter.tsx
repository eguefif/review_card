import { useState, useEffect } from 'react';
import styled from '@emotion/styled';
import { invoke } from '@tauri-apps/api/core';

import { Card as CardType, Quiz } from '../../types/card';
import Card from './card/Card';
import Prompt from './prompt/Prompt';
import { H1 } from '@reusables/Titles.tsx';
import { useAddCard } from '@dataStore/useDataStore';

export default function CardCrafter() {
  const [errorMessage, setError] = useState('');
  const [content, setContent] = useState('');
  const [prompt, setPrompt] = useState('');
  const [topic, setTopic] = useState('');
  const [quiz, setQuiz] = useState<Quiz>([]);

  const { addCard, error } = useAddCard();

  async function promptAi() {
    try {
      const [result, quiz] = await invoke('prompt', { topic: topic });
      setContent(result);
      setQuiz(quiz);
    } catch (e) {
      console.log(e);
      setError("There was a problem with the Anthropic API. Retry later");
    }
  }

  function onPromptSubmit(event: React.ChangeEvent<HTMLTextAreaElement>) {
    if (event.key === 'Enter') {
      try {
        promptAi();
      } catch (e) {
        console.log(e);
        setError("There was a problem with the Anthropic API. Retry later");
      }
    }
  }

  function updatePrompt(e: React.ChangeEvent<HTMLTextAreaElement>) {
    setTopic(e.target.value);
  }

  async function persistData() {
    console.log('saving...');
    // Split the content into title and body
    // using the first return to line
    const splitCharIndex = content.indexOf('\n');
    const title = content.substring(0, splitCharIndex);
    const body = content.substring(splitCharIndex + 1, content.length);

    const serializedQuiz = JSON.stringify(quiz);

    console.log('title: ', title)
    console.log('content: ', body)
    console.log('quiz: ', serializedQuiz)
    await addCard({ title, content, quiz: serializedQuiz });
    //let result = await invoke('save_card', { content: content, questions: quiz });
  }

  return (
    <Wrapper>
      <H1>Create Your Review Card</H1>

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
            errorMessage.length === 0
              ? <Card
                content={content}
                setContent={setContent}
                quiz={quiz}
                setQuiz={setQuiz}
              />
              : <ErrorMessage>{errorMessage}</ErrorMessage>
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
