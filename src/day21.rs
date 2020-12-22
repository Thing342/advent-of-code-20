use std::collections::{HashSet, HashMap, BTreeSet};
use std::fs::File;
use std::io::{BufReader, BufRead};

use itertools::{iproduct, Itertools};

const FILE: &str = "inputs/day21.txt";
const CONTAINS: &str = "(contains";

#[derive(Debug, Hash, Eq, PartialEq, Clone, Ord, PartialOrd)]
struct Ingredient(String);

#[derive(Debug, Hash, Eq, PartialEq, Clone, Ord, PartialOrd)]
struct Allergen(String);

#[derive(Debug)]
struct Recipe {
    ingredients: HashSet<Ingredient>,
    listed_allergens: HashSet<Allergen>
}

impl Recipe {
    fn parse(line: String) -> Recipe {

        let mut tokens = line.split(" ");
        let mut listed_allergens = HashSet::new();
        let mut ingredients = HashSet::new();

        let mut reading_allergens = false;

        while let Some(t) = tokens.next() {
            if reading_allergens {
                let ag = t.trim_end_matches(")").trim_end_matches(",");
                listed_allergens.insert(Allergen(ag.to_string()));
            } else if t.starts_with(CONTAINS) {
                reading_allergens = true;
                continue;
            } else {
                ingredients.insert(Ingredient(t.to_string()));
            }
        }

        Recipe {
            listed_allergens,
            ingredients
        }
    }
}

pub fn main() {
    let file = File::open(FILE).expect("Failed to open file");
    let rdr = BufReader::new(file);
    let recipes = rdr.lines()
        .filter_map(|r| r.ok())
        .map(|s| Recipe::parse(s))
        .collect::<Vec<_>>();

    let all_allergens = recipes.iter()
        .fold(BTreeSet::new(), |mut bag, r| {
            bag.extend(r.listed_allergens.iter());
            bag
        });

    /*
    let all_ingredients = recipes.iter()
        .fold(HashSet::new(), |mut bag, r| {
            bag.extend(r.ingredients.iter());
            bag
        });
*/
    //eprintln!("{:?}\n{:?}", all_allergens, all_ingredients);

    let possible_mappings = all_allergens.iter()
        .map(|ag| {
            let bag = recipes.iter()
                .filter(|r| r.listed_allergens.contains(*ag))
                .fold(HashSet::new(), |mut bag, recipe| {
                    if bag.is_empty() {
                        bag.extend(recipe.ingredients.iter())
                    } else {
                        bag.retain(|ing| recipe.ingredients.contains(ing))
                    }
                    bag
                });
            (*ag, bag)
        }).collect::<HashMap<_,_>>();

    //eprintln!("{:#?}", possible_mappings);

    let mut allergen_free = HashSet::new();
    for recipe in &recipes {
        for ing in &recipe.ingredients {
            let b = possible_mappings.iter().any(|(k,v)| v.contains(ing));
            if !b {
                allergen_free.insert(ing);
            }
        }
    }

    //eprintln!("{:#?}", allergen_free);

    let count = iproduct!(recipes.iter(), allergen_free.iter())
        .filter(|(recipe, ing)| recipe.ingredients.contains(ing))
        .count();

    println!("DAY 21, PART 1: {}", count);

    let mut mapping_bags = possible_mappings.clone();
    let mut mappings = HashMap::new();

    while let Some((ag, bag)) = mapping_bags.iter().find(|(k,v)| v.len() == 1) {
        let ing = *bag.iter().next().unwrap();
        let ag = *ag;
        mappings.insert(ag, ing);
        for (ag2, bag) in &mut mapping_bags {
            bag.remove(ing);
        }
    }

    //eprintln!("{:#?}", mappings);
    //eprintln!("{:#?}", all_allergens);

    let p2 = all_allergens.iter().map(|ag| &mappings[ag].0 ).join(",");
    println!("DAY 21, PART 2: {}", p2);
}