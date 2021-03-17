let a = [1, 2, 3];
let a1 = [1, 2, 3];

a.iter().map(|x| 2 * x)
        .filter(|x| x.is_positive())
        .max();

a.iter().zip(a1.iter());

a.iter().enumerate();

let data = vec![vec![1, 2, 3, 4], vec![5, 6]];
let flattened = data.into_iter().flatten().collect::<Vec<u8>>();
