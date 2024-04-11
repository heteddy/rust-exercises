pub fn search<'a>(query: &str, contents: &'a str, ignore_case: bool) -> Vec<&'a str> {
    contents
        .lines()
        .filter(|line| {
            if ignore_case {
                line.to_lowercase().contains(query)
            }else{
                line.to_lowercase().contains(query)
            }
        })
        .collect()
}
