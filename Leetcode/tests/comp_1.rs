mod comp_1 {
    #[test]
    fn test_remove_duplicates() {
        use crate::remove_duplicates;
        assert_eq!(remove_duplicates("abbaca".to_string()), "ca");
        assert_eq!(remove_duplicates("azxxzy".to_string()), "ay");
    }
}
