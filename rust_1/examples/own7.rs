fn borrow(text: &String) {
    println!("I immutably borrowed this text: {}", text);
}

fn mut_borrow(text: &mut String) {
    text.push_str(" + new text");
    println!("I mutably borrowed text and changed it to: {}", text);
}

fn main() {
    let mut og_text = "old text".to_string();

    borrow(&og_text);
    borrow(&og_text);
    mut_borrow(&mut og_text);
}
