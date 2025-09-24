import styled from '@emotion/styled';

export default function Review() {
  return (
    <Wrapper>
      <h2>Review Cards</h2>
      <p>This is a placeholder for the review functionality.</p>
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