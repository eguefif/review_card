import styled from '@emotion/styled';

export default function Edit() {
  return (
    <Wrapper>
      <h2>Edit Cards</h2>
      <p>This is a placeholder for the edit functionality.</p>
    </Wrapper>
  );
}

const Wrapper = styled.div`
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: 16px;
  padding: 24px;
`;