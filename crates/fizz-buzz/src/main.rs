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

fn main() {
    for i in 0..=100 {
        println!("{}", fizzbuzz(i));
    }
}
