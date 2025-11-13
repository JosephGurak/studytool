

use std::io;
use std::collections::HashMap;

#[derive(Debug)]
struct FlashCardBack {
    answer: String
}

#[derive(Eq,PartialEq,Hash,Debug)]
struct FlashCardFront {
    topic: String
}




// have user input menu for studying terms and topics in rust, AWS, Datastructs & Algorithms
// make nice CLI app for personal use
// use rand crate to mix up cards in a topic.
fn main() {
    let mut flashcards: HashMap<FlashCardFront, FlashCardBack> = HashMap::new();
    flashcards.insert(
        FlashCardFront {
            topic: "What is EC2".to_owned()
        },
        FlashCardBack {
            answer: "Elastic Compute Cloud".to_owned()
        }
    );




}
