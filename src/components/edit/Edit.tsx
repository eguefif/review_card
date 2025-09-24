import styled from '@emotion/styled';
import { invoke } from '@tauri-apps/api/core';
import { useEffect, useState } from 'react';

import {type Card } from '@types/card';
import { CardsList } from './CardsList';

export default function Edit() {
  const [cards, setCards] = useState();
  // TODO: add pagination mechanism with a function that will get

  async function get_all_cards(offset: number = 0, limit: number = 0):  Card[] {
    return await invoke('get_all_cards', { offset: 0, limit: 10 } );
  }

  useEffect(() => {
    setCards(get_all_cards());
    return () => {};
  }, []);

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
