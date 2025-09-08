import styled from '@emotion/styled';

interface Question {
    question: string,
    options: string[],
}
export default function Questions(questions) {

    let content = questions.map((question) => { 
    <Question 
        question={question.question}
        options={question.options}
            >
    </Question>;
    });

    return (
        {content}
    );
}

function Question(question, options) {

    let options_component = options.map((option) => <Option>{option}</Option>);
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

const Option = styled.li``
