use std::{collections::HashSet, fmt};

pub struct KnowledgeBase {
    axiomatic_facts: HashSet<Fact>,
    pub working_memory: HashSet<Fact>,
    axiomatic_rules: Vec<Rule>,
    //derived_rules: Vec<Rule>
}
impl KnowledgeBase {
    pub fn new() -> Self {
        KnowledgeBase {
            axiomatic_facts: HashSet::new(),
            working_memory: HashSet::new(),
            axiomatic_rules: Vec::new(),
            //derived_rules: Vec::new()
        }
    }
    pub fn add_axiomatic_fact(&mut self, fact: Fact) {
        self.axiomatic_facts.insert(fact);
    }
    pub fn add_axiomatic_rule(&mut self, rule: Rule) {
        self.axiomatic_rules.push(rule);
    }
    pub fn add_fact(&mut self, fact: Fact) {
        self.working_memory.insert(fact);
    }
    #[allow(dead_code)]
    pub fn remove_fact(&mut self, fact: &Fact) {
        self.working_memory.remove(fact);
    }
    /*pub fn add_derived_rule(&mut self, rule: Rule) {
        if !self.derived_rules.iter().any(|r| r.equals(&rule)) {
            self.derived_rules.push(rule);
        }
    }*/
    pub fn get_facts(&self) -> impl Iterator<Item = &Fact> {
        self.axiomatic_facts.union(&self.working_memory)
    }
    pub fn get_rules(&self) -> impl Iterator<Item = &Rule> {
        self.axiomatic_rules.iter()
    }
    pub fn has_fact(&self, fact: &Fact) -> bool {
        self.axiomatic_facts.contains(fact) || self.working_memory.contains(fact)
    }
    #[allow(dead_code)]
    pub fn has_rule(&self, rule: &Rule) -> bool {
        self.axiomatic_rules.contains(rule)
    }
    #[allow(dead_code)]
    pub fn clear_working_memory(&mut self) {
        self.working_memory.clear();
    }
    #[allow(dead_code)]
    pub fn clear(&mut self) {
        self.axiomatic_facts.clear();
        self.axiomatic_rules.clear();
        self.working_memory.clear();
    }
    pub fn from_strings(facts: Vec<&str>, rules: Vec<(&str, &str)>) -> Self {
        let mut knowledge_base: KnowledgeBase = KnowledgeBase::new();
        for fact_str in facts {
            knowledge_base.add_axiomatic_fact(Fact::from_string(fact_str));
        }
        for (antecedents, consequent) in rules {
            knowledge_base.add_axiomatic_rule(Rule::from_string(antecedents, consequent));
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
        writeln!(f, "Working Memory:")?;
        for fact in &self.working_memory {
            writeln!(f, "  {}", fact)?;
        }
        Ok(())
    }
}

#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub enum Fact {
    Number(NumericFact),
    Atomic(AtomicFact),
    Predicate(PredicateFact),
    Variable(Variable),
}
impl Fact {
    pub fn is_negative(&self) -> bool {
        match self {
            Fact::Number(_) => false,
            Fact::Atomic(atomic) => !atomic.positive,
            Fact::Predicate(predicate) => !predicate.positive,
            Fact::Variable(_) => false
        }
    }
    #[allow(dead_code)]
    pub fn negate(&mut self) {
        match self {
            Fact::Number(_) => {}
            Fact::Atomic(atomic) => atomic.negate(),
            Fact::Predicate(predicate) => predicate.negate(),
            Fact::Variable(_) => {}
        }
    }
    pub fn get_negated(&self) -> Fact {
        match self {
            Fact::Number(_) => self.clone(),
            Fact::Atomic(atomic) => Fact::Atomic(atomic.get_negated()),
            Fact::Predicate(predicate) => Fact::Predicate(predicate.get_negated()),
            Fact::Variable(_) => self.clone()
        }
    }
    pub fn from_string(string: &str) -> Self {
        if string.ends_with('?') {
            Fact::Variable(Variable::from_string(string))
        } else if string.contains('(') && string.contains(')') {
            Fact::Predicate(PredicateFact::from_string(string))
        } else if string.parse::<i32>().is_ok() {
            Fact::Number(NumericFact::from_string(string))
        } else { // floats are atoms
            Fact::Atomic(AtomicFact::from_string(string))
        }
    }
}
impl fmt::Display for Fact {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Fact::Number(n) => write!(f, "{}", n),
            Fact::Atomic(a) => write!(f, "{}", a),
            Fact::Predicate(p) => write!(f, "{}", p),
            Fact::Variable(v) => write!(f, "{}", v),
        }
    }
}

#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct NumericFact {
    pub value: i32
}
impl NumericFact {
    pub fn new(value: i32) -> Self {
        NumericFact { value }
    }
    pub fn from_string(string: &str) -> Self {
        NumericFact::new(string.parse().unwrap())
    }
}
impl fmt::Display for NumericFact {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.value)
    }
}

#[derive(Clone, Debug, Hash, PartialEq, Eq)]
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
    pub fn get_negated(&self) -> Self {
        AtomicFact::new(self.name.clone(), !self.positive)
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

#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct PredicateFact {
    pub name: String,
    pub arguments: Vec<Fact>,
    pub positive: bool
}
impl PredicateFact {
    pub fn new(name: String, arguments: Vec<Fact>, positive: bool) -> Self {
        PredicateFact { name, arguments, positive }
    }
    pub fn negate(&mut self) {
        self.positive = !self.positive;
    }
    pub fn get_negated(&self) -> Self {
        PredicateFact::new(self.name.clone(), self.arguments.clone(), !self.positive)
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
        let arguments: Vec<String> = self.arguments.iter().map(|t| format!("{}", t)).collect();
        write!(f, "{}{}({})", if self.positive { "" } else { "!" }, self.name, arguments.join(", "))
    }
}

#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct Variable {
    pub name: String
}
impl Variable {
    pub fn new(name: String) -> Self {
        Variable { name }
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

#[derive(Clone, PartialEq, Eq)]
pub struct Rule {
    pub antecedents: Vec<AntecedentItem>, // Postfix Stack-Based Evaluation
    pub consequent: Fact,
}
impl Rule {
    pub fn new(antecedents: Vec<AntecedentItem>, consequent: Fact) -> Self {
        Rule { antecedents, consequent }
    }
    pub fn from_string(antecedents: &str, consequent: &str) -> Self {
        let antecedents_items: Vec<AntecedentItem> = Self::parse_antecedents(antecedents);
        let consequent_fact: Fact = Fact::from_string(consequent);
        Rule::new(antecedents_items, consequent_fact).validate()
    }
    pub fn validate(self) -> Self {
        let mut stack_height:u32 = 0;
        for item in &self.antecedents {
            match item {
                AntecedentItem::Fact(_) => stack_height += 1,
                AntecedentItem::And | AntecedentItem::Or | AntecedentItem::Equals | AntecedentItem::NotEquals | AntecedentItem::GreaterThan | AntecedentItem::GreaterOrEquals | AntecedentItem::LesserThan | AntecedentItem::LesserOrEquals => {
                    if stack_height < 2 {
                        panic!("Invalid postfix expression: not enough operands");
                    }
                    stack_height -= 1;
                }
            }
        }
        if stack_height != 1 {
            panic!("Invalid postfix expression: stack should have exactly one item at end");
        }
        self
    }
    fn parse_antecedents(input: &str) -> Vec<AntecedentItem> {
        if input.trim().is_empty() { return Vec::new(); }
        fn tokenize(input: &str) -> Vec<String> {
            let mut tokens: Vec<String> = Vec::new();
            let mut current_token: String = String::new();
            let mut comma_space = false;
            for ch in input.chars() {
                match ch {
                    ' ' | '\t' | '\n' => {
                        if !comma_space {
                            if !current_token.is_empty() {
                                tokens.push(current_token.clone());
                                current_token.clear();
                            }
                        }
                        comma_space = false;
                    }
                    '[' | ']' => {
                        if !current_token.is_empty() {
                            tokens.push(current_token.clone());
                            current_token.clear();
                        }
                        tokens.push(ch.to_string());
                        comma_space = false;
                    }
                    ',' => {
                        current_token.push(ch);
                        comma_space = true
                    }
                    _ => {
                        current_token.push(ch);
                        comma_space = false;
                    }
                }
            }
            if !current_token.is_empty() {
                tokens.push(current_token);
            }
            tokens
        }
        fn infix_to_postfix(tokens: &[String]) -> Vec<AntecedentItem> {
            let mut output: Vec<AntecedentItem> = Vec::new();
            let mut operator_stack: Vec<String> = Vec::new();
            for token in tokens {
                match token.as_str() {
                    "&" => {
                        while let Some(op) = operator_stack.last() {
                            if matches!(op.as_str(), "&" | ">" | ">=" | "<" | "<=" | "==" | "!=") {
                                let popped_op: String = operator_stack.pop().unwrap();
                                output.push(token_to_antecedent_item(&popped_op));
                            } else {
                                break;
                            }
                        }
                        operator_stack.push(token.clone());
                    }
                    "|" => {
                        while let Some(op) = operator_stack.last() {
                            if matches!(op.as_str(), "|" | "&" | ">" | ">=" | "<" | "<=" | "==" | "!=") {
                                let popped_op: String = operator_stack.pop().unwrap();
                                output.push(token_to_antecedent_item(&popped_op));
                            } else {
                                break;
                            }
                        }
                        operator_stack.push(token.clone());
                    }
                    "==" => {
                        while let Some(op) = operator_stack.last() {
                            if matches!(op.as_str(), ">" | ">=" | "<" | "<=" | "==" | "!=") {
                                let popped_op: String = operator_stack.pop().unwrap();
                                output.push(token_to_antecedent_item(&popped_op));
                            } else {
                                break;
                            }
                        }
                        operator_stack.push(token.clone());
                    }
                    "!=" => {
                        while let Some(op) = operator_stack.last() {
                            if matches!(op.as_str(), ">" | ">=" | "<" | "<=" | "==" | "!=") {
                                let popped_op: String = operator_stack.pop().unwrap();
                                output.push(token_to_antecedent_item(&popped_op));
                            } else {
                                break;
                            }
                        }
                        operator_stack.push(token.clone());
                    }
                    ">" => {
                        while let Some(op) = operator_stack.last() {
                            if matches!(op.as_str(), ">" | ">=" | "<" | "<=" | "==" | "!=") {
                                let popped_op: String = operator_stack.pop().unwrap();
                                output.push(token_to_antecedent_item(&popped_op));
                            } else {
                                break;
                            }
                        }
                        operator_stack.push(token.clone());
                    }
                    ">=" => {
                        while let Some(op) = operator_stack.last() {
                            if matches!(op.as_str(), ">" | ">=" | "<" | "<=" | "==" | "!=") {
                                let popped_op: String = operator_stack.pop().unwrap();
                                output.push(token_to_antecedent_item(&popped_op));
                            } else {
                                break;
                            }
                        }
                        operator_stack.push(token.clone());
                    }
                    "<" => {
                        while let Some(op) = operator_stack.last() {
                            if matches!(op.as_str(), ">" | ">=" | "<" | "<=" | "==" | "!=") {
                                let popped_op: String = operator_stack.pop().unwrap();
                                output.push(token_to_antecedent_item(&popped_op));
                            } else {
                                break;
                            }
                        }
                        operator_stack.push(token.clone());
                    }
                    "<=" => {
                        while let Some(op) = operator_stack.last() {
                            if matches!(op.as_str(), ">" | ">=" | "<" | "<=" | "==" | "!=") {
                                let popped_op: String = operator_stack.pop().unwrap();
                                output.push(token_to_antecedent_item(&popped_op));
                            } else {
                                break;
                            }
                        }
                        operator_stack.push(token.clone());
                    }
                    "[" => operator_stack.push(token.clone()),
                    "]" => {
                        while let Some(op) = operator_stack.pop() {
                            if op == "[" { break; }
                            output.push(token_to_antecedent_item(&op));
                        }
                    }
                    _ => output.push(AntecedentItem::Fact(Fact::from_string(token.as_str())))
                }
            }
            while let Some(op) = operator_stack.pop() {
                output.push(token_to_antecedent_item(&op));
            }
            output
        }
        fn token_to_antecedent_item(token: &str) -> AntecedentItem {
            match token {
                "&" => AntecedentItem::And,
                "|" => AntecedentItem::Or,
                "==" => AntecedentItem::Equals,
                "!=" => AntecedentItem::NotEquals,
                ">" => AntecedentItem::GreaterThan,
                ">=" => AntecedentItem::GreaterOrEquals,
                "<" => AntecedentItem::LesserThan,
                "<=" => AntecedentItem::LesserOrEquals,
                _ => AntecedentItem::Fact(Fact::from_string(token)),
            }
        }
        let tokens: Vec<String> = tokenize(input);
        infix_to_postfix(&tokens)
    }
    fn postfix_to_infix(&self) -> String {
        let mut stack: Vec<String> = Vec::new();
        for item in &self.antecedents {
            match item {
                AntecedentItem::Fact(fact) => stack.push(fact.to_string()),
                _ => {
                    if stack.len() >= 2 {
                        let right: String = stack.pop().unwrap();
                        let left: String = stack.pop().unwrap();
                        stack.push(format!("({} {} {})", left, item, right));
                    } else {
                        stack.push(item.to_string());
                    }
                }
            }
        }
        if stack.len() == 1 {
            let result: String = stack.pop().unwrap();
            if result.starts_with('(') && result.ends_with(')') {
                result
            } else {
                result
            }
        } else {
            self.antecedents.iter().map(|item: &AntecedentItem| item.to_string()).collect::<Vec<_>>().join(" ")
        }
    }
}
impl fmt::Display for Rule {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "IF {} → {}", self.postfix_to_infix(), self.consequent)
    }
}

#[derive(Clone, PartialEq, Eq)]
pub enum AntecedentItem {
    Fact(Fact),
    And, Or,
    Equals, NotEquals,
    GreaterThan, GreaterOrEquals,
    LesserThan, LesserOrEquals
}
impl fmt::Display for AntecedentItem {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AntecedentItem::Fact(fact) => write!(f, "{}", fact),
            AntecedentItem::And => write!(f, "&"),
            AntecedentItem::Or => write!(f, "|"),
            AntecedentItem::Equals => write!(f, "=="),
            AntecedentItem::NotEquals => write!(f, "!="),
            AntecedentItem::GreaterThan => write!(f, ">"),
            AntecedentItem::GreaterOrEquals => write!(f, ">="),
            AntecedentItem::LesserThan => write!(f, "<"),
            AntecedentItem::LesserOrEquals => write!(f, "<="),
        }
    }
}
