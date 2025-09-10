use std::fmt;
use crate::fact::{Fact};

#[derive(Clone)]
pub struct Rule {
    pub antecedents: Vec<Fact>,
    pub consequent: Fact,
}
impl Rule {
    pub fn new(antecedents: Vec<Fact>, consequent: Fact) -> Self {
        Rule { antecedents, consequent }
    }
    pub fn equals(&self, other: &Rule) -> bool {
        self.antecedents.len() == other.antecedents.len() &&
        self.antecedents.iter().zip(other.antecedents.iter()).all(|(a, b)| a.equals(b)) &&
        self.consequent.equals(&other.consequent)
    }
    pub fn from_string(antecedent_strings: Vec<&str>, consequence_strings: &str) -> Self {
        let antecedent = antecedent_strings.into_iter().map(|s| Fact::from_string(s)).collect();
        let consequent = Fact::from_string(consequence_strings);
        Rule::new(antecedent, consequent)
    }
}
impl fmt::Display for Rule {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let antecedent_str: Vec<String> = self.antecedents.iter().map(|a| format!("{}", a)).collect();
        write!(f, "IF {} THEN {}", antecedent_str.join(" AND "), self.consequent)
    }
}