pub mod puzzle1;
pub mod puzzle2;

fn split_contents(contents: String) -> Vec<Vec<i64>> {
    let mut lines: Vec<Vec<i64>> = Vec::new();

    for line in contents.lines() {
        let values: Vec<i64> = line
            .split(" ")
            .map(|v| v.parse::<i64>().expect("all values should be ints"))
            .collect();

        lines.push(values);
    }

    lines
}
