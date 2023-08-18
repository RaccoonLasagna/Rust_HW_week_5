fn count_vowels(input: &str) -> i64 {
    cv(input, 0)
}

fn cv(input: &str, index: usize) -> i64 {
    if index >= input.chars().count() {
        return 0;
    }
    let character = input.chars().nth(index).unwrap();
    let vowels = ['A', 'a', 'E', 'e', 'I', 'i', 'O', 'o', 'U', 'u'];
    let count = if vowels.contains(&character) { 1 } else { 0 };
    count + cv(input, index + 1)
}

fn main() {
    print!("{}", count_vowels("19712aihbixcvlkz/'{[}]]}}}$%&#@-+=/*-+;;:<> ラストが好き"))
}

#[test]
fn test_vowels_count1() {
    assert_eq!(count_vowels(""), 0);
    assert_eq!(count_vowels("abEcd"), 2);
    assert_eq!(count_vowels("ab12Exey5 7x8U3y5z"), 4);
    assert_eq!(count_vowels("AaEeIiOoUu"), 10);
    assert_eq!(count_vowels("19712aihbixcvlkz/'{[}]]}}}$%&#@-+=/*-+;;:<> ラストが好き"), 3);
}