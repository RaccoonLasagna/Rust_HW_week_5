fn split_grades(grades: Vec<&str>) -> (Vec<&str>, Vec<&str>){
    let mut above_d = Vec::new();
    let mut d_or_f = Vec::new();

    for grade in grades {
        match grade {
            "Excellent with A+" | "A" | "B" | "C" => above_d.push(grade),
            "D" | "Failed with F" => d_or_f.push(grade),
            _ => {}
        }
    }
    let tuple = (above_d, d_or_f);
    tuple
}

fn main() {
    let grades = vec!["G", "Failed with F", "A+", "D", "C"];
    print!("{:?}", split_grades(grades));
}

#[test]
fn test_vowels_count1() {
    assert_eq!(split_grades(["B", "Failed with F", "Excellent with A+", "D", "C"].to_vec()), (["B", "Excellent with A+", "C"].to_vec(), ["Failed with F", "D"].to_vec()));
    assert_eq!(split_grades(["G", "Failed with F", "Excellent with A+", "D", "C"].to_vec()), (["Excellent with A+", "C"].to_vec(), ["Failed with F", "D"].to_vec()));
    assert_eq!(split_grades([].to_vec()), ([].to_vec(), [].to_vec()));
    assert_eq!(split_grades(["ラストが好き", "%", "A", "D", "", "{}{}", "Failed with F", "B"].to_vec()), (["A", "B"].to_vec(), ["D", "Failed with F"].to_vec()));
    assert_eq!(split_grades(["1", "123123", "A", "Failed with F"].to_vec()), (["A"].to_vec(), ["Failed with F"].to_vec()));
    assert_eq!(split_grades(["Excellent with A+", "A", "B", "C", "D", "Failed with F", "Invalid score"].to_vec()), (["Excellent with A+", "A", "B", "C"].to_vec(), ["D", "Failed with F"].to_vec()));
}
