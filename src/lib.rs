mod diff;


pub fn delta(prev: String, cur: String) -> String {
    return diff::longest_common_subsequence(prev, cur);
}

#[test]
fn test_delta() {
    assert_eq!(delta(String::from("abcde"), String::from("abfeg")).len(), 3);
    assert_eq!(delta(String::from("abcde"), String::from("")).len(), 0);
    assert_eq!(delta(String::from("abc"), String::from("abdefg")).len(), 2);
}