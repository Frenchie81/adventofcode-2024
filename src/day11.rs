use std::collections::HashMap;

pub mod puzzle1;
pub mod puzzle2;

fn parse_content(content: &str) -> Vec<String> {
    content
        .split_whitespace()
        .map(|str| str.to_string())
        .collect()
}

fn run(stones: Vec<String>, blinks: usize) -> i64 {
    let mut cache: HashMap<String, i64> = HashMap::new();
    let mut stone_count = 0;
    for stone in stones {
        stone_count += count_stones(stone, blinks, &mut cache);
    }
    stone_count
}

fn count_stones(stone: String, blinks: usize, cache: &mut HashMap<String, i64>) -> i64 {
    let cache_key = format!("{stone}:{blinks}");
    if cache.contains_key(&cache_key) {
        return *cache.get(&cache_key).unwrap();
    }

    let mut next_stone = stone.to_string();
    let mut blinks_remaining = blinks;
    while blinks_remaining > 0 {
        if next_stone == "0" {
            next_stone = "1".to_string();
            blinks_remaining -= 1;
        } else if next_stone.len() % 2 == 0 {
            let middle = next_stone.len() / 2;
            let new_stone1 = next_stone[0..middle].to_string();
            let new_stone2 = next_stone[middle..next_stone.len()].to_string();

            let new_stone1: i64 = new_stone1.parse().expect("always a number");
            let new_stone2: i64 = new_stone2.parse().expect("always a number");

            blinks_remaining -= 1;
            let mut stone_count = count_stones(new_stone1.to_string(), blinks_remaining, cache);
            stone_count += count_stones(new_stone2.to_string(), blinks_remaining, cache);
            cache.insert(cache_key, stone_count);
            return stone_count;
        } else {
            let stone_as_int: i64 = (next_stone.parse::<i64>().expect("always a number")) * 2024;
            next_stone = stone_as_int.to_string();
            blinks_remaining -= 1;
        }
    }

    cache.insert(cache_key, 1);
    1
}
