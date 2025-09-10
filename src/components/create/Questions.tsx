import styled from '@emotion/styled';
import Question from './Question.tsx';

interface Option {
  id: number,
  answer: string,
}

interface Question {
  //id: number,
  question: string,
  //options: Option[],
  options: string[],
}

export default function Questions(questions) {
  let content = questions.questions.map((question) => {
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
      {content}
  );
}

const Option = styled.li``

