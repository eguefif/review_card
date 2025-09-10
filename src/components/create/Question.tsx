import styled from '@emotion/styled';

export default function Question({question, options}) {
  console.log(options);
    let options_component = options.map((option) => <Option key={option}>{option}</Option>);
    return (
    <QuestionWrapper>
            <h3>{question}</h3>
            <ul>
            { options_component }
            </ul>
    </QuestionWrapper>
    );
}

const QuestionWrapper = styled.div`
    display: flex;
    flex-direction: column;
`;


const Option = styled.li``;
