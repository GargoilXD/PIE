use std::fmt;

use crate::{fact::Fact, rule::Rule};

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
    pub fn has_rule(&self, rule: &Rule) -> bool {
        self.all_rules().iter().any(|existing_rule| existing_rule.equals(rule))
    }
    pub fn clear_derived(&mut self) {
        self.derived_facts.clear();
        self.derived_rules.clear();
    }
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