import styled from '@emotion/styled';

export default function CardCrafter() {

  function createCard(formData) {
    console.log(formData.get('title'));
    console.log(formData.get('content'));
  }

  return (
    <Wrapper>
      <h1>Create Your Review Card</h1>
      <CardForm action={createCard}>
        <InputWrapper>
          <Label>Title</Label>
          <TitleInput name="title" />
        </InputWrapper>
        <ContentWrapper>
          <Label>Card Content</Label>
          <CardContent name="content"/>
        </ContentWrapper>
        <SubmitButton type="submit">Create</SubmitButton>
      </CardForm>
    </Wrapper>
  )
}

const Wrapper = styled.div`
  width: 75%;
  margin: auto;
  display: flex;
  flex-direction: column;
  justify-content: start;
  align-items: center;
`;

const CardForm = styled.form`
  width: 50%;
  display: flex;
  flex-direction: column;
  justify-content: start;
  align-items: center;
  gap: 16px;
`;

const InputWrapper = styled.div`
  display: flex;
  gap: 8px;
  width: 50%;
`

const Label = styled.label`
`;

const TitleInput = styled.input`
  width: 100%; 
`;

const ContentWrapper = styled.div`
  width: 100%;
  display: flex;
  flex-direction: column;
  justify-content: start;
  align-items: center;
  gap: 8px;
`;

const CardContent = styled.textarea`
  width: 100%;
  height: 344px;
`;

const SubmitButton = styled.button`
`
