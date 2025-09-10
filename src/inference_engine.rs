use std::collections::HashMap;

use crate::knowledge_base::*;

/// Pressupositions:
/// 1. Facts and rules are well-formed.
/// 2. No contradictory facts exist in the knowledge base.
/// 3. Rules do not create contradictions when applied.
/// 4. The knowledge base is finite and does not contain cycles that could lead to infinite loops during inference.
/// 5. Variables in facts and rules are properly scoped and do not conflict with each other
/// 6. The inference engine operates under the closed-world assumption, meaning that any fact not present in the knowledge base is considered false.
/// 7. The inference engine uses a depth-first search strategy for backward chaining and a breadth-first strategy for forward chaining.
/// 8. The inference engine does not support probabilistic reasoning or uncertainty in facts or rules

pub struct InferenceEngine {
    pub knowledge_base: KnowledgeBase,
    debug: bool
}
impl InferenceEngine {
    pub fn new(knowledge_base: KnowledgeBase) -> Self {
        InferenceEngine { knowledge_base, debug: false }
    }
    #[allow(dead_code)]
    pub fn set_debug(&mut self, debug: bool) {
        self.debug = debug;
    }
    pub fn infer(&mut self) {
        let mut changed: bool = true;
        while changed {
            changed = false;
            let mut newly_inferred: Vec<Fact> = Vec::new();
            for rule in self.knowledge_base.get_rules() {
                let valid_substitutions: Vec<HashMap<String, Fact>> = self.find_valid_substitutions(&rule.antecedents, 0, &HashMap::new());
                for substitution in valid_substitutions {
                    let new_fact: Fact = self.apply_substitution(&rule.consequent, &substitution);
                    if !self.knowledge_base.has_fact(&new_fact) && !newly_inferred.contains(&new_fact) {
                        if self.debug { println!("Inferred new fact: {}", new_fact); }
                        newly_inferred.push(new_fact);
                        changed = true;
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
                if engine.debug { println!("unify consequent: {} + {}", rule.consequent, fact); }
                if let Some(consequent_substitution) = engine.unify(&rule.consequent, fact) {
                    let mut all_antecedents_proven: bool = true;
                    for antecedent in &rule.antecedents {
                        let partially_substituted_antecedent: Fact = engine.apply_substitution(antecedent, &consequent_substitution);
                        let mut antecedent_substitution: HashMap<String, Fact> = HashMap::new();
                        for existing_fact in engine.knowledge_base.get_facts() {
                            if engine.debug { println!("unify antecedent: {} + {}", partially_substituted_antecedent, existing_fact); }
                            if let Some(unified_substitution) = engine.unify(&partially_substituted_antecedent, &existing_fact) {
                                if let Some(combined_substitution) = engine.combine_substitutions(&antecedent_substitution, unified_substitution) {
                                    antecedent_substitution = combined_substitution;
                                }
                            }
                        }
                        let substituted_antecedent: Fact = engine.apply_substitution(&partially_substituted_antecedent, &antecedent_substitution);
                        if !process(engine, &substituted_antecedent, proven_facts) {
                            all_antecedents_proven = false;
                            break;
                        }
                    }
                    if all_antecedents_proven {
                        proven_facts.push(fact.clone());
                        return true;
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
        if output.is_empty() {
            String::from("No")
        } else {
            output.join(",\n")
        }
    }
    fn find_valid_substitutions(&self, antecedents: &Vec<Fact>, index: usize, current_substitution: &HashMap<String, Fact>) -> Vec<HashMap<String, Fact>> {
        if index >= antecedents.len() { return vec![current_substitution.clone()]; }
        let antecedent: &Fact = &antecedents[index];
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
            if self.debug { println!("unify antecedent: {} + {}", antecedent, fact); }
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
                if combined.values().any(|fact| fact == &value) { return None; }
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
                let new_terms: Vec<Fact> = predicate.terms.iter().map(|term| self.apply_substitution(term, substitution)).collect();
                Fact::Predicate(PredicateFact::new(predicate.name.clone(), new_terms, predicate.positive))
            }
            _ => fact.clone(),
        }
    }
    fn unify(&self, fact1: &Fact, fact2: &Fact) -> Option<HashMap<String, Fact>> {
        match (fact1, fact2) {
            (Fact::Atomic(_), Fact::Atomic(_)) => {
                Some(HashMap::new())
            }
            (Fact::Variable(variable1), fact) => {
                //if matches!(fact, Fact::Variable(variable2) if variable1.name != variable2.name) { return None }
                Some(HashMap::from([(variable1.name.clone(), fact.clone())]))
            },
            (fact, Fact::Variable(variable1)) => {
                //if matches!(fact, Fact::Variable(variable2) if variable1.name != variable2.name) { return None; }
                Some(HashMap::from([(variable1.name.clone(), fact.clone())]))
            },
            (Fact::Predicate(predicate1), Fact::Predicate(predicate2)) => {
                if predicate1.name != predicate2.name || predicate1.terms.len() != predicate2.terms.len() || predicate1.positive != predicate2.positive {
                    return None;
                }
                let mut substitutions: HashMap<String, Fact> = HashMap::new();
                for (term1, term2) in predicate1.terms.iter().zip(predicate2.terms.iter()) {
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

