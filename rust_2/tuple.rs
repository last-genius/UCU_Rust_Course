    let tup1: (i32, f64, u8) = (500, 6.4, 1);

    let tup2 = (500, 6.4, 1);

    let (x, y, z) = tup1;

    println!("The value of y is: {}", y);

    let five_hundred = tup1.0;
    let six_point_four = tup1.1;
    let one = tup1.2;
