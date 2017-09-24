/// Adds one to the number given.
///
/// # Examples
///
/// ```
/// let five = 5;
///
/// assert_eq!(7, adder::add_two(five));
/// ```
pub fn add_two(a: i32) -> i32 {
    a + 2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_adds_two_and_two() {
        assert_eq!(4, add_two(2));
    }

    #[test]
    fn it_adds_three_and_two() {
        assert_eq!(5, add_two(3));
    }

    #[test]
    #[ignore]
    fn one_hundred() {
        assert_eq!(102, add_two(100));
    }
}
