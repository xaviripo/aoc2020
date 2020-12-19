use std::collections::HashMap;

use regex::Regex;

use crate::lib;

pub const INPUT_FILE: &str = "input/19.txt";

enum Rule {
    Letter(char),
    List(Vec<usize>),
    Or(Vec<usize>, Vec<usize>),
}

fn parse(line: String) -> Rule {

    if line.starts_with('"') {
        return Rule::Letter(line.chars().nth(1).unwrap())
    }

    if line.contains('|') {
        let mut split = line.split(" | ");
        let first = split.next().unwrap();
        let second = split.next().unwrap();
        return Rule::Or(
            first.split(' ').map(|s| s.parse().unwrap()).collect(),
            second.split(' ').map(|s| s.parse().unwrap()).collect(),
        );
    }

    Rule::List(line.split(' ').map(|s| s.parse().unwrap()).collect())

}

fn parse_lines<T: Iterator<Item=String>>(lines: T) -> HashMap<usize, Rule> {
    lines
    .map(|line| {
        let mut split = line.split(": ");
        let id: usize = split.next().unwrap().parse().unwrap();
        let body = split.next().unwrap().to_string();
        (id, parse(body))
    })
    .collect()
}

fn list_to_regex(list: &Vec<usize>, rules: &HashMap<usize, Rule>, regexs: &mut HashMap<usize, String>, recursive: bool) -> String {
    list.into_iter().map(|index| {
        if let Some(regex) = regexs.get(index) {
            regex.clone()
        } else if recursive {
            // 8: 42 | 42 8
            // 11: 42 31 | 42 11 31
            let regex = match *index {
                8 => format!("(?:{})+", to_regex(rules.get(&42).unwrap(), rules, regexs, recursive)),
                11 => {
                    let r42 = to_regex(rules.get(&42).unwrap(), rules, regexs, recursive);
                    let r31 = to_regex(rules.get(&31).unwrap(), rules, regexs, recursive);

                    let mut res = format!("(?:{}{})?", r42, r31);
                    // To any and all future employers and reviewers: please have mercy upon my code
                    for _ in 0..70 {
                        res = format!("(?:{}{}{})?", r42, res, r31);
                    }
                    format!("{}{}{}", r42, res, r31)
                },
                _ => to_regex(rules.get(index).unwrap(), rules, regexs, recursive),
            };
            regexs.insert(*index, regex.clone());
            regex
        } else {
            let regex = to_regex(rules.get(index).unwrap(), rules, regexs, recursive);
            regexs.insert(*index, regex.clone());
            regex
        }
    })
    .collect::<String>()
}

fn to_regex(rule: &Rule, rules: &HashMap<usize, Rule>, regexs: &mut HashMap<usize, String>, recursive: bool) -> String {
    match rule {
        Rule::Letter(letter) => letter.to_string(),
        Rule::List(list) => list_to_regex(list, rules, regexs, recursive),
        Rule::Or(first, second) => {
            let first_regex = list_to_regex(first, rules, regexs, recursive);
            let second_regex = list_to_regex(second, rules, regexs, recursive);
            format!("(?:{}|{})", first_regex, second_regex)
        }
    }
}

fn first_regex(rules: HashMap<usize, Rule>, recursive: bool) -> String {
    let first = rules.get(&0).unwrap();
    let regex = to_regex(first, &rules, &mut HashMap::new(), recursive);
    format!("^{}$", regex)
}

fn parse_file<T: Iterator<Item=String>>(mut lines: T, first: bool) -> usize {
    let rules = lines
    .by_ref()
    .take_while(|line| line.trim() != "")
    .map(|s| s.to_string());

    let regex = Regex::new(first_regex(parse_lines(rules), first).as_str()).unwrap();

    lines
    .filter(|input| regex.is_match(input.as_str()))
    .count()
}

pub fn run1(file: &str) -> std::io::Result<usize> {
    Ok(parse_file(lib::read_lines(file)?, false))
}

pub fn run2(file: &str) -> std::io::Result<usize> {
    Ok(parse_file(lib::read_lines(file)?, true))
}

#[cfg(test)]
mod test {

    use super::*;

    const INPUT_1: &str =
r#"0: 4 1 5
1: 2 3 | 3 2
2: 4 4 | 5 5
3: 4 5 | 5 4
4: "a"
5: "b"

ababbb
bababa
abbbab
aaabbb
aaaabbb
"#;

    #[test]
    fn run1() {
        let input = INPUT_1.lines().map(|s| s.to_string());
        assert_eq!(2, parse_file(input, false));
    }

    const INPUT_2: &str =
r#"42: 9 14 | 10 1
9: 14 27 | 1 26
10: 23 14 | 28 1
1: "a"
11: 42 31
5: 1 14 | 15 1
19: 14 1 | 14 14
12: 24 14 | 19 1
16: 15 1 | 14 14
31: 14 17 | 1 13
6: 14 14 | 1 14
2: 1 24 | 14 4
0: 8 11
13: 14 3 | 1 12
15: 1 | 14
17: 14 2 | 1 7
23: 25 1 | 22 14
28: 16 1
4: 1 1
20: 14 14 | 1 15
3: 5 14 | 16 1
27: 1 6 | 14 18
14: "b"
21: 14 1 | 1 14
25: 1 1 | 1 14
22: 14 14
8: 42
26: 14 22 | 1 20
18: 15 15
7: 14 5 | 1 21
24: 14 1

abbbbbabbbaaaababbaabbbbabababbbabbbbbbabaaaa
bbabbbbaabaabba
babbbbaabbbbbabbbbbbaabaaabaaa
aaabbbbbbaaaabaababaabababbabaaabbababababaaa
bbbbbbbaaaabbbbaaabbabaaa
bbbababbbbaaaaaaaabbababaaababaabab
ababaaaaaabaaab
ababaaaaabbbaba
baabbaaaabbaaaababbaababb
abbbbabbbbaaaababbbbbbaaaababb
aaaaabbaabaaaaababaa
aaaabbaaaabbaaa
aaaabbaabbaaaaaaabbbabbbaaabbaabaaa
babaaabbbaaabaababbaabababaaab
aabbbbbaabbbaaaaaabbbbbababaaaaabbaaabba
"#;

    #[test]
    fn run2() {
        let input = INPUT_2.lines().map(|s| s.to_string());
        assert_eq!(12, parse_file(input, true));
    }

}
