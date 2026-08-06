pub fn find_vowels() {
    let word: String = String::from("django");
    let word_two: usize = vowels_func(&word);
    println!("there are {} vowels from this word {}", word_two, word);
}

fn vowels_func(my_param: &String) -> usize {
    let mut count = 0;

    for ch in my_param.chars() {
        if ch == 'a' || ch == 'A' {
            count += 1;
        } else if ch == 'e' || ch == 'E' {
            count += 1;
        } else if ch == 'i' || ch == 'I' {
            count += 1;
        } else if ch == 'o' || ch == 'O' {
            count += 1;
        } else if ch == 'u' || ch == 'U' {
            count += 1;
        }
    }

    count
}
