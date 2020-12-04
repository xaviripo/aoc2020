use crate::lib;

pub const INPUT_FILE: &str = "input/4.txt";

pub fn count_fields(fields: impl Iterator<Item=(String, String)>, one: bool) -> usize {
    if one {
        count_fields1(fields)
    } else {
        count_fields2(fields)
    }
}

pub fn count_fields1(fields: impl Iterator<Item=(String, String)>) -> usize {
    fields
    .filter(|(key, _)| key != "cid")
    .count()
}

pub fn count_fields2(fields: impl Iterator<Item=(String, String)>) -> usize {
    fields
    .filter(|(key, value)| match key.as_str() {
        "byr" => {
            if value.len() != 4 {
                return false;
            }
            if let Ok(number) = value.clone().parse::<u16>() {
                return 1920 <= number && number <= 2002;
            }
            false
        },
        "iyr" => {
            if value.len() != 4 {
                return false;
            }
            if let Ok(number) = value.clone().parse::<u16>() {
                return 2010 <= number && number <= 2020;
            }
            false
        },
        "eyr" => {
            if value.len() != 4 {
                return false;
            }
            if let Ok(number) = value.clone().parse::<u16>() {
                2020 <= number && number <= 2030
            } else {
                false
            }
        },
        "hgt" => {
            let len = value.len();
            if value.ends_with("cm") {
                if let Ok(number) = value[..len-2].parse::<usize>() {
                    150 <= number && number <= 193
                } else {
                    false
                }
            } else if value.ends_with("in") {
                if let Ok(number) = value[..len-2].parse::<usize>() {
                    59 <= number && number <= 76
                } else {
                    false
                }
            } else {
                false
            }
        },
        "hcl" => {
            let mut chars = value.chars();
            value.len() == 7 && chars.next().unwrap() == '#' && chars.all(|c| match c {
                '0'..='9' | 'a'..='f' => true,
                _ => false,
            })
        },
        "ecl" => match value.as_str() {
            "amb" | "blu" | "brn" | "gry" | "grn" | "hzl" | "oth" => true,
            _ => false
        },
        "pid" => value.chars().count() == 9 && value.chars().all(|c| match c {
            '0'..='9' => true,
            _ => false
        }),
        "cid" => false,
        _ => false,
    })
    .count()
}

pub fn extract_fields(line: &String) -> Vec<(String, String)> {
    line
    .split(" ")
    .filter(|&entry| entry != "")
    .map(|field| {
        let mut key_value = field.split(":").map(|piece| piece.to_string());
        let key = key_value.next();
        let value = key_value.next();
        (key.unwrap(), value.unwrap())
    })
    .collect()
}

pub fn count_passports(lines: Vec<String>, one: bool) -> usize {
    lines
        .split(|line| line == "")
        .filter(|passport_lines| {
            7 == passport_lines
            .into_iter()
            .map(|line| count_fields(extract_fields(line).into_iter(), one))
            .sum::<usize>()
        })
        .count()
}

pub fn run1(file: &str) -> std::io::Result<usize> {
    Ok(count_passports(lib::read_lines(file)?.collect(), true))
}

pub fn run2(file: &str) -> std::io::Result<usize> {
    Ok(count_passports(lib::read_lines(file)?.collect(), false))
}

#[cfg(test)]
mod test {
    use super::count_passports;


    const INPUT_1: &str =
"ecl:gry pid:860033327 eyr:2020 hcl:#fffffd
byr:1937 iyr:2017 cid:147 hgt:183cm

iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884
hcl:#cfa07d byr:1929

hcl:#ae17e1 iyr:2013
eyr:2024
ecl:brn pid:760753108 byr:1931
hgt:179cm

hcl:#cfa07d eyr:2025 pid:166559648
iyr:2011 ecl:brn hgt:59in";

    #[test]
    fn run1() {
        assert_eq!(2, count_passports(INPUT_1.to_string().lines().map(|line| line.to_string()).collect(), true));
    }

    const INPUT_2_INVALID: &str =
"eyr:1972 cid:100
hcl:#18171d ecl:amb hgt:170 pid:186cm iyr:2018 byr:1926

iyr:2019
hcl:#602927 eyr:1967 hgt:170cm
ecl:grn pid:012533040 byr:1946

hcl:dab227 iyr:2012
ecl:brn hgt:182cm pid:021572410 eyr:2020 byr:1992 cid:277

hgt:59cm ecl:zzz
eyr:2038 hcl:74454a iyr:2023
pid:3556412378 byr:2007";

    #[test]
    fn run2_invalid() {
        assert_eq!(0, count_passports(INPUT_2_INVALID.to_string().lines().map(|line| line.to_string()).collect(), false));
    }

    const INPUT_2_VALID: &str =
"pid:087499704 hgt:74in ecl:grn iyr:2012 eyr:2030 byr:1980
hcl:#623a2f

eyr:2029 ecl:blu cid:129 byr:1989
iyr:2014 pid:896056539 hcl:#a97842 hgt:165cm

hcl:#888785
hgt:164cm byr:2001 iyr:2015 cid:88
pid:545766238 ecl:hzl
eyr:2022

iyr:2010 hgt:158cm hcl:#b6652a ecl:blu byr:1944 eyr:2021 pid:093154719";

    #[test]
    fn run2_valid() {
        assert_eq!(4, count_passports(INPUT_2_VALID.to_string().lines().map(|line| line.to_string()).collect(), false));
    }


}