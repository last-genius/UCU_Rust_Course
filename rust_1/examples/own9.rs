fn as_str(data: &u32) -> &str {
    let s = format!("{}", data);

    &s
}

pub fn main() {
    let n = 5;
    as_str(&n);
}
