use rational::*;

#[test]
fn can_be_used_in_a_hashmap() {
    use std::collections::HashMap;

    let mut hash_map = HashMap::new();
    hash_map.insert(Rational::new(1, 2), "half");
    hash_map.insert(Rational::new(1, -3), "negative third");

    assert_eq!(hash_map[&Rational::new(1, 2)], "half");
    assert_eq!(hash_map[&Rational::new(1, -3)], "negative third");
}