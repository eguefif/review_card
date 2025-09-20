import styled from '@emotion/styled';

export default function EditableOption({ answer, id, editOption, deleteOption }) {
  function handleOnChangeEvent(event) {
    const newAnswer = event.target.value;
    editOption(id, newAnswer);
  }
  return (
    <Option>
      <button onClick={() => deleteOption(id)}>X</button>
      <OptionInput value={answer} onChange={handleOnChangeEvent} />
    </Option>
  );
}

const Option = styled.div``;

const OptionInput = styled.input``;
