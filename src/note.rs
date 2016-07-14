use regex::Regex;

pub fn from_str(s: &str) -> isize {
    let re = Regex::new(r#"([A-Ga-g])([#b]*)(-?\d+)"#).unwrap();
    s.to_string().len() as isize
}
