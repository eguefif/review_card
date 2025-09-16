import styled from '@emotion/styled';

export default function Prompt({ submit, setPrompt, prompt}) {

  return (
  <>
      <Label focus>Card Content</Label>
      <StyledTextArea name="prompt"
        value={prompt}
        onChange={setPrompt}
        placeHolder="My review card is about"
        onKeyDown={submit}
      />
  </>);
}

const Label = styled.label`
`;

const StyledTextArea = styled.textarea`
  width: 100%;
  height: 100px;
`;
