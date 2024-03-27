fn main() {
    let test1 = "We need more space.";
    assert_eq!(trim_spaces(test1), "We need more space.");

    let test2 = String::from("   There's space in front.");
    assert_eq!(trim_spaces(&test2), "There's space in front.");

    let test3 = String::from("There's space to the rear. ");
    assert_eq!(trim_spaces(&test3[..]), "There's space to the rear.");

    let test4 = "  We're surrounded by space!    ";
    assert_eq!(trim_spaces(test4), "We're surrounded by space!");

    let test5 = "     ";
    assert_eq!(trim_spaces(test5), "");

    let test6 = "";
    assert_eq!(trim_spaces(test6), "");

    let test7 = " 🚀 ";
    assert_eq!(trim_spaces(test7), "🚀");
    println!("Tests passed!");
}

/* YOUR CODE GOES HERE */
fn trim_spaces(s: &str) -> &str {
    let mut start_index = 0;
    for (index, letter) in s.chars().enumerate() {
        if letter != ' ' {
            start_index = index;
            break;
        }
    }

    let mut end_index = 0;
    for (index, letter) in s.chars().rev().enumerate() {
        if letter != ' ' {
            end_index = s.len() - index;
            break;
        }
    }

    println!("Start: {start_index} End: {end_index}");

    &s[start_index..end_index]
}
