// #[cfg(test)]
use multiplication2::multiplication;
mod tests {
    use super::*;

    #[test]
    fn test_positive_multiple() {
        assert_eq!(multiplication(&3, &2), 6);
        
    }
    #[test]
    fn test_zero_multiple() {
        assert_eq!(multiplication(&0, &-5), 0);
        
    }
    #[test]
    fn test_negative_multiple() {
        assert_eq!(multiplication(&-2, &-10), 20);
    }
    #[test]
    fn large_multiple() {
        assert_eq!(multiplication(&31000, &200), 6200000);
    }
}