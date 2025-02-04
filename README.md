# Quiz Tool

## Overview
The `Quiz Tool` is a Rust-based CLI application that allows users to take quizzes loaded from a JSON file. The application tracks the user's progress, including correct answers, incorrect attempts, and the current question. Progress is saved and restored between sessions, making it ideal for educational or recreational purposes.

---

## Features
- **Load Questions from JSON**: Load a list of quiz questions, including options and correct answers, from a JSON file.
- **Save and Restore Progress**: Automatically saves progress to a file (`progress.json`) and restores it on the next session.
- **Difficulty Levels**: Questions can have varying difficulty levels (`Easy`, `Medium`, `Hard`).
- **Interactive Quiz**: Users answer multiple-choice questions interactively in the terminal, with immediate feedback on their answers.
- **Progress Reset**: Progress is reset upon quiz completion.

---

## Requirements
### Dependencies
The project uses the following Rust crates:
- `serde` and `serde_json`: For serializing and deserializing JSON.
- `std`: Standard library for file handling, I/O, and collections.

To install the required crates, include the following in your `Cargo.toml`:
```toml
[dependencies]
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
```

### JSON Files
The application requires two JSON files:
1. **Questions File** (`files/questions.json`): Contains the quiz questions.
    - Structure:
      ```json
      [
        {
          "id": 1,
          "text": "What is the capital of France?",
          "options": ["Paris", "Berlin", "Madrid", "Rome"],
          "correct_answer": "Paris",
          "category": "Geography",
          "difficulty": "Easy"
        }
      ]
      ```
2. **Progress File** (`files/progress.json`): Tracks the user's progress. This file is automatically generated if it doesn't exist.

---

## Usage
### Running the Quiz
1. **Build and Run**:
   ```
   cargo run
   ```
2. **Answering Questions**:
    - Enter the option number corresponding to your answer.
    - Correct answers will advance to the next question.
    - Incorrect answers allow you to retry.

3. **Progress Display**:
    - The current question, score, and incorrect attempts are displayed during the quiz.

### Files Directory
Ensure that the `files/` directory exists in the root of the project and contains the `questions.json` file.

### Resetting Progress
Progress is reset automatically upon completing the quiz or can be reset by deleting the `progress.json` file.

---

## Example Workflow
1. Add your questions to `files/questions.json`:
   ```json
   [
       {
           "id": 1,
           "text": "What is 2 + 2?",
           "options": ["3", "4", "5"],
           "correct_answer": "4",
           "category": "Math",
           "difficulty": "Easy"
       },
       {
           "id": 2,
           "text": "What is the capital of Japan?",
           "options": ["Seoul", "Tokyo", "Beijing"],
           "correct_answer": "Tokyo",
           "category": "Geography",
           "difficulty": "Medium"
       }
   ]
   ```
2. Run the application:
   ```
   cargo run
   ```
3. Answer the questions and view your score upon completion.

---

## Code Structure
### Modules
- **`Question`**: Represents a single quiz question.
- **`Progress`**: Tracks user progress during the quiz.
- **`QuizManager`**: Manages quiz logic, including question display, user input, and saving/restoring progress.

---

## Customisation
### Adding Categories or Features
- Modify the `Question` struct to include additional fields (e.g., `hints` or `tags`).
- Adjust the `QuizManager` logic to incorporate new features.

---

## Contributions
Contributions to this project are welcome. Feel free to open issues or submit pull requests.

---

## License
This project is licensed under the MIT License. See the `LICENSE` file for details.