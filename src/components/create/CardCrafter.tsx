import { useState, useEffect } from 'react';
import styled from '@emotion/styled';
import { invoke } from '@tauri-apps/api/core';

import Card from './Card';
import Prompt from './Prompt';

export default function CardCrafter() {
  const [content, setContent] = useState('');
  const [prompt, setPrompt] = useState('');
  const [topic, setTopic] = useState('');
  const [questions, setQuestions] = useState('');

  async function promptAi() {
    const [result, questions] = await invoke('prompt_ai', { topic: topic });
    setContent(result);
    setQuestions(questions);
  }

  function onPromptSubmit(event) {
    if (event.key === 'Enter') {
      promptAi();
    }
  }

  function updatePrompt(e: React.ChangeEvent<HTMLTextAreaElement>) {
    setTopic(e.target.value);
  }

  function updateQuiz(id, newQuestion, options) {
    const newQuestions = questions.map((question) => {
      if (question.id == id) {
        return { ...question, question: newQuestion, options };
      }
      return question;
    });
    setQuestions(newQuestions);
  }

  async function persistData() {
    console.log('saving...');
    let result = await invoke('save_card',{ card: {content: content, questions: questions } }); 
  }

  return (
    <Wrapper>
      <h1>Create Your Review Card</h1>

      <ContentWrapper>
        <PromptWrapper>
          <Prompt
            submit={onPromptSubmit}
            setPrompt={updatePrompt}
            prompt={prompt}
          />
          {content ? <SaveButton onClick={persistData}>Save</SaveButton> : ''}
        </PromptWrapper>

        <CardWrapper>
          <Card
            content={content}
            setContent={setContent}
            questions={questions}
            updateQuiz={updateQuiz}
          />
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
