import Database from '@tauri-apps/plugin-sql';

const db = await Database.load('sqlite:cardreview.db');

/**
 * Inserts a new card into the database
 * @param title - Card title
 * @param content - Card content/body text
 * @param quiz - JSON serialized quiz data
 * @returns Database execution result
 */
const insertCard = async (title: string, content: string, quiz: string) => {
  const result = await db.execute('INSERT INTO cards (title, content, quiz) VALUES ($1, $2, $3)', [title, content, quiz]);
  return result;
};

/**
 * Retrieves cards from the database with optional pagination
 * @param offset - Number of records to skip (default: 0)
 * @param limit - Maximum number of records to return (default: null for all)
 * @returns Array of card records
 */
const selectAllCards = async (offset: number = 0, limit: number = null) => {
  let result = null;
  if (limit) {
    const query = 'SELECT * FROM cards OFFSET $1 LIMIT $2;';
    result = await db.execute(query, [offset, limit]);
    console.log(result);
  } else {
    const query = 'SELECT * FROM cards;';
    result = await db.select(query, [offset]);
  }
  return result;
}

/**
 * Updates an existing card in the database
 * @param id - Card ID to update
 * @param title - New card title
 * @param content - New card content/body text
 * @param quiz - New JSON serialized quiz data
 * @returns Database execution result
 */
const updateCard = async (id: number, title: string, content: string, quiz: string) => {
  const query = 'UPDATE cards SET title = $2, content = $3, quiz = $4 WHERE id = $1'
  return await db.execute(query, [id, title, content, quiz]);
}

export { insertCard, selectAllCards, updateCard };
