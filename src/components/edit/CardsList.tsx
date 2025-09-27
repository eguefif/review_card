import { type Card } from '@types/Card';
import { getAllCards } from '@dataStore/useDataStore';

export function CardsList({ cards } : { cards: Card[] }) {
  console.log('In cardslist: ', cards);
  if (!cards || cards?.length === 0) {
    return '';
  }

  const cardsList = cards.map((card: Card) => {
    return <li>{card.title}</li>;
  });

  return (
    <ul>
      { cardsList }
    </ul>
  );
}
