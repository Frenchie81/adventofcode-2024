pub mod puzzle1;
pub mod puzzle2;

fn parse_content(content: &str) -> Vec<i64> {
    let mut parsed = Vec::new();
    let mut is_file = true;
    let mut file_id: i64 = 0;
    for c in content.chars() {
        if c == '\n' {
            break;
        }
        let c = c.to_digit(10).expect("should always be a base 10 digit");
        for _ in 0..c {
            if is_file {
                parsed.push(file_id);
            } else {
                parsed.push(-1);
            }
        }

        if is_file {
            file_id += 1;
        }
        is_file = !is_file;
    }

    parsed
}

fn move_files(files: &mut [i64]) {
    let mut left_index = 0;

    let mut i = files.len();
    loop {
        i -= 1;
        let val = files[i];

        if val == -1 {
            continue;
        }

        let start_index = left_index;
        for j in start_index..i {
            if files[j] == -1 {
                files[j] = val;
                files[i] = -1;
                break;
            }
            left_index = j;
        }
        if i == 0 {
            break;
        }
    }
}

fn get_checksum(files: &[i64]) -> i64 {
    let mut checksum = 0;
    for (i, v) in files.iter().enumerate() {
        if *v == -1 {
            break;
        }

        checksum += i as i64 * v;
    }
    checksum
}
