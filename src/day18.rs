use crate::lib;

pub const INPUT_FILE: &str = "input/18.txt";

#[derive(PartialEq, Debug, Clone, Copy)]
enum Token {
    Number(usize),
    Sum,
    Product,
    ParenOpen(usize),
    ParenClose(usize),
}

enum Expression {
    Number(usize),
    Sum(Box<Expression>, Box<Expression>),
    Product(Box<Expression>, Box<Expression>),
}

// 1 + 2 * 3 -> (1 + 2) * 3 -> Product(Sum(1, 2), 3)
fn tokenize(line: String) -> Vec<Token> {

    let mut level = 0;

    line
    .replace('(', "]")
    .replace(')', "[")
    .replace(' ', "")
    .chars()
    .rev()
    .map(|c| match c {
        '0'..='9' => Token::Number(c.to_string().parse().unwrap()),
        '*' => Token::Product,
        '+' => Token::Sum,
        '[' => {
            level += 1;
            Token::ParenOpen(level - 1)
        },
        ']' => {
            level -= 1;
            Token::ParenClose(level)
        },
        _ => panic!()
    })
    .collect()

}

#[derive(Debug, Clone)]
enum GroupedToken {
    Number(usize),
    Sum,
    Product,
    Group(Vec<GroupedToken>),
}

fn group(tokens: Vec<Token>) -> Vec<GroupedToken> {

    let mut result = vec![];

    let mut iter = tokens.into_iter();
    while let Some(token) = iter.next() {
        result.push(match token {
            Token::Number(value) => GroupedToken::Number(value),
            Token::Sum => GroupedToken::Sum,
            Token::Product => GroupedToken::Product,
            Token::ParenOpen(level) => {
                let inner_tokens = iter.by_ref().take_while(|token| *token != Token::ParenClose(level)).collect();
                GroupedToken::Group(group(inner_tokens))
            },
            Token::ParenClose(_) => panic!(),
        });
    }

    result

}

// 1 + (2 + 3) + 4
fn parse(mut grouped_tokens: Vec<GroupedToken>) -> Expression {

    grouped_tokens.reverse();

    let mut iter = grouped_tokens.into_iter();
    let mut lhs = match iter.next().unwrap() {
        GroupedToken::Number(value) => Expression::Number(value),
        GroupedToken::Group(inner_tokens) => parse(inner_tokens),
        _ => panic!(),
    };

    while let (Some(op), Some(rhs_grouped)) = (iter.next(), iter.next()) {
        let rhs = match rhs_grouped {
            GroupedToken::Number(value) => Expression::Number(value),
            GroupedToken::Group(inner_tokens) => parse(inner_tokens),
            _ => panic!(),
        };
        lhs = match op {
            GroupedToken::Sum => Expression::Sum(Box::new(lhs), Box::new(rhs)),
            GroupedToken::Product => Expression::Product(Box::new(lhs), Box::new(rhs)),
            _ => panic!()
        };
    }

    lhs

}

fn group_sums(mut grouped_tokens: Vec<GroupedToken>) -> Vec<GroupedToken> {

    grouped_tokens.reverse();

    let mut iter = grouped_tokens.into_iter();
    let first = iter.next().unwrap();
    let mut stack = vec![match first {
        GroupedToken::Group(inner_tokens) => GroupedToken::Group(group_sums(inner_tokens)),
        _ => first,
    }];

    while let (Some(op), Some(mut rhs)) = (iter.next(), iter.next()) {
        rhs = match rhs {
            GroupedToken::Group(inner_tokens) => GroupedToken::Group(group_sums(inner_tokens)),
            _ => rhs,
        };
        match op {
            GroupedToken::Sum => {
                let lhs = stack.pop().unwrap();
                stack.push(GroupedToken::Group(vec![lhs, GroupedToken::Sum, rhs]));
            },
            GroupedToken::Product => {
                stack.push(op);
                stack.push(rhs);
            },
            _ => panic!()
        };
    }

    stack

}

fn evaluate(expr: Expression) -> usize {
    match expr {
        Expression::Number(value) => value,
        Expression::Sum(lhs, rhs) => evaluate(*lhs) + evaluate(*rhs),
        Expression::Product(lhs, rhs) => evaluate(*lhs) * evaluate(*rhs),
    }
}

fn run<T: Iterator<Item=String>>(lines: T, first: bool) -> usize {
    lines
    .map(tokenize)
    .map(group)
    .map(|grouped_tokens| {
        if first {
            grouped_tokens
        } else {
            group_sums(grouped_tokens)
        }
    })
    .map(parse)
    .map(evaluate)
    .sum()
}

pub fn run1(file: &str) -> std::io::Result<usize> {
    Ok(run(lib::read_lines(file)?, true))
}

pub fn run2(file: &str) -> std::io::Result<usize> {
    Ok(run(lib::read_lines(file)?, false))
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_tokenize() {
        use Token::*;
        let input = "1 + (2 * (3 + 4))".to_string();
        // Parser changes () to ][ and reverses order
        let output: Vec<Token> = vec![
            Number(1),
            Sum,
            ParenClose(0),
                Number(2),
                Product,
                ParenClose(1),
                    Number(3),
                    Sum,
                    Number(4),
                ParenOpen(1),
            ParenOpen(0)
        ].into_iter().rev().collect();
        assert_eq!(output, tokenize(input));
    }

    #[test]
    fn simple_test() {
        assert_eq!(13, run(vec!["1 + 2 * 3 + 4"].iter().map(|s| s.to_string()), true));
    }

    const INPUT: &str =
"1 + 2 * 3 + 4 * 5 + 6
1 + (2 * 3) + (4 * (5 + 6))
2 * 3 + (4 * 5)
5 + (8 * 3 + 9 + 3 * 4 * 3)
5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))
((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2";

    #[test]
    fn run1() {
        let input = INPUT.lines().map(|s| s.to_string());
        assert_eq!(71 + 51 + 26 + 437 + 12240 + 13632, run(input, true));
    }

    // 2 * 3 + (4 * 5) -> 46
    #[test]
    fn run2() {
        let input = INPUT.lines().map(|s| s.to_string());
        assert_eq!(231 + 51 + 46 + 1445 + 669060 + 23340, run(input, false));
    }

}
