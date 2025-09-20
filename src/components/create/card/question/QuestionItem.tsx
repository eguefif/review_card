import styled from '@emotion/styled';
import OptionItem from './OptionItem';
import { Option } from '../../../../types/card';

interface QuestionItemProps {
  question: string;
  options: Option[];
  updateQuiz: (id: string, newQuestion: string, options: Option[]) => void;
  deleteQuestion: (id: string) => void;
  id: string;
}

export default function QuestionItem({ question, options, updateQuiz, deleteQuestion, id }: QuestionItemProps) {
  let options_component = options.map((option) => {
    return (
      <OptionItem
        key={option.id}
        answer={option.answer}
        id={option.id}
        editOption={editOption}
        deleteOption={deleteOption}
      />
    );
  });

  function editQuestion(event) {
    const newQuestion = event.target.value;
    updateQuiz(id, newQuestion, options);
  }

  function editOption(optionId, newAnswer) {
    const newOptions = options.map((option) => {
      if (option.id == optionId) {
        return { ...option, answer: newAnswer };
      }
      return option;
    });
    updateQuiz(id, question, newOptions);
  }

  function deleteOption(optionId) {
    const newOptions = options.filter((option) => option.id != optionId);
    updateQuiz(id, question, newOptions);
  }

  return (
    <Wrapper>
      <QuestionWrapper>
        <button 
          onClick={() => deleteQuestion(id)}
          >
            X
          </button>
        <Label>Question</Label>
        <QuestionInput value={question} onChange={editQuestion} />
      </QuestionWrapper>

      <AnswersWrapper>{options_component}</AnswersWrapper>
    </Wrapper>
  );
}

const Wrapper = styled.div`
  width: 100%;
  margin: 16px;
  display: flex;
  flex-direction: column;
  gap: 8px;
`;

const QuestionWrapper = styled.div`
  width: 100%;
  display: flex;
  flex-direction: row;
  justify-content: start;
  align-items: baseline;
  gap: 16px;
`;

const QuestionInput = styled.input`
  width: 100%;
  padding: 8px;
  font-size: 1.2rem;
`;

const Label = styled.label``;

const AnswersWrapper = styled.div`
  display: flex;
  flex-direction: column;
  gap: 4px;
  margin-left: 4px;
`;
