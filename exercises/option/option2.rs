// option2.rs
// Make me compile! Execute `rustlings hint option2` for hints

// Add a trait for multiplying and in-place modifying the values in the Vec, also check for overflow.
trait MultiplyBy {
    fn multiply_by(&mut self, i: i8) -> &mut Self;
}

impl MultiplyBy for Vec<Option<i8>> {
    fn multiply_by(&mut self, i: i8) -> &mut Vec<Option<i8>> {
        (0..self.len()).for_each(|pos| {
            if let Some(v) = self[pos] {
                self[pos] = v.checked_mul(i);
            }
        });
        self
    }
}

fn main() {
    let optional_word = Some(String::from("rustlings"));
    // TODO: Make this an if let statement whose value is "Some" type
    if let Some(word) = optional_word {
        println!("The word is: {}", word);
    } else {
        println!("The optional word doesn't contain anything");
    }

    let mut optional_integers_vec: Vec<Option<i8>> = Vec::new();
    for x in 1..10 {
        optional_integers_vec.push(Some(x));
    }
    optional_integers_vec.multiply_by(20);

    // TODO: make this a while let statement - remember that vector.pop also adds another layer of Option<T>
    // You can stack `Option<T>`'s into while let and if let
    while let Some(v) = optional_integers_vec.pop() {
        if let Some(integer) = v {
            println!("current value: {}", integer);
        } else {
            println!("no value because of overflow!");
        }
    }
    
}
