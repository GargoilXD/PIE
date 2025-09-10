use std::collections::HashMap;

use crate::knowledge_base::*;

pub struct InferenceEngine {
    pub knowledge_base: KnowledgeBase,
}
impl InferenceEngine {
    pub fn new(knowledge_base: KnowledgeBase) -> Self {
        InferenceEngine { knowledge_base }
    }
    fn unify(&self, fact1: &Fact, fact2: &Fact) -> HashMap<String, Fact> {
        match (fact1, fact2) {
            (Fact::Atomic(atomic_fact1), Fact::Atomic(atomic_fact2)) => {
                if atomic_fact1 == atomic_fact2 {
                    HashMap::from([(atomic_fact1.name.clone(), Fact::Atomic(atomic_fact1.clone()))])
                } else {
                    HashMap::new()
                }
            }
            (Fact::Variable(variable), fact) => {
                if matches!(fact, Fact::Variable(variable2) if variable.name == variable2.name) {
                    return HashMap::new();
                }
                HashMap::from([(variable.name.clone(), fact.clone())])
            },
            (fact, Fact::Variable(variable)) => {
                if matches!(fact, Fact::Variable(variable2) if variable.name == variable2.name) {
                    return HashMap::new();
                }
                HashMap::from([(variable.name.clone(), fact.clone())])
            },
            (Fact::Predicate(predicate1), Fact::Predicate(predicate2)) => {
                if predicate1.name != predicate2.name || predicate1.terms.len() != predicate2.terms.len() || predicate1.positive != predicate2.positive {
                    return HashMap::new();
                }
                let mut substitutions: HashMap<String, Fact> = HashMap::new();
                for (term1, term2) in predicate1.terms.iter().zip(predicate2.terms.iter()) {
                    let argument_substitution: HashMap<String, Fact> = self.unify(term1, term2);
                    if argument_substitution.is_empty() { return HashMap::new(); }
                    let combined_substitution: Option<HashMap<String, Fact>> = self.combine_substitutions(&substitutions, argument_substitution);
                    if let Some(combined) = combined_substitution {
                        substitutions = combined;
                    } else {
                        return HashMap::new();
                    }
                }
                substitutions
            }
            _ => HashMap::new(),
        }
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
    fn find_valid_substitutions(&self, antecedents: &Vec<Fact>, index: usize, current_substitution: &HashMap<String, Fact>) -> Vec<HashMap<String, Fact>> {
        if index >= antecedents.len() { return vec![current_substitution.clone()]; }
        let antecedent: &Fact = &antecedents[index];
        let mut valid_substitutions: Vec<HashMap<String, Fact>> = Vec::new();
        for fact in self.knowledge_base.get_facts() {
            //println!("unify: {} + {}", antecedent, fact);
            let unified_substitution: HashMap<String, Fact> = self.unify(antecedent, fact);
            if !unified_substitution.is_empty() {
                if let Some(combined_substitution) = self.combine_substitutions(&current_substitution, unified_substitution) {
                    let further_substitutions: Vec<HashMap<String, Fact>> = self.find_valid_substitutions(antecedents, index + 1, &combined_substitution);
                    valid_substitutions.extend(further_substitutions);
                }
            }
        }
        valid_substitutions
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
                        println!("Inferred new fact: {}", new_fact);
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
            for rule in engine.knowledge_base.get_rules() {
                //println!("unify consequent: {} + {}", rule.consequent, fact);
                let consequent_substitution: HashMap<String, Fact> = engine.unify(&rule.consequent, fact);
                if !consequent_substitution.is_empty() {
                    let mut all_antecedents_proven: bool = true;
                    for antecedent in &rule.antecedents {
                        let partially_substituted_antecedent: Fact = engine.apply_substitution(antecedent, &consequent_substitution);
                        let mut antecedent_substitution: HashMap<String, Fact> = HashMap::new();
                        for existing_fact in engine.knowledge_base.get_facts() {
                            //println!("unify existing: {} + {}", existing_fact, substituted_antecedent);
                            let unified_substitution: HashMap<String, Fact> = engine.unify(&existing_fact, &partially_substituted_antecedent);
                            if !unified_substitution.is_empty() {
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
                println!("Proved new fact: {}", fact);
                self.knowledge_base.add_fact(fact);
            }
            true
        } else {
            false
        }
    }
    pub fn query(&self, query: &Fact) -> Vec<HashMap<String, Fact>> {
        let mut results: Vec<HashMap<String, Fact>> = Vec::new();
        for fact in self.knowledge_base.get_facts() {
            //println!("unify query: {} + {}", query, fact);
            let substitution: HashMap<String, Fact> = self.unify(query, fact);
            if !substitution.is_empty() {
                results.push(substitution);
            }
        }
        for fact in self.knowledge_base.get_facts() {
            //println!("unify query: {} + {}", query, fact);
            let substitution: HashMap<String, Fact> = self.unify(query, fact);
            if !substitution.is_empty() {
                results.push(substitution);
            }
        }
        //println!("results for {}: {:?}", query, results);
        results
    }
}

