import { type Card } from '@types/Card';

export function CardsList({ cards } : { cards: Card[] }) {
  if (!cards || cards?.length() === 0) {
    return '';
  }

  const cardsList = cards.map((card: Card) => {
    const title = card.content.split('\n');
    return <li>{title}</li>;
  });

  return (
    <ul>
      { cardsList }
    </ul>
  );
}
