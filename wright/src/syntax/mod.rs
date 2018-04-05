//! Module containing syntax rules for wright.

extern crate regex;
use self::regex::Regex;

//todo

struct Syntax {

}

pub fn get_syntax() -> Regex {
    let letter = Regex::new("[[:alpha:]]").unwrap();
    let digit = Regex::new("[[:digit:]]").unwrap();
    let word = Regex::new("[[:word:]]").unwrap();
    let alphanumeric = or(&letter, &digit);
    let identifier = cap("ident",combine(&letter, &append(&word, "*")));
    let litint = cap("num", append(&digit,"+"));
    identifier
}

fn or(left: &Regex, right: &Regex) -> Regex {
    Regex::new(&(left.to_string() + "|" + right.as_str())).unwrap()
}

fn cap(name: &str, re: Regex) -> Regex {
    Regex::new(format!("(?P<{}>{})", name, re.as_str()).as_str()).unwrap()
}

fn combine(left: &Regex, right: &Regex) -> Regex {
    Regex::new(format!("{}{}", *left, *right).as_str()).unwrap()
}

fn append(re: &Regex, add: &str) -> Regex {
    let mut res = re.to_string();
    res.push_str(add);
    Regex::new(res.as_str()).unwrap()
}