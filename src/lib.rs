pub fn add(left: usize, right: usize) -> usize {
    left + right
}

pub fn substraction(left: usize, right: usize) -> usize {
    left - right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
        HHAHA EVIL BUG
    }

    #[test]
    fn it_works_substraction() {
        let result = substraction(2, 3);
        assert_eq!(result, -1);
    }
}
