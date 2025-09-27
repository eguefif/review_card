import styled from '@emotion/styled';
import { useEffect, useState } from 'react';

import {type Card } from '@types/card';
import { CardsList } from './CardsList';
import { useSelectAllCards } from '@dataStore/useDataStore';

export default function Edit() {
  const [cards, setCards] = useState();
  const { getAllCards, error } = useSelectAllCards();

  useEffect(() => {
    async function fetchCards() {
      const result = await getAllCards();
      if (result) {
        setCards(result);
      }
    }
    fetchCards();
  }, [getAllCards]);

  return (
    <Wrapper>
      <h2>Edit Cards</h2>
      <CardsList cards={cards} />
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
