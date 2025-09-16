import styled from '@emotion/styled';
import Question from './Question.tsx';

interface Question {
  question: string,
  options: string[],
}

export default function Questions({ questions }) {
  let content = questions.map((question) => {
      console.log(question);
      return (<Question 
          question={question.question}
          options={question.options}
          key={question.question}
      >
      </Question>);
  }
  );

  return (
      content
  );
}

const Option = styled.li``

