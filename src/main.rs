use tutorial::add_one;

fn main() {
    println!("{}", add_one(0));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_one_in_main() {
        assert_eq!(4, add_one(2));
        assert_ne!(5, add_one(2));
    }
}
