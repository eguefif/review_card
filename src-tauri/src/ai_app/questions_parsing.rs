#[derive(Debug, serde::Serialize, serde::Deserialize)]
struct Question{
    question: String,
    response: Vec::<String>,
}

pub fn extract_json(content: &str) -> &str {

}
#[cfg(test)]

mod tests {
    use super::*;

    fn it_should_get_json_string() {
        let response = "
Here's a JSON with multiple-choice questions based on the CSS Grid review card: ```json { \"quizTitle\": \"CSS Grid MCQ Quiz\", \"questions\": [ { \"question\": \"What is CSS Grid primarily used for?\", \"options\": [ \"Styling text\", \"Creating two-dimensional layouts\", \"Animating web elements\", \"Managing database connections\" ], \"correctAnswer\": \"Creating two-dimensional layouts\" }, { \"question\": \"What CSS property is used to create a grid container?\", \"options\": [ \"grid-layout: enable\", \"display: grid\", \"grid-type: container\", \"layout: grid\" ], \"correctAnswer\": \"display: grid\" }, { \"question\": \"What does 'fr' unit represent in grid layout?\", \"options\": [ \"Fixed width\", \"Font resize\", \"Fractional unit\", \"Frame rate\" ], \"correctAnswer\": \"Fractional unit\" }, { \"question\": \"Which of the following is NOT a key concept in CSS Grid?\", \"options\": [ \"Grid Container\", \"Grid Lines\", \"Grid Tracks\", \"Grid Shadows\" ], \"correctAnswer\": \"Grid Shadows\" }, { \"question\": \"What is the primary advantage of CSS Grid over Flexbox?\", \"options\": [ \"Better performance\", \"Two-dimensional layout control\", \"More browser compatibility\", \"Easier syntax\" ], \"correctAnswer\": \"Two-dimensional layout control\" } ] } ```
    "
    }
}
