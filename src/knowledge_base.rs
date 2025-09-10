use std::fmt;

pub struct KnowledgeBase {
    axiomatic_facts: Vec<Fact>,
    axiomatic_rules: Vec<Rule>,
    derived_facts: Vec<Fact>,
    derived_rules: Vec<Rule>
}
impl KnowledgeBase {
    pub fn new() -> Self {
        KnowledgeBase {
            axiomatic_facts: Vec::new(),
            axiomatic_rules: Vec::new(),
            derived_facts: Vec::new(),
            derived_rules: Vec::new()
        }
    }
    pub fn add_axiomatic_fact(&mut self, fact: Fact) {
        if !self.axiomatic_facts.iter().any(|f| f.equals(&fact)) {
            self.axiomatic_facts.push(fact);
        }
    }
    pub fn add_axiomatic_rule(&mut self, rule: Rule) {
        if !self.axiomatic_rules.iter().any(|r| r.equals(&rule)) {
            self.axiomatic_rules.push(rule);
        }
    }
    pub fn add_derived_fact(&mut self, fact: Fact) {
        if !self.derived_facts.iter().any(|f| f.equals(&fact)) {
            self.derived_facts.push(fact);
        }
    }
    #[allow(dead_code)]
    pub fn add_derived_rule(&mut self, rule: Rule) {
        if !self.derived_rules.iter().any(|r| r.equals(&rule)) {
            self.derived_rules.push(rule);
        }
    }
    pub fn all_facts(&self) -> Vec<&Fact> {
        self.axiomatic_facts.iter().chain(self.derived_facts.iter()).collect()
    }
    pub fn all_rules(&self) -> Vec<&Rule> {
        self.axiomatic_rules.iter().chain(self.derived_rules.iter()).collect()
    }
    pub fn has_fact(&self, fact: &Fact) -> bool {
        self.all_facts().iter().any(|existing_fact| existing_fact.equals(fact))
    }
    #[allow(dead_code)]
    pub fn has_rule(&self, rule: &Rule) -> bool {
        self.all_rules().iter().any(|existing_rule| existing_rule.equals(rule))
    }
    #[allow(dead_code)]
    pub fn clear_derived(&mut self) {
        self.derived_facts.clear();
        self.derived_rules.clear();
    }
    #[allow(dead_code)]
    pub fn clear(&mut self) {
        self.axiomatic_facts.clear();
        self.axiomatic_rules.clear();
        self.derived_facts.clear();
        self.derived_rules.clear();
    }
    pub fn from_strings(facts: Vec<&str>, rules: Vec<(&Vec<&str>, &str)>) -> Self {
        let mut knowledge_base: KnowledgeBase = KnowledgeBase::new();
        for fact_str in facts {
            knowledge_base.add_axiomatic_fact(Fact::from_string(fact_str));
        }
        for (antecedents, consequent) in rules {
            knowledge_base.add_axiomatic_rule(Rule::from_string(antecedents.clone(), consequent));
        }
        knowledge_base
    }
}
impl fmt::Display for KnowledgeBase {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "Axiomatic Facts:")?;
        for fact in &self.axiomatic_facts {
            writeln!(f, "  {}", fact)?;
        }
        writeln!(f, "Axiomatic Rules:")?;
        for rule in &self.axiomatic_rules {
            writeln!(f, "  {}", rule)?;
        }
        writeln!(f, "Derived Facts:")?;
        for fact in &self.derived_facts {
            writeln!(f, "  {}", fact)?;
        }
        writeln!(f, "Derived Rules:")?;
        for rule in &self.derived_rules {
            writeln!(f, "  {}", rule)?;
        }
        Ok(())
    }
}

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