enum List<T> {
    Cons(T, Box<List<T>>),
    Nil,
}

// Creating a new list
let list: List<i32> = List::Cons(1, Box::new(List::Cons(2, Box::new(List::Nil))));
println!("{:?}", list); // prints Cons(1, Cons(2, Nil))
