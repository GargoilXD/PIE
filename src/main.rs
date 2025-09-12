use std::vec;

mod inference_engine;
mod knowledge_base;

use crate::inference_engine::InferenceEngine;
use crate::knowledge_base::KnowledgeBase;
// Up Next
// Temporal Reasoning
// Rule Learning
// Reinforcement Learning
fn main() {
    let mut inference_engine: InferenceEngine = InferenceEngine::new(
        KnowledgeBase::from_strings(
            vec![
                //"food?",
                "amount(food, 0)",
                "is_hungry",
                "amount(animals, 0)",
                "amount(fruits, 0)",
                "!has_weapon",
                "amount(wood, 0)",
                "has_axe",
                "amount(water, 0)",
            ],
            vec![
                ("amount(food, 1)", "!is_hungry"),
                ("amount(animals, 1)", "amount(food, 1)"),
                ("amount(fruits, 1)", "amount(food, 1)"),
                //("amount(food, 1)", "!is_hungry"),
                ("has_weapon", "amount(animals, 1)"),
                ("amount(wood, 1)", "has_weapon"),
                ("has_axe", "amount(wood, 1)"),
                ("amount(wood, 1)", "has_axe"),
            ]
        )
    );
    inference_engine.set_debug(false);
    if true {
        inference_engine.infer();
        println!("New facts:");
        for fact in inference_engine.knowledge_base.working_memory {
            println!("  {}", fact);
        }
    } else {
        let fact: knowledge_base::Fact = knowledge_base::Fact::from_string("avoid(marine_1, tank_1)");
        inference_engine.prove(&fact);
        println!("Has query: {}\n{}", fact, inference_engine.query(&fact));
    }
}

#[cfg(test)]
mod tests;
