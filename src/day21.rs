use std::collections::{HashMap, HashSet};

use crate::lib;

pub const INPUT_FILE: &str = "input/21.txt";

// Idea:
// 1. Walk the file and build a HashMap of candidates Allergen -> Vec<Ingredient>
// 2. Once that's done, iteratively run through the map
// When we find an Allergen -> Ingredient association, move it to the solution map and remove both from the candidates map
// 3. Once the candidates map is empty, we have all the associations

type Allergen = String;
type Ingredient = String;

fn candidates<T: Iterator<Item=String>>(lines: T) -> HashMap<Allergen, HashSet<Ingredient>> {
    lines
    // from each line extract a Vec<(Allergen, Ingredient)>
    .map(|line| {
        let mut ingredients_allergens = line.split(" (contains ");
        let ingredients = ingredients_allergens.next().unwrap().split(' ');
        let mut allergens: Vec<(Allergen, Vec<Ingredient>)> = vec![];
        if let Some(allergens_raw) = ingredients_allergens.next() {
            for mut allergen in allergens_raw.split(' ') {
                // Remove trailing , or )
                allergen = &allergen[..allergen.len() - 1];
                allergens.push((allergen.to_string(), ingredients.clone().map(|ingredient| ingredient.to_string()).collect()));
            }
        }
        allergens
    })
    .flatten()
    .fold(HashMap::new(), |mut candidates, (allergen, ingredients)| {
        if let Some(allergen_ingredients) = candidates.remove(&allergen) {
            candidates.insert(allergen, ingredients.into_iter().collect::<HashSet<_>>().intersection(&allergen_ingredients).map(|s| s.clone()).collect());
        } else {
            candidates.insert(allergen, ingredients.into_iter().collect());
        }
        candidates
    })
}

fn solution(mut candidates: HashMap<Allergen, HashSet<Ingredient>>) -> HashMap<Allergen, Ingredient> {
    let mut solution: HashMap<Allergen, Ingredient> = HashMap::new();

    while candidates.len() > 0 {
        let mut removed_allergen = String::new();
        let mut removed_ingredient = String::new();
        for (allergen, ingredients) in &candidates {
            if ingredients.len() == 1 {
                removed_ingredient = ingredients.clone().drain().next().unwrap();
                removed_allergen = allergen.clone();
                solution.insert(removed_allergen.clone(), removed_ingredient.clone());
            }
        }
        candidates = candidates
        .into_iter()
        .filter(|(allergen, _)| *allergen != removed_allergen)
        .map(|(allergen, mut ingredients)| {
            ingredients.remove(&removed_ingredient);
            (allergen, ingredients)
        })
        .collect();
    }

    solution
}

fn non_allergic<T: Iterator<Item=String> + Clone>(lines: T) -> usize {

    let allergic_ingredients: HashSet<Ingredient> = solution(candidates(lines.clone())).into_iter()
    .map(|(_, ingredient)| ingredient)
    .collect();

    lines
    .map(|line| {
        line.split(" (contains ").next().unwrap().split(' ').map(|s| s.to_string()).collect::<Vec<Ingredient>>()
    })
    .flatten()
    .filter(|ingredient| !allergic_ingredients.contains(ingredient))
    .count()

}

fn allergic(allergens_ingredients: HashMap<Allergen, Ingredient>) -> String {
    let mut allergens_ingredients_vec: Vec<(Allergen, Ingredient)> = allergens_ingredients.into_iter().collect();

    allergens_ingredients_vec.sort_unstable();

    allergens_ingredients_vec.into_iter()
    .map(|(_, ingredient)| ingredient)
    .collect::<Vec<_>>()
    .join(",")
}

pub fn run1(file: &str) -> std::io::Result<usize> {
    Ok(non_allergic(lib::read_lines(file)?.collect::<Vec<_>>().into_iter()))
}

pub fn run2(file: &str) -> std::io::Result<String> {
    let input = lib::read_lines(file)?;
    Ok(allergic(solution(candidates(input))))
}


#[cfg(test)]
mod test {
    use super::*;

    const INPUT_1: &str =
"mxmxvkd kfcds sqjhc nhms (contains dairy, fish)
trh fvjkl sbzzf mxmxvkd (contains dairy)
sqjhc fvjkl (contains soy)
sqjhc mxmxvkd sbzzf (contains fish)
";

    #[test]
    fn test_candidates() {
        let input = INPUT_1.lines().map(|s| s.to_string());
        let output: HashMap<Allergen, HashSet<Ingredient>> = vec![
            ("dairy", vec!["mxmxvkd"].into_iter().collect()),
            ("fish", vec!["mxmxvkd", "sqjhc"].into_iter().collect()),
            ("soy", vec!["sqjhc", "fvjkl"].into_iter().collect()),
        ]
        .into_iter()
        .map(|(a, i): (&str, HashSet<&str>)| (
            a.to_string(),
            i.into_iter().map(|s| s.to_string()).collect(),
        ))
        .collect();
        assert_eq!(output, candidates(input));
    }

    #[test]
    fn test_solution() {
        let input = INPUT_1.lines().map(|s| s.to_string());
        let output: HashMap<Allergen, Ingredient> = vec![
            ("dairy", "mxmxvkd"),
            ("fish", "sqjhc"),
            ("soy", "fvjkl"),
        ]
        .into_iter()
        .map(|(a, i): (&str, &str)| (a.to_string(), i.to_string()))
        .collect();
        assert_eq!(output, solution(candidates(input)));
    }

    #[test]
    fn run1() {
        let input = INPUT_1.lines().map(|s| s.to_string());
        assert_eq!(5, non_allergic(input));
    }

    #[test]
    fn run2() {
        let input = INPUT_1.lines().map(|s| s.to_string());
        assert_eq!("mxmxvkd,sqjhc,fvjkl", allergic(solution(candidates(input))));
    }

}