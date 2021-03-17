let s1 = String::from("Linux");
let s2 = String::from("Club");
let s3 = String::from("UCU");

// String concatenation
let s = format!("{}_{}@{}", s1, s2, s3);
assert_eq!(s, "Linux_Club@UCU");

let s = s1 + "_" + &s2 + "@" + &s3;
assert_eq!(s, "Linux_Club@UCU");

// YOU CAN'T INDEX A STRING IN RUST!
&s[0];

