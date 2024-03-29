use regex::Regex;

#[derive(PartialEq, Debug)]
pub struct Problem {
    pub number: String,
    pub name: String,
}

impl Problem {
    pub fn new(number: &str, name: &str) -> Self {
        Problem {
            number: number.to_string(),
            name: name.to_string(),
        }
    }
}

pub fn get_problem_from(title: String) -> Problem {
    let re = Regex::new(r"^(\d{1,4})\. ((?:.+ ?)+)$").unwrap();

    match re.captures(&title) {
        Some(captures) => {
            let number = format!("{:0>4}", captures.get(1).unwrap().as_str().to_string());
            let name = captures
                .get(2)
                .unwrap()
                .as_str()
                .to_lowercase()
                .split_whitespace()
                .collect::<Vec<&str>>()
                .join("-");

            Problem { number, name }
        }
        None => panic!("Invalid LeetCode title"),
    }
}

#[cfg(test)]
mod tests {
    use crate::{get_problem_from, Problem};

    #[test]
    fn test_separates_number_and_name() {
        assert_eq!(
            get_problem_from("1234. Problem Name One".to_string()),
            Problem::new("1234", "problem-name-one")
        );
    }

    #[test]
    fn test_prepending_zeroes() {
        assert_eq!(
            get_problem_from("234. Problem Name One".to_string()),
            Problem::new("0234", "problem-name-one")
        );
        assert_eq!(
            get_problem_from("34. Problem Name One".to_string()),
            Problem::new("0034", "problem-name-one")
        );
        assert_eq!(
            get_problem_from("4. Problem Name One".to_string()),
            Problem::new("0004", "problem-name-one")
        );
    }

    #[test]
    fn test_weird_titles() {
        assert_eq!(
            get_problem_from("303. Range Sum Query - Immutable".to_string()),
            Problem::new("0303", "range-sum-query---immutable")
        );
        assert_eq!(
            get_problem_from("28. Implement strStr()".to_string()),
            Problem::new("0028", "implement-strstr()")
        );
        assert_eq!(
            get_problem_from("15. 3Sum".to_string()),
            Problem::new("0015", "3sum")
        );
    }

    #[test]
    #[should_panic(expected = "Invalid LeetCode title")]
    fn test_invalid_problem_title() {
        get_problem_from("Problem Name One".to_string());
    }
}
