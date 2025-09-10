use std::fmt;

#[derive(Clone, Debug)]
pub enum Fact {
    Atomic(AtomicFact),
    Predicate(PredicateFact),
    Variable(Variable),
}
impl Fact {
    #[allow(dead_code)]
    pub fn negate(&mut self) {
        match self {
            Fact::Atomic(atomic) => atomic.negate(),
            Fact::Predicate(predicate) => predicate.negate(),
            Fact::Variable(_) => {}
        }
    }
    pub fn equals(&self, other: &Fact) -> bool {
        match (self, other) {
            (Fact::Atomic(a), Fact::Atomic(b)) => a.equals(b),
            (Fact::Predicate(a), Fact::Predicate(b)) => a.equals(b),
            (Fact::Variable(a), Fact::Variable(b)) => a.equals(b),
            _ => false
        }
    }
    pub fn from_string(string: &str) -> Self {
        if string.ends_with('?') {
            Fact::Variable(Variable::from_string(string))
        } else if string.contains('(') && string.contains(')') {
            Fact::Predicate(PredicateFact::from_string(string))
        } else {
            Fact::Atomic(AtomicFact::from_string(string))
        }
    }
}
impl fmt::Display for Fact {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Fact::Atomic(a) => write!(f, "{}", a),
            Fact::Predicate(p) => write!(f, "{}", p),
            Fact::Variable(v) => write!(f, "{}", v),
        }
    }
}

#[derive(Clone, Debug)]
pub struct AtomicFact {
    pub name: String,
    pub positive: bool
}
impl AtomicFact {
    pub fn new(name: String, positive: bool) -> Self {
        AtomicFact { name, positive }
    }
    pub fn negate(&mut self) {
        self.positive = !self.positive;
    }
    pub fn equals(&self, other: &AtomicFact) -> bool {
        self.name == other.name && self.positive == other.positive
    }
    pub fn from_string(string: &str) -> Self {
        let (positive, name) = if string.starts_with('!') {
            (false, string[1..].to_string())
        } else {
            (true, string.to_string())
        };
        AtomicFact::new(name, positive)
    }
}
impl fmt::Display for AtomicFact {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}{}", if self.positive { "" } else { "!" }, self.name)
    }
}

#[derive(Clone, Debug)]
pub struct PredicateFact {
    pub name: String,
    pub terms: Vec<Fact>,
    pub positive: bool
}
impl PredicateFact {
    pub fn new(name: String, terms: Vec<Fact>, positive: bool) -> Self {
        PredicateFact { name, terms, positive }
    }
    pub fn negate(&mut self) {
        self.positive = !self.positive;
    }
    pub fn equals(&self, other: &PredicateFact) -> bool {
        self.name == other.name &&
        self.terms.len() == other.terms.len() &&
        self.positive == other.positive &&
        self.terms.iter().zip(other.terms.iter()).all(|(a, b)| a.equals(b))
    }
    pub fn from_string(string: &str) -> Self {
        let (positive, rest) = if string.starts_with('!') {
            (false, &string[1..])
        } else {
            (true, string)
        };
        let name_end = rest.find('(').unwrap_or(rest.len());
        let name = rest[..name_end].to_string();
        let terms_str = &rest[name_end+1..rest.len()-1];
        let terms: Vec<Fact> = if terms_str.trim().is_empty() {
            Vec::new()
        } else {
            terms_str.split(',').map(|s| Fact::from_string(s.trim())).collect()
        };
        PredicateFact::new(name, terms, positive)
    }
}
impl fmt::Display for PredicateFact {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let terms_str: Vec<String> = self.terms.iter().map(|t| format!("{}", t)).collect();
        write!(f, "{}{}({})", if self.positive { "" } else { "!" }, self.name, terms_str.join(", "))
    }
}

#[derive(Clone, Debug)]
pub struct Variable {
    pub name: String
}
impl Variable {
    pub fn new(name: String) -> Self {
        Variable { name }
    }
    pub fn equals(&self, other: &Variable) -> bool {
        self.name == other.name
    }
    pub fn from_string(string: &str) -> Self {
        let name = if string.ends_with('?') {
            string[..string.len()-1].to_string()
        } else {
            string.to_string()
        };
        Variable::new(name)
    }
}
impl fmt::Display for Variable {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}?", self.name)
    }
}

