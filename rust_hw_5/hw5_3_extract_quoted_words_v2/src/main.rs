// fn extract_quoted_words(input: &str) -> Vec<String> {
//     let split_input = input.split_whitespace();
//     let mut return_vec = Vec::new();
//     for words in split_input {
//         let first_char = words.chars().nth(0).unwrap();
//         let last_char = words.chars().nth(words.chars().count() - 1).unwrap();
//         if first_char == '*' && last_char == '*' {
//             let mut word_vec = Vec::new();
//             for i in 1..words.chars().count() - 1 {
//                 word_vec.push(words.chars().nth(i).unwrap());
//             }
//             let joined_word: String = word_vec.iter().collect();
//             return_vec.push(joined_word);
//         }
//     }
//     return_vec
// }

// fn main() {
//     let input = "C ** *C++* *Java *Python* Rust*";
//     print!("{:?}", extract_quoted_words(input));
// }

// #[test]
// fn test_extract_quoted_words() {
//     assert_eq!(extract_quoted_words_r(""), Vec::<&str>::new());
//     assert_eq!(
//         extract_quoted_words_r("C ** *C++* *Java *Python* Rust*"),
//         ["", "C++", "Python"].to_vec() // "**", "*C++*", "*Python*"
//     );
//     assert_eq!(
//         extract_quoted_words_r("afsfs *sd*  *safsdf *dfdf*"),
//         ["sd", "dfdf"].to_vec()
//     );
//     assert_eq!(
//         extract_quoted_words_r("*************** aaaaaa *aaaaaaaaaa *a*"),
//         ["*************", "a",].to_vec()
//     );
// }