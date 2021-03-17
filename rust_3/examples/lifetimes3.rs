fn main() {
    println!("{}", first_word("word1 word2"))
}

fn first_word<'a>(s: &'a str) -> &'a str {
    s.split(' ').collect::<Vec<&'a str>>()[0]
}
