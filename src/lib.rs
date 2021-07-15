fn solution(s: &str) -> String {
    let mut result = String::new();
    for t in s.chars() {
        if t.is_ascii_uppercase() {
            result.push(' ');
        }
        result.push(t);
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solution() {
        assert_eq!(solution("camelCasing"), "camel Casing");
        assert_eq!(solution("camelCasingTest"), "camel Casing Test");
    }
}
