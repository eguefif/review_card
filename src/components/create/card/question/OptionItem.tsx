import styled from '@emotion/styled';

interface OptionItemProps {
  answer: string;
  id: string;
  editOption: (id: string, newAnswer: string) => void;
  deleteOption: (id: string) => void;
}

export default function OptionItem({ answer, id, editOption, deleteOption }: OptionItemProps) {
  function handleOnChangeEvent(event) {
    const newAnswer = event.target.value;
    editOption(id, newAnswer);
  }
  return (
    <OptionWrapper>
      <button onClick={() => deleteOption(id)}>X</button>
      <OptionInput value={answer} onChange={handleOnChangeEvent} />
    </OptionWrapper>
  );
}

const OptionWrapper = styled.div``;

const OptionInput = styled.input``;
