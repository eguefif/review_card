pub fn build_review_card_prompt(topic: &str) -> String {
    format!(
        "I want you to create a review card. This review card should only be text and separate in meaningfull sections. Can you create a review card for the following topic?\n\n{}",
        topic
    )
}

pub fn get_mcq_example() -> &'static str {
    r#"
        [
            {
                "question": "What is France's capital city",
                "options": ["Paris", "Lyon", "Marseille"]
            },
            {
                "question": "What is the baby of the bear",
                "options": ["Kitty", "Cub", "Lion"]
            }
        ]
    "#
}

pub fn get_mcq_format() -> &'static str {
    r#"
        [
            {
                "question": "string",
                "options": ["string"]
            }
        ]
    "#
}

pub fn build_mcq_prompt(card: &str) -> String {
    let example = get_mcq_example();
    let format = get_mcq_format();

    format!(
        "Based on the following review card, can you generate a few questions and answers for a MCQ quiz? You MUST return ONLY a JSON array with no wrapper object.\n\nReview card: {}\n\nRequired JSON format: {}\n\nExample output: {}\n\nIMPORTANT: Return ONLY the JSON array, not wrapped in any object like {{\"quizQuestions\": ...}}.",
        card, format, example
    )
}