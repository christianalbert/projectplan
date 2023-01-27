fn main() {
    println!("Hello, world!");
}

// fucntion that returns the square of a number
fn square(x: i32) -> i32 {
    x * x
}

// 
// Path: src/lib.rs
#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn test_square() {
       assert_eq!(square(2), 4);
       assert_eq!(square(3), 9);
       assert_eq!(square(4), 16);
       assert_eq!(square(5), 25);
       assert_eq!(square(6), 36);
       
    }
}

