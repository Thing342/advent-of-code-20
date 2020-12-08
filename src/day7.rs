use regex::Regex;
use std::fs::File;
use std::io::{BufReader, BufRead};
use std::collections::{HashMap, HashSet};

#[derive(Debug)]
struct IndexMap<T> {
    index: usize,
    map: HashMap<T, usize>
}

impl <T> IndexMap<T> where
    T: std::cmp::Eq + std::hash::Hash
{
    fn with_capacity(capacity: usize) -> IndexMap<T> {
        IndexMap {
            index: 0, map: HashMap::with_capacity(capacity)
        }
    }

    fn new() -> IndexMap<T> {
        IndexMap { index: 0, map: HashMap::new() }
    }

    fn insert(&mut self, key: T) -> usize {
        if let Some(ind) = self.map.get(&key) {
            *ind
        } else {
            let idx = self.index;
            self.map.insert(key, idx);
            self.index += 1;
            idx
        }
    }

    fn get(&self, key: &T) -> Option<&usize> {
        self.map.get(key)
    }

    fn len(&self) -> usize {
        self.map.len()
    }
}

#[derive(Debug)]
struct ColorRuleMatrix {
    color_index: IndexMap<String>,
    color_matrix: Vec<Vec<usize>>
}

impl ColorRuleMatrix {
    fn with_capacity(capacity: usize) -> ColorRuleMatrix {
        ColorRuleMatrix {
            color_index: IndexMap::with_capacity(capacity),
            color_matrix: vec![vec![0usize; capacity]; capacity]
        }
    }

    fn add_requirement(&mut self, parent_color: &str, child_color: &str, n: usize) {
        let parent_idx = self.color_index.insert(parent_color.to_string());
        let child_idx = self.color_index.insert(child_color.to_string());

        self.color_matrix[parent_idx][child_idx] = n;
    }

    fn size(&self) -> usize {
        self.color_index.len()
    }

    fn get_enclosable_colors_rec(&self, color_index: usize, parent_colors: &mut HashSet<usize>) {
        for parent_color_idx in 0..self.size() {
            let n = self.color_matrix[parent_color_idx][color_index];
            if n != 0 {
                //eprintln!("#{} requires {} of #{}", parent_color_idx, n, color_index);
                parent_colors.insert(parent_color_idx);
                self.get_enclosable_colors_rec(parent_color_idx, parent_colors);
            }
        }
    }

    fn get_enclosable_colors(&self, color_index: usize) -> HashSet<usize> {
        let mut parent_colors = HashSet::new();
        self.get_enclosable_colors_rec(color_index, &mut parent_colors);
        parent_colors
    }

    fn get_enclosed_number(&self, color_index: usize) -> usize {
        let rule = &self.color_matrix[color_index];
        let mut total = 0;

        for (child_color_idx, n) in rule.iter().enumerate() {
            if *n != 0 {
                //eprintln!("#{} requires {} of #{}", color_index, *n, child_color_idx);
                total += n + n * self.get_enclosed_number(child_color_idx);
            }
        }

        total
    }
}

pub fn main() {
    let parent_regex = Regex::new("([a-z ]*) bags* contain").unwrap();
    let member_regex = Regex::new("([0-9]+) ([a-z ]*) bags*").unwrap();

    let file = File::open("inputs/day7.txt").unwrap();
    let rdr = BufReader::new(file);
    let file_lines = rdr.lines()
        .filter_map(|f| f.ok())
        .collect::<Vec<String>>();

    let mut color_rules = ColorRuleMatrix::with_capacity(file_lines.len());

    for line in file_lines {
        let first = parent_regex.captures_iter(&line).next().unwrap();
        let parent_color = &first[1];
        //eprintln!("Parent Color: {}", parent_color);
        for cap in member_regex.captures_iter(&line) {
            let child_color = &cap[2];
            let n = (&cap[1]).parse().unwrap();
            //eprintln!("\tMember Color: {}, Amount: {}", child_color, &cap[1]);
            color_rules.add_requirement(parent_color, child_color, n);
        }

    }

    let color = "shiny gold".to_string();
    let color_idx = *color_rules.color_index.get(&color).unwrap();
    let p1 = color_rules.get_enclosable_colors(color_idx);

    println!("DAY 7, PART 1: {}", p1.len());

    let p2 = color_rules.get_enclosed_number(color_idx);

    println!("DAY 7, PART 2: {}", p2);
}