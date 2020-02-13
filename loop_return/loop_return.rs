fn main() {
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter % 5 == 0 && counter % 3 == 0 {
            break counter;
        }
    };

    println!("{}", result)
}