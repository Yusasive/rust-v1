fn main() {
    let max_value = 300;
    let mut even_numbers: Vec<i32> = Vec::new(); // Sequence type to store even numbers
    
    for i in 1..=max_value {
        if i % 2 == 0 {
            println!("{}", i); // Print even number
            even_numbers.push(i); // Store even number in the sequence
        }
    }

    println!("Even numbers sequence: {:?}", even_numbers); // Print the sequence of even numbers
}
