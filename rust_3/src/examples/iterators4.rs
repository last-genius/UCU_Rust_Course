let mut values = vec![41];

for x in values.iter_mut() {
    *x += 1;
}

for x in &mut values { // same as `values.iter_mut()`
    *x += 1;
}
