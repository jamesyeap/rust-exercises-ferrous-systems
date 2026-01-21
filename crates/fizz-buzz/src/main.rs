fn fizzbuzz(input: i32) -> String {
    if input % (3 * 5) == 0 {
        "FizzBuzz".into()
    } else if input % 3 == 0 {
        "Fizz".into()
    } else if input % 5 == 0 {
        "Buzz".into()
    } else {
        input.to_string()
    }
}

fn fizzbuzz_with_match_pattern_matching(input: i32) -> String {
    let remainders = (input % 3, input % 5);
    match remainders {
        (0, 0) => "FizzBuzz".into(),
        (0, _) => "Fizz".into(),
        (_, 0) => "Buzz".into(),
        _ => input.to_string(),
    }
}

fn main() {
    for i in 0..=100 {
        // println!("{}", fizzbuzz(i));
        println!("{}", fizzbuzz_with_match_pattern_matching(i));
    }
}

#[cfg(test)]
mod test {
    use crate::{fizzbuzz, fizzbuzz_with_match_pattern_matching};

    #[test]
    fn should_match() {
        for i in 0..=100 {
            assert_eq!(fizzbuzz(i), fizzbuzz_with_match_pattern_matching(i));
        }
    }
}
