import styled from '@emotion/styled';

export default function EditableOption({ answer, id, editOption }) {
  function handleOnChangeEvent(event) {
    const newAnswer = event.target.value;
    editOption(id, newAnswer);
  }
  return (
    <Option>
      <OptionInput value={answer} onChange={handleOnChangeEvent} />
    </Option>
  );
}

const Option = styled.div``;

const OptionInput = styled.input``;
