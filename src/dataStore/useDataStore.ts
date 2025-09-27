import { useState, useCallback } from 'react';
import { insertCard, selectAllCards, updateCard } from './cardStore';

interface CardData {
  id?: number,
  title: string,
  content: string,
  quiz: string, // Json serialized quiz
}

/**
 * React hook for adding cards to the database with error handling
 * @returns Object containing addCard function and error state
 */
export function useAddCard() {
  const [error, setError] = useState();

  const addCard = useCallback(async (data: CardData) => {
    try {
      const result = await insertCard(data.title, data.content, data.quiz)
    } catch(e) {
      setError(`Database error: ${e}`);
    }
  }, []);

  return {
    addCard,
    error
  }
}

/**
 * React hook for fetching all cards from the database with pagination support
 * @returns Object containing getAllCards function and error state
 */
export function useSelectAllCards() {
  const [error, setError] = useState();

  const getAllCards = useCallback(async (offset: number = 0, limit: number = null) => {
    try {
      const result = await selectAllCards(offset, limit);
      return result;
    } catch(e) {
      setError(`Database error: ${e}`);
    }
  }, []);

  return {
    getAllCards,
    error
  }
}

/**
 * React hook for updating existing cards in the database with error handling
 * @returns Object containing updateCard function and error state
 */
export function useUpdateCard() {
  const [error, setError] = useState();

  const updateCardData = useCallback(async (data: CardData) => {
    try {
      return await updateCard(data.id, data.title, data.content, data.quiz);
    } catch(e) {
      setError(`Database error: ${e}`);
    }
  }, []);

  return {
    updateCard: updateCardData,
    error
  }
}
