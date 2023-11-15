pub fn add_one(x: i32) -> i32 {
    let x = add_one_again(x);

    x + 1
}

fn add_one_again(x: i32) -> i32 {
    x + 1
}
