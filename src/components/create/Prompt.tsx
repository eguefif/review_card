import styled from '@emotion/styled';
import { ChangeEvent, KeyboardEvent } from 'react';

interface PromptProps {
  submit: (e: KeyboardEvent<HTMLTextAreaElement>) => void;
  setPrompt: (e: ChangeEvent<HTMLTextAreaElement>) => void;
  prompt: string;
}

export default function Prompt({ submit, setPrompt, prompt }: PromptProps) {
  return (
    <>
      <Label>Card Content</Label>
      <StyledTextArea
        name="prompt"
        value={prompt}
        onChange={setPrompt}
        placeholder="My review card is about"
        onKeyDown={submit}
      />
    </>
  );
}

const Label = styled.label``;

const StyledTextArea = styled.textarea`
  width: 100%;
  height: 100px;
`;
