fn main() {
    println!("{}", first_word("word1 word2"))
}

fn first_word(s: &str) -> &str {
    s.split(' ').collect::<Vec<&str>>()[0]
}
