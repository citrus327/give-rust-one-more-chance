pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    fn it_works_too() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    fn must_fail() {
        panic!("Make this test fail");
    }
}
