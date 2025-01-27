use std::collections::HashMap;
use std::fs;
use std::io::{self, Write};
use std::path::Path;
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Question {
    id: u32,
    text: String,
    options: Vec<String>,
    correct_answer: String,
    category: Option<String>,
    difficulty: Difficulty,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Difficulty {
    Easy,
    Medium,
    Hard,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Progress {
    current_question_id: u32,
    correct_answers: u32,
    incorrect_attempts: u32,
}

impl Progress {
    pub fn new() -> Self {
        Self {
            current_question_id: 1,
            correct_answers: 0,
            incorrect_attempts: 0,
        }
    }
}

#[derive(Debug)]
pub struct QuizManager {
    questions: HashMap<u32, Question>,
    progress: Progress,
}

impl QuizManager {
    pub fn new() -> Self {
        Self {
            questions: HashMap::new(),
            progress: Progress::new(),
        }
    }

    pub fn load_from_file<P: AsRef<Path>>(path: P) -> Result<Self, Box<dyn std::error::Error>> {
        let file_content = fs::read_to_string(path)?;
        let questions: Vec<Question> = serde_json::from_str(&file_content)?;

        let mut quiz_manager = QuizManager::new();
        for question in questions {
            quiz_manager.questions.insert(question.id, question);
        }

        // Try to load progress from save file
        quiz_manager.load_progress()?;

        Ok(quiz_manager)
    }

    fn load_progress(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        match fs::read_to_string("files/progress.json") {
            Ok(content) => {
                self.progress = serde_json::from_str(&content)?;
            }
            Err(_) => {
                self.progress = Progress::new();
                self.save_progress()?;
            }
        }
        Ok(())
    }

    fn save_progress(&self) -> Result<(), Box<dyn std::error::Error>> {
        // Create files directory if it doesn't exist
        fs::create_dir_all("files")?;
        let json = serde_json::to_string(&self.progress)?;
        fs::write("files/progress.json", json)?;
        Ok(())
    }

    fn display_question(&self, question: &Question) {
        println!("\n{}", question.text);
        for (i, option) in question.options.iter().enumerate() {
            println!("{}. {}", i + 1, option);
        }
    }

    pub fn run_quiz(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let total_questions = self.questions.len() as u32;

        while self.progress.current_question_id <= total_questions {
            if let Some(question) = self.questions.get(&self.progress.current_question_id) {
                println!("\nQuestion {} of {}", self.progress.current_question_id, total_questions);
                println!("Current score: {}/{} (Incorrect attempts: {})",
                         self.progress.correct_answers,
                         self.progress.current_question_id - 1,
                         self.progress.incorrect_attempts);

                self.display_question(question);

                loop {
                    print!("\nEnter your answer (1-{}): ", question.options.len());
                    io::stdout().flush()?;

                    let mut input = String::new();
                    io::stdin().read_line(&mut input)?;

                    match input.trim().parse::<usize>() {
                        Ok(num) if num > 0 && num <= question.options.len() => {
                            let selected_answer = &question.options[num - 1];

                            if selected_answer == &question.correct_answer {
                                println!("\n✓ Correct!");
                                self.progress.correct_answers += 1;
                                self.progress.current_question_id += 1;
                                self.save_progress()?;
                                break;
                            } else {
                                println!("\n✗ Incorrect, try again!");
                                self.progress.incorrect_attempts += 1;
                                self.save_progress()?;
                                println!("\nLet's try that again...");
                                self.display_question(question);
                            }
                        }
                        _ => {
                            println!("Invalid input! Please enter a number between 1 and {}",
                                     question.options.len());
                            self.display_question(question);
                        }
                    }
                }
            }
        }

        println!("\nQuiz completed!");
        println!("Final score: {}/{}", self.progress.correct_answers, total_questions);
        println!("Total incorrect attempts: {}", self.progress.incorrect_attempts);

        // Reset progress after completion
        self.progress = Progress::new();
        self.save_progress()?;

        Ok(())
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut quiz = QuizManager::load_from_file("files/questions.json")?;
    quiz.run_quiz()?;
    Ok(())
}