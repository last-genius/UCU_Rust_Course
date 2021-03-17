let mut values = vec![41];

for x in values.iter() {
    assert_eq!(*x, 42);
}

for x in &values { // same as `values.iter()`
    assert_eq!(*x, 42);
}

