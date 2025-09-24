use std::collections::HashMap;

use crate::knowledge_base::*;

/// The inference engine operates under the closed-world assumption, meaning that any fact not present in the knowledge base is considered false.

pub struct InferenceEngine { pub knowledge_base: KnowledgeBase, debug: bool }
impl InferenceEngine {
    pub fn new(knowledge_base: KnowledgeBase) -> Self {
        InferenceEngine { knowledge_base, debug: false }
    }
    pub fn set_debug(&mut self, debug: bool) {
        self.debug = debug;
    }
    pub fn infer(&mut self) {
        let mut changed: bool = true;
        while changed {
            changed = false;
            let mut newly_inferred: Vec<Fact> = Vec::new();
            for rule in self.knowledge_base.get_rules() {
                let mut antecedents: Vec<Fact> = Vec::new();
                for item in &rule.antecedents {
                    if let AntecedentItem::Fact(fact) = item {
                        if let Fact::Variable(_) | Fact::Number(_) = fact { continue; }
                        antecedents.push(fact.clone());
                    }
                }
                let valid_substitutions: Vec<HashMap<String, Fact>> = self.find_valid_substitutions(&antecedents, 0, &HashMap::new());
                for valid_substitution in valid_substitutions {
                    let all_antecedents_satisfied: bool = self.evaluate_antecedents(
                        &rule.antecedents,
                        &mut |antecedent: &Fact| {
                            let substituted_antecedent: Fact = self.apply_substitution(antecedent, &valid_substitution);
                            if self.knowledge_base.has_fact(&substituted_antecedent) { return true; }
                            if substituted_antecedent.is_negative() {
                                if !self.knowledge_base.has_fact(&substituted_antecedent.get_negated()) {
                                    return true;
                                }
                            }
                            return false;
                        },
                        &mut |operator: &AntecedentItem, left: &Fact, right: &Fact| {
                            let substituted_left: &Fact = &self.apply_substitution(left, &valid_substitution);
                            let substituted_right: &Fact = &self.apply_substitution(right, &valid_substitution);
                            match operator {
                                AntecedentItem::Equals => substituted_left == substituted_right,
                                AntecedentItem::NotEquals => substituted_left != substituted_right,
                                AntecedentItem::GreaterThan => {
                                    let left_number: &i32 = if let Fact::Number(NumericFact { value }) = substituted_left {
                                        value
                                    } else {
                                        panic!("Not a number")
                                    };
                                    let right_number: &i32 = if let Fact::Number(NumericFact { value }) = substituted_right {
                                        value
                                    } else {
                                        panic!("Not a number")
                                    };
                                    left_number > right_number
                                }
                                AntecedentItem::GreaterOrEquals => {
                                    let left_number: &i32 = if let Fact::Number(NumericFact { value }) = substituted_left {
                                        value
                                    } else {
                                        panic!("Not a number")
                                    };
                                    let right_number: &i32 = if let Fact::Number(NumericFact { value }) = substituted_right {
                                        value
                                    } else {
                                        panic!("Not a number")
                                    };
                                    left_number >= right_number
                                }
                                AntecedentItem::LesserThan => {
                                    let left_number: &i32 = if let Fact::Number(NumericFact { value }) = substituted_left {
                                        value
                                    } else {
                                        panic!("Not a number")
                                    };
                                    let right_number: &i32 = if let Fact::Number(NumericFact { value }) = substituted_right {
                                        value
                                    } else {
                                        panic!("Not a number")
                                    };
                                    left_number < right_number
                                }
                                AntecedentItem::LesserOrEquals => {
                                    let left_number: &i32 = if let Fact::Number(NumericFact { value }) = substituted_left {
                                        value
                                    } else {
                                        panic!("Not a number")
                                    };
                                    let right_number: &i32 = if let Fact::Number(NumericFact { value }) = substituted_right {
                                        value
                                    } else {
                                        panic!("Not a number")
                                    };
                                    left_number <= right_number
                                }
                                _ => unreachable!()
                            }
                        }
                    );
                    if all_antecedents_satisfied {
                        let new_fact: Fact = self.apply_substitution(&rule.consequent, &valid_substitution);
                        if !self.knowledge_base.has_fact(&new_fact) && !newly_inferred.contains(&new_fact) {
                            if self.debug { println!("Inferred new fact: {}", new_fact); }
                            newly_inferred.push(new_fact);
                            changed = true;
                        }
                    }
                }
            }
            for fact in newly_inferred { self.knowledge_base.add_fact(fact); }
        }
    }
    pub fn prove(&mut self, fact: &Fact) -> bool {
        fn process(engine: &InferenceEngine, fact: &Fact, proven_facts: &mut Vec<Fact>) -> bool {
            if engine.knowledge_base.has_fact(fact) { return true; }
            if fact.is_negative() {
                if !engine.knowledge_base.has_fact(&fact.get_negated()) {
                    return true;
                }
            }
            for rule in engine.knowledge_base.get_rules() {
                if engine.debug { println!("unify consequent: {} U {}", rule.consequent, fact); }
                if let Some(consequent_substitution) = engine.unify(&rule.consequent, fact) {
                    let mut antecedents: Vec<Fact> = Vec::new();
                    for item in &rule.antecedents {
                        if let AntecedentItem::Fact(fact) = item {
                            if let Fact::Variable(_) | Fact::Number(_) = fact { continue; }
                            antecedents.push(fact.clone());
                        }
                    }
                    /*let mut antecedents: Vec<Fact> = Vec::new();
                    for antecedent in unproven_antecedents {
                        if !process(engine, &antecedent, proven_facts) {
                            antecedents.push(antecedent);
                        }
                    }*/
                    //antecedents could be unproven
                    let valid_substitutions: Vec<HashMap<String, Fact>> = engine.find_valid_substitutions(&antecedents, 0, &consequent_substitution);
                    for valid_substitution in valid_substitutions {
                        let all_antecedents_proven: bool = engine.evaluate_antecedents(
                            &rule.antecedents,
                            &mut |antecedent: &Fact| {
                                process(engine, &engine.apply_substitution(antecedent, &valid_substitution), proven_facts)
                            },
                            &mut |operator: &AntecedentItem, left: &Fact, right: &Fact| {
                                let substituted_left: &Fact = &engine.apply_substitution(left, &valid_substitution);
                                let substituted_right: &Fact = &engine.apply_substitution(right, &valid_substitution);
                                match operator {
                                    AntecedentItem::Equals => substituted_left == substituted_right,
                                    AntecedentItem::NotEquals => substituted_left != substituted_right,
                                    AntecedentItem::GreaterThan => {
                                        let left_number: &i32 = if let Fact::Number(NumericFact { value }) = substituted_left {
                                            value
                                        } else {
                                            panic!("Not a number")
                                        };
                                        let right_number: &i32 = if let Fact::Number(NumericFact { value }) = substituted_right {
                                            value
                                        } else {
                                            panic!("Not a number")
                                        };
                                        left_number > right_number
                                    }
                                    AntecedentItem::GreaterOrEquals => {
                                        let left_number: &i32 = if let Fact::Number(NumericFact { value }) = substituted_left {
                                            value
                                        } else {
                                            panic!("Not a number")
                                        };
                                        let right_number: &i32 = if let Fact::Number(NumericFact { value }) = substituted_right {
                                            value
                                        } else {
                                            panic!("Not a number")
                                        };
                                        left_number >= right_number
                                    }
                                    AntecedentItem::LesserThan => {
                                        let left_number: &i32 = if let Fact::Number(NumericFact { value }) = substituted_left {
                                            value
                                        } else {
                                            panic!("Not a number")
                                        };
                                        let right_number: &i32 = if let Fact::Number(NumericFact { value }) = substituted_right {
                                            value
                                        } else {
                                            panic!("Not a number")
                                        };
                                        left_number < right_number
                                    }
                                    AntecedentItem::LesserOrEquals => {
                                        let left_number: &i32 = if let Fact::Number(NumericFact { value }) = substituted_left {
                                            value
                                        } else {
                                            panic!("Not a number")
                                        };
                                        let right_number: &i32 = if let Fact::Number(NumericFact { value }) = substituted_right {
                                            value
                                        } else {
                                            panic!("Not a number")
                                        };
                                        left_number <= right_number
                                    }
                                    _ => unreachable!()
                                }
                            }
                        );
                        if all_antecedents_proven {
                            proven_facts.push(fact.clone());
                            return true;
                        }
                    }
                }
            }
            false
        }
        let mut proven_facts: Vec<Fact> = Vec::new();
        if process(self, fact, &mut proven_facts) {
            for fact in proven_facts {
                if self.debug { println!("Proved new fact: {}", fact); }
                self.knowledge_base.add_fact(fact);
            }
            true
        } else {
            false
        }
    }
    pub fn query(&self, query: &Fact) -> String {
        let mut output: Vec<String> = Vec::new();
        for fact in self.knowledge_base.get_facts() {
            if let Some(substitution) = self.unify(query, fact) {
                output.push(self.apply_substitution(query, &substitution).to_string());
            }
        }
        if query.is_negative() {
            if output.is_empty() { String::from("Yes") } else { output.join(",\n") }
        } else {
            if output.is_empty() { String::from("No") } else { output.join(",\n") }
        }
    }
    fn evaluate_antecedents(&self, antecedents: &Vec<AntecedentItem>, fact_evaluator: &mut impl FnMut(&Fact) -> bool, operation_evaluator: &impl Fn(&AntecedentItem, &Fact, &Fact) -> bool) -> bool {
        #[derive(PartialEq, Eq)]
        enum StackItem<'s> { Fact(&'s Fact), Value(bool) }
        let mut stack: Vec<StackItem> = Vec::new();
        for item in antecedents {
            match item {
                AntecedentItem::Fact(fact) => stack.push(StackItem::Fact(fact)),
                AntecedentItem::And => {
                    let right: StackItem = stack.pop().unwrap();
                    match stack.pop().unwrap() {
                        StackItem::Fact(fact) => {
                            if !fact_evaluator(&fact) {
                                stack.push(StackItem::Value(false));
                                continue;
                            }
                        }
                        StackItem::Value(value) => {
                            if !value {
                                stack.push(StackItem::Value(false));
                                continue;
                            }
                        }
                    }
                    match right {
                        StackItem::Fact(fact) => stack.push(StackItem::Value(fact_evaluator(&fact))),
                        StackItem::Value(value) => stack.push(StackItem::Value(value))
                    }
                }
                AntecedentItem::Or => {
                    let right: StackItem = stack.pop().unwrap();
                    match stack.pop().unwrap() {
                        StackItem::Fact(fact) => {
                            if fact_evaluator(&fact) {
                                stack.push(StackItem::Value(true));
                                continue;
                            }
                        }
                        StackItem::Value(value) => {
                            if value {
                                stack.push(StackItem::Value(true));
                                continue;
                            }
                        }
                    }
                    match right {
                        StackItem::Fact(fact) => stack.push(StackItem::Value(fact_evaluator(&fact))),
                        StackItem::Value(value) => stack.push(StackItem::Value(value))
                    }
                }
                AntecedentItem::Equals | AntecedentItem::NotEquals | AntecedentItem::GreaterThan | AntecedentItem::GreaterOrEquals | AntecedentItem::LesserThan | AntecedentItem::LesserOrEquals => {
                    let right: &Fact = match stack.pop().unwrap() {
                        StackItem::Fact(fact) => fact,
                        _ => panic!("cannot compair")
                    };
                    let left: &Fact = match stack.pop().unwrap() {
                        StackItem::Fact(fact) => fact,
                        _ => panic!("cannot compair")
                    };
                    stack.push(StackItem::Value(operation_evaluator(item, left, right)));
                }
            }
        }
        match stack.pop().unwrap() {
            StackItem::Value(value) => value,
            StackItem::Fact(fact) => fact_evaluator(fact)
        }
    }
    fn find_valid_substitutions(&self, antecedents: &Vec<Fact>, index: usize, current_substitution: &HashMap<String, Fact>) -> Vec<HashMap<String, Fact>> {
        if index >= antecedents.len() { return vec![current_substitution.clone()]; }
        let antecedent: &Fact = &self.apply_substitution(&antecedents[index], current_substitution);
        if antecedent.is_negative() {
            if self.knowledge_base.has_fact(&self.apply_substitution(antecedent, current_substitution)) {
                return self.find_valid_substitutions(antecedents, index + 1, current_substitution);
            }
            let negated_antecedent: Fact = antecedent.get_negated();
            if self.knowledge_base.has_fact(&self.apply_substitution(&negated_antecedent, current_substitution)) {
                return Vec::new();
            } else {
                return self.find_valid_substitutions(antecedents, index + 1, current_substitution);
            }
        }
        let mut valid_substitutions: Vec<HashMap<String, Fact>> = Vec::new();
        for fact in self.knowledge_base.get_facts() {
            if self.debug { println!("unify antecedent: {} U {}", antecedent, fact); }
            if let Some(unified_substitution) = self.unify(antecedent, fact) {
                if let Some(combined_substitution) = self.combine_substitutions(&current_substitution, unified_substitution) {
                    let further_substitutions: Vec<HashMap<String, Fact>> = self.find_valid_substitutions(antecedents, index + 1, &combined_substitution);
                    valid_substitutions.extend(further_substitutions);
                }
            }
        }
        valid_substitutions
    }
    fn combine_substitutions(&self, substitution1: &HashMap<String, Fact>, substitution2: HashMap<String, Fact>) -> Option<HashMap<String, Fact>> {
        let mut combined: HashMap<String, Fact> = substitution1.clone();
        for (key, value) in substitution2 {
            if let Some(existing) = combined.get(&key) {
                if existing != &value { return None; }
            } else {
                combined.insert(key, value);
            }
        }
        Some(combined)
    }
    fn apply_substitution(&self, fact: &Fact, substitution: &HashMap<String, Fact>) -> Fact {
        if substitution.is_empty() { return fact.clone(); }
        match fact {
            Fact::Variable(variable) => {
                if let Some(replacement) = substitution.get(&variable.name) {
                    replacement.clone()
                } else {
                    fact.clone()
                }
            },
            Fact::Predicate(predicate) => {
                let new_arguments: Vec<Fact> = predicate.arguments.iter().map(|term| self.apply_substitution(term, substitution)).collect();
                Fact::Predicate(PredicateFact::new(predicate.positive, predicate.name.clone(), new_arguments))
            }
            _ => fact.clone(),
        }
    }
    fn unify(&self, fact1: &Fact, fact2: &Fact) -> Option<HashMap<String, Fact>> {
        match (fact1, fact2) {
            (Fact::Number(numeric_fact1), Fact::Number(numeric_fact2)) => {
                if numeric_fact1 == numeric_fact2 { Some(HashMap::new()) } else { None }
            }
            (Fact::Atomic(atomic_fact1), Fact::Atomic(atomic_fact2)) => {
                if atomic_fact1 == atomic_fact2 { Some(HashMap::new()) } else { None }
            }
            (Fact::Variable(variable1), fact) => {
                Some(HashMap::from([(variable1.name.clone(), fact.clone())]))
            }
            (fact, Fact::Variable(variable1)) => {
                Some(HashMap::from([(variable1.name.clone(), fact.clone())]))
            }
            (Fact::Predicate(predicate1), Fact::Predicate(predicate2)) => {
                if predicate1.name != predicate2.name || predicate1.arguments.len() != predicate2.arguments.len() || predicate1.positive != predicate2.positive {
                    return None;
                }
                let mut substitutions: HashMap<String, Fact> = HashMap::new();
                for (term1, term2) in predicate1.arguments.iter().zip(predicate2.arguments.iter()) {
                    if let Some(argument_substitution) = self.unify(term1, term2) {
                        let combined_substitution: Option<HashMap<String, Fact>> = self.combine_substitutions(&substitutions, argument_substitution);
                        if let Some(combined) = combined_substitution {
                            substitutions = combined;
                        } else {
                            return None;
                        }
                    } else {
                        return None;
                    }
                }
                Some(substitutions)
            }
            _ => None,
        }
    }
}

