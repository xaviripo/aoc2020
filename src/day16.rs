use std::{collections::HashMap, ops::RangeInclusive};

use crate::lib;

pub const INPUT_FILE: &str = "input/16.txt";

type Fields = HashMap<String, (RangeInclusive<usize>, RangeInclusive<usize>)>;

type Notes = (
    Fields, // Fields
    Vec<usize>, // Your ticket
    Vec<Vec<usize>>, // Nearby tickets
);

/// Given a iterator of lines of the input file, returns:
/// - fields
/// - your ticket
/// - nearby tickets
fn contents_to_notes(mut contents: impl Iterator<Item=String>) -> Notes {

    // Fields
    let fields = contents
    .by_ref()
    .take_while(|line| line != "")
    .fold(HashMap::new(), |mut fields, line| {
        let pieces: Vec<_> = line.split(':').collect();
        let name = pieces[0];
        let mut rules = pieces[1].split(" or ").map(|rule| {
            let ends: Vec<usize> = rule.split('-').map(|end| end.trim().parse().unwrap()).collect();
            ends[0]..=ends[1]
        });
        fields.insert(name.to_string(), (rules.next().unwrap(), rules.next().unwrap()));

        fields
    });

    // Skip "your ticket:" line
    contents.by_ref().next();

    // Your tickets
    let your_ticket: Vec<usize> = contents
    .by_ref()
    .next()
    .unwrap()
    .split(',')
    .map(|number| number.parse().unwrap())
    .collect();

    // Skip empty line and "nearby tickets:" line
    contents.by_ref().next();
    contents.by_ref().next();

    // Nearby tickets
    let nearby_tickets: Vec<Vec<usize>> = contents.map(|line| {
        line
        .split(',')
        .map(|number| number.parse().unwrap())
        .collect()
    }).collect();

    (fields, your_ticket, nearby_tickets)

}

fn all_rules<'a>(fields: &'a Fields) -> Vec<&'a RangeInclusive<usize>> {
    fields
    .values()
    .fold(vec![], |mut rules, (rule0, rule1)| {
        rules.push(rule0);
        rules.push(rule1);
        rules
    })
}

fn valid_number(number: usize, rules: &Vec<&RangeInclusive<usize>>) -> bool {
    rules
    .iter()
    .any(|rule| {
        rule.contains(&number)
    }) 
}

fn error_rate((fields, _, nearby_tickets): Notes) -> usize {

    let all_rules = all_rules(&fields);

    nearby_tickets
    .into_iter()
    .flatten()
    .fold(vec![], |mut invalid, number| {
        if !valid_number(number, &all_rules) {
            invalid.push(number);
        }
        invalid
    })
    .into_iter()
    .sum()

}

pub fn run1(file: &str) -> std::io::Result<usize> {
    let input = lib::read_lines(file)?;
    Ok(error_rate(contents_to_notes(input)))
}

fn valid_ticket(ticket: &Vec<usize>, rules: &Vec<&RangeInclusive<usize>>) -> bool {
    ticket.into_iter().all(|number| valid_number(*number, rules))
}

fn transpose(source: Vec<Vec<usize>>) -> Vec<Vec<usize>> {
    (0..source[0].len())
    .map(|i| source.iter().map(|inner| inner[i]).collect::<Vec<usize>>())
    .collect()
}

fn fields_order((fields, your_ticket, nearby_tickets): Notes) -> Vec<String> {

    let all_rules = all_rules(&fields);
    let valid_tickets: Vec<Vec<usize>> = nearby_tickets.clone()
    .into_iter()
    .filter(|ticket| valid_ticket(ticket, &all_rules))
    .collect();

    // 1. Transpose the lines, so that we have columns of numbers
    // 2. For each field, see which columns it could correspond to
    // 3. Find a field that only corresponds to one column. Remove the field, and the column from the other fields, and repeat.

    // 1. Transpose the lines, so that we have columns of numbers
    let columns = transpose({
        let mut all_tickets = valid_tickets.clone();
        all_tickets.push(your_ticket);
        all_tickets
    });

    // 2. For each field, see which columns it could correspond to
    let mut fields_columns = fields
    .into_iter()
    .map(|(name, rules)| {
        let filtered_columns = columns
        .iter()
        .enumerate()
        .filter(|(_, column)| valid_ticket(*column, &vec![&rules.0, &rules.1]))
        .map(|(index, _)| index)
        .collect::<Vec<_>>();
        (name, filtered_columns)
    })
    .fold(HashMap::new(), |mut fields_columns, (name, filtered_columns)| {
        fields_columns.insert(name, filtered_columns);
        fields_columns
    });

    // 3. Find a field that only corresponds to one column. Remove the field, and the column from the other fields, and repeat.
    let mut solution_map: HashMap<String, usize> = HashMap::new();
    while fields_columns.len() > 0 {
        let (name, columns) = fields_columns.clone().into_iter().find(|(_, columns)| columns.len() == 1).unwrap();
        solution_map.insert(name.clone(), columns[0]);

        // Remove solved field
        fields_columns = fields_columns
        .into_iter()
        .filter(|(other, _)| other != &name)
        .map(|(name, other_columns)| {
            (name, other_columns.into_iter().filter(|column| columns[0] != *column).collect())
        })
        .collect();
    }

    let mut solution_vec: Vec<(usize, String)> = solution_map
    .into_iter()
    .map(|(name, column)| (column, name))
    .collect();

    solution_vec.sort_unstable();

    solution_vec
    .into_iter()
    .map(|(_, name)| name)
    .collect()

}

fn multiply_departures(your_ticket: Vec<usize>, columns: Vec<String>) -> usize {
    let indices = columns
    .into_iter()
    .enumerate()
    .filter(|(_, name)| name.contains("departure"));

    indices
    .map(|(index, _)| your_ticket[index])
    .product()
}

pub fn run2(file: &str) -> std::io::Result<usize> {
    let input = lib::read_lines(file)?;
    let (fields, your_ticket, nearby_tickets) = contents_to_notes(input);
    let columns = fields_order((fields, your_ticket.clone(), nearby_tickets));
    Ok(multiply_departures(your_ticket, columns))
}

#[cfg(test)]
mod test {
    use super::*;


    const INPUT_1: &str =
"class: 1-3 or 5-7
row: 6-11 or 33-44
seat: 13-40 or 45-50

your ticket:
7,1,14

nearby tickets:
7,3,47
40,4,50
55,2,20
38,6,12";

    #[test]
    fn run1() {
        let input = INPUT_1.lines().map(|s| s.to_string());
        assert_eq!(71, error_rate(contents_to_notes(input)));
    }

    const INPUT_2: &str =
"class: 0-1 or 4-19
row: 0-5 or 8-19
seat: 0-13 or 16-19

your ticket:
11,12,13

nearby tickets:
3,9,18
15,1,5
5,14,9";

    #[test]
    fn run2() {
        // the first position must be row, the second position must be class, and the third position must be seat
        let input = INPUT_2.lines().map(|s| s.to_string());
        let correct = vec!["row", "class", "seat"];
        let mine = fields_order(contents_to_notes(input));
        assert_eq!(correct, mine);
    }

}
