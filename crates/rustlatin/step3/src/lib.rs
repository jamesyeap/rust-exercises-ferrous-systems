#![allow(unused)]

use std::ops::Index;

fn rustlatin(sentence: &str) -> Vec<char> {
    //                           ^^^^^^^
    // The correct return type needs to be added by you,
    // depending on what the vector's exact type is.
    let mut collection_of_chars = Vec::new();

    for word in sentence.split(" ") {
        if let Some(first_char) = word.chars().nth(0) {
            collection_of_chars.push(first_char);
        }
    }

    collection_of_chars
}

// Uncomment this:
#[test]
fn return_the_char() {
    assert_eq!(
        vec!['n', 't', 'd', 'b', 'i', 'a', 'r', 'v'],
        rustlatin("note the difference between iterator and return values")
    )
}
