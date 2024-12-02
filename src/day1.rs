pub mod puzzle1;
pub mod puzzle2;

fn split_contents(contents: String) -> (Vec<i64>, Vec<i64>) {
    let mut left: Vec<i64> = Vec::new();
    let mut right: Vec<i64> = Vec::new();
    for line in contents.lines() {
        let splits: Vec<&str> = line.split("   ").collect();
        if splits.len() != 2 {
            continue;
        }

        let value1: i64 = splits[0].parse().expect("value1 should be int");
        let value2: i64 = splits[1].parse().expect("value2 should be int");

        left.push(value1);
        right.push(value2);
    }

    (left, right)
}
