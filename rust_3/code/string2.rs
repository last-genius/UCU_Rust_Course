// You can slice it, but it will panic if you try
// to slice inside a UTF-8 character
&s[0..4];

// It's better to iterate over the array with these methods:
for c in s.chars() {
    println!("{}", c);
}

for b in s.bytes() {
    println!("{}", b);
}
