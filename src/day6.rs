use std::collections::HashSet;

fn map_group_any(group_s: &str) -> HashSet<char> {
    group_s.chars()
        .filter(|c| 'a' <= *c && *c <= 'z')
        .collect()
}

fn map_group_all(group_s: &str) -> HashSet<char> {
    let mut members = group_s.split("\n")
        .map(|member_s| member_s.chars()
            .filter(|c| 'a' <= *c && *c <= 'z')
            .collect::<HashSet<char>>()
        );

    let leader = members.next().unwrap();
    let combined = members.fold(leader, |set1, set2| &set1 & &set2);

    //eprintln!("{:?}", combined);
    combined
}

fn main() {
    let input = std::fs::read_to_string("inputs/day6.txt").unwrap();

    let total: usize = input.split("\n\n")
        .map(|group_s| map_group_any(group_s).len())
        .sum();

    println!("DAY 6, PART 1: {}", total);

    let total: usize = input.split("\n\n")
        .map(|group_s| map_group_all(group_s).len())
        .sum();

    println!("DAY 6, PART 2: {}", total);
}