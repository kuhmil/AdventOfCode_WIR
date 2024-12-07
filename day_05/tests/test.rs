// use day_05::parse_lines;


// mod tests {
//     #[test]
//     fn test_correct_updates() {
//         let input = r#"
//         47|53
//         97|13
//         97|61
//         97|47
//         75|29
//         61|13
//         75|53
//         29|13
//         97|29
//         53|29
//         61|53
//         97|53
//         61|29
//         47|13
//         75|47
//         97|75
//         47|61
//         75|61
//         47|29
//         75|13
//         53|13

//         75,47,61,53,29
//         97,61,53,29,13
//         75,29,13
//         75,97,47,61,53
//         61,13,29
//         97,13,75,29,47
//         "#;

//         let mut rules = Vec::new();
//         let mut updates: Vec<Vec<usize>> = Vec::new();

//         parse_lines(input, &mut rules, &mut updates).expect("Failed to parse lines");

//         let page_number_total = correct_updates(&updates, &rules).expect("Failed to find correct updates");
//         let incorrect_page_number_total = incorrect_updates(&updates, &rules).expect("Failed to find incorrect updates");

//         assert_eq!(page_number_total, 143); // Example expected value
//         assert_eq!(incorrect_page_number_total, 123); // Example expected value
//     }
// }
