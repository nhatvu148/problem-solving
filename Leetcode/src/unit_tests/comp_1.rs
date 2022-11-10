mod comp_1_tests {
    use crate::components::comp_1::*;

    #[test]
    fn test_remove_duplicates() {
        assert_eq!(remove_duplicates("abbaca".to_string()), "ca");
        assert_eq!(remove_duplicates("azxxzy".to_string()), "ay");
    }

    #[test]
    fn test_stocker_spanner() {
        let mut obj = StockSpanner::new();
        assert_eq!(obj.next(100), 1);
        assert_eq!(obj.next(80), 1);
        assert_eq!(obj.next(60), 1);
        assert_eq!(obj.next(70), 2);
        assert_eq!(obj.next(60), 1);
        assert_eq!(obj.next(75), 4);
        assert_eq!(obj.next(85), 6);
    }

    #[test]
    fn test_make_good() {
        assert_eq!(make_good("leEeetcode".to_string()), "leetcode".to_string());
        assert_eq!(make_good("abBAcC".to_string()), "".to_string());
        assert_eq!(make_good("s".to_string()), "s".to_string());
    }

    #[test]
    fn test_maximum69_number() {
        assert_eq!(maximum69_number(9669), 9969);
        assert_eq!(maximum69_number(9996), 9999);
        assert_eq!(maximum69_number(9999), 9999);
    }

    #[test]
    fn test_reverse_vowels() {
        assert_eq!(reverse_vowels("hello".to_string()), "holle".to_string());
        assert_eq!(
            reverse_vowels("leetcode".to_string()),
            "leotcede".to_string()
        );
    }

    #[test]
    fn test_two_sum() {
        assert_eq!(two_sum(vec![2, 7, 11, 15], 9), vec![0, 1]);
        assert_eq!(two_sum(vec![3, 2, 4], 6), vec![1, 2]);
        assert_eq!(two_sum(vec![3, 3], 6), vec![1, 0]);
    }
}