fn count_vowels(input: &str) -> i64{
    let split = input.chars();
    let mut count = 0;
    let vowels = ['A', 'a', 'E', 'e', 'I', 'i', 'O', 'o', 'U', 'u'];
    for character in split{
        count += if vowels.contains(&character){1}else{0}
    }
    count
}

fn main(){
    print!("{}", count_vowels("afafsfsad"))
}

#[test]
fn test_vowels_count1() {
    assert_eq!(count_vowels(""), 0);
    assert_eq!(count_vowels("abEcd"), 2);
    assert_eq!(count_vowels("ab12Exey5 7x8U3y5z"), 4);
    assert_eq!(count_vowels("AaEeIiOoUu"), 10);
    assert_eq!(count_vowels("19712aihbixcvlkz/'{[}]]}}}$%&#@-+=/*-+;;:<> ラストが好き"), 3);
}