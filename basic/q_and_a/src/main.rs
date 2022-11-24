// question and answer class
use std::io;

struct Question {
    question: String,
    answer: String,
}

// implement Question class
impl Question {
    pub fn new(question: &str, answer: &str) -> Question {
        Question {
            question: question.to_string(),
            answer: answer.to_string(),
        }
    }
}

fn main() {
    // create questions
    let q1 = Question::new("What is the capital of France?", "Paris");

    // get input from user
    let mut input = String::new();
    println!("{} ", q1.question);
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    // check answer
    if input.trim() == q1.answer {
        println!("Correct!");
    } else {
        println!("Incorrect!");
    }
}
