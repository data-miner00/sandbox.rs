pub fn search<'a>(query: &str, contents: &'a str, ignore_case: bool) -> Vec<&'a str> {
    let mut results: Vec<&str> = vec![];

    for line in contents.lines() {
        if ignore_case && line.to_lowercase().contains(&query.to_lowercase()) {
            results.push(line)
        } else if line.contains(query) {
            results.push(line);
        }
    }

    results
}

pub fn search_iter<'a>(query: &str, contents: &'a str, ignore_case: bool) -> Vec<&'a str> {
    contents
        .lines()
        .filter(|line| {
            if ignore_case {
                line.to_lowercase().contains(&query.to_lowercase())
            } else {
                line.contains(query)
            }
        })
        .collect()
}

#[cfg(test)]
#[allow(non_snake_case)]
mod tests {
    use std::vec;

    use super::*;

    #[test]
    fn givenMatch_caseSensitive_returnMatch() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";

        assert_eq!(
            vec!["safe, fast, productive."],
            search(query, contents, false)
        );
    }

    #[test]
    fn givenMatches_caseInsensitive_returnMatches() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        assert_eq!(vec!["Rust:", "Trust me."], search(query, contents, true));
    }
}
