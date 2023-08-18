fn grade(grade: f64) -> &'static str{
    if grade > 100.0 {
        return "Invalid score";
    } else if grade >= 95.0 {
        return "Excellent with A+";
    } else if grade >= 81.0 {
        return "A";
    } else if grade >= 71.0 {
        return "B";
    } else if grade >= 61.0 {
        return "C";
    } else if grade >= 50.0 {
        return "D";
    } else if grade >= 0.0 {
        return "Failed with F";
    } else {
        return "Invalid score";
    }

}

fn split_scores(scores: Vec<f64>) -> (Vec<(&'static str, f64)>, Vec<(&'static str, f64)>){
    let mut above_d = Vec::new();
    let mut d_or_f = Vec::new();
    for score in scores {
        let grade = grade(score);
        let grade_tuple: (&str, f64) = (grade, score);
        match grade {
            "Excellent with A+" | "A" | "B" | "C" => above_d.push(grade_tuple),
            "D" | "Failed with F" => d_or_f.push(grade_tuple),
            _ => {}
        }
    }
    let result_tuple = (above_d, d_or_f);
    result_tuple
}

fn main() {
    let scores = [132., 99., 42., 88.];
    print!("{:?}", split_scores(scores.to_vec()))
}

#[test]
fn test_vowels_count1() {
    assert_eq!(split_scores([132., 99., 42., 88.].to_vec()), ([("Excellent with A+", 99.0), ("A", 88.0)].to_vec(), [("Failed with F", 42.0)].to_vec()));
    assert_eq!(split_scores([-100., 500., 42., 88.].to_vec()), ([("A", 88.0)].to_vec(), [("Failed with F", 42.0)].to_vec()));
    assert_eq!(split_scores([-100., 500.].to_vec()), ([].to_vec(), [].to_vec()));
    assert_eq!(split_scores([].to_vec()), ([].to_vec(), [].to_vec()));
}