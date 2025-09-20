pub struct AppData {
    pub next_card_id: u64,
}

impl AppData {
    /// Returns the next available card ID and increments the internal counter.
    pub fn get_next_card_id(&mut self) -> u64 {
        let retval = self.next_card_id;
        self.next_card_id += 1;

        return retval;
    }
}
