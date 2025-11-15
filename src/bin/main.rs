

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

struct MainHashMap {
    nested_hashmap: HashMap<String, HashMap<FlashCardFront,FlashCardBack>>
}



// have user input menu for studying terms and topics in rust, AWS, Datastructs & Algorithms
// make nice CLI app for personal use
// use rand crate to mix up cards in a topic.
fn main() {
    let mut flashcards = MainHashMap{ nested_hashmap: HashMap::new()};

    
    let mut aws_flashcards: HashMap<FlashCardFront, FlashCardBack> = HashMap::new();
    aws_flashcards.insert(
        FlashCardFront {
            topic: "What is EC2".to_owned()
        },
        FlashCardBack {
            answer: "Elastic Compute Cloud".to_owned()
        }
    );


    for (key, value) in aws_flashcards {
        println!("{:?} {:?}",key,value);
    }


}
