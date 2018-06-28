#[cfg(test)]

pub fn read (filename: &str) -> Vec<Vec<String>> {
    use std::io::{BufReader};
    use std::io::prelude::*;
    use std::fs::File;

    let file = File::open(filename)
        .expect("There was a problem opening the file");

    let buffer = BufReader::new(file);

    let content_by_line: Vec<String> = buffer
        .lines()
        .filter_map(Result::ok)
        .collect();

    let mut csv_vector = vec![vec![]; content_by_line.len()];
    let mut row = 0;

    for i in content_by_line {
        let row_content: Vec<String> = i
            .split(",")
            .map(|s| s.to_string())
            .collect();
        for j in row_content {
            csv_vector[row].push(j);
        }
        row += 1;
    }

    csv_vector
}

mod tests {
    #[allow(unused_imports)]
    use super::*;

    #[test]
    fn reads_csv() {
        let test_csv = vec![
            vec![
                String::from("number"),
                String::from("name"),
                String::from("math"),
                String::from("chemistry"),
                String::from("science"),
                String::from("physics"),
                String::from("geography")
            ]
        ];
        assert_eq!(read("test.csv"), test_csv);
    }
}
