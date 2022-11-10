use leetcode::components::comp_1::*;

#[test]
fn test_integration() {
    assert_eq!(remove_duplicates("abbaca".to_string()), "ca");
    assert_eq!(remove_duplicates("azxxzy".to_string()), "ay");
}
