

use std::io;
use std::collections::HashMap;


#[derive(Debug)]
struct FlashCardBack {
    answer: String
}

#[derive(Eq,PartialEq,Hash,Debug)]
struct FlashCardFront {
    category: String,
    question: String
}


#[derive(Debug)]
struct AwsFlashcards {
    card: HashMap<FlashCardFront, FlashCardBack>
}

impl AwsFlashcards {
    fn new() -> Self {
        Self {
            card: HashMap::new(),
        }
    }

    fn add_card(&mut self, key: FlashCardFront, value: FlashCardBack) {
        self.card.insert(key,value);
    }

    fn get_answer(&self, key: &FlashCardFront) -> Option<&FlashCardBack> {
        self.card.get(key)
    }

}


// have user input menu for studying terms and topics in rust, AWS, Datastructs & Algorithms
// make nice CLI app for personal use
// use rand crate to mix up cards in a topic.
fn main() {

   // let mut aws_flashcards: HashMap<FlashCardFront, FlashCardBack> = HashMap::new();
    let mut aws_flashcards = AwsFlashcards::new();

    let front = FlashCardFront {
            category: "cloud concepts".to_owned(),
            question: "What is EC2".to_owned()
    };
    let back = FlashCardBack {
            answer: "Elastic Compute Cloud".to_owned()
    };

    aws_flashcards.add_card(front,back);


    // work on better structture for this and searching via category or question
    let question = FlashCardFront {
        category: "cloud concepts".to_owned(),
        question: "What is EC2".to_owned(),
    };
    println!("{:?}", aws_flashcards.get_answer(&question));

}
