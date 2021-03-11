let mut values = vec![41];

for x in values.into_iter() {
    *x += 1;
}

for x in values { // same as `values.into_iter()`
    *x += 1;
}
