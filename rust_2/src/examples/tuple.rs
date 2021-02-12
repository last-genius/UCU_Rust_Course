    let tup1: (i32, f64, u8) = (500, 6.4, 1);

    let tup2 = (500, 6.4, 1);

    let (x, y, z) = tup1;

    println!("The value of y is: {}", y);

    let five_hundred = x.0;
    let six_point_four = x.1;
    let one = x.2;
