use std::vec;

mod inference_engine;
mod knowledge_base;

use crate::inference_engine::InferenceEngine;
use crate::knowledge_base::KnowledgeBase;

fn main() {
    let mut inference_engine: InferenceEngine = InferenceEngine::new(
        KnowledgeBase::from_strings(
            vec![
                "player_nearby",
                "has_ammo"
            ],
            vec![
                ("player_nearby AND has_ammo", "should_attack")
            ]
        )
    );
    inference_engine.set_debug(true);
    let fact: knowledge_base::Fact = knowledge_base::Fact::from_string("should_attack");
    if true { inference_engine.infer(); } else { inference_engine.prove(&fact); }
    println!("query: {}\n{}", fact, inference_engine.query(&fact));
}

#[cfg(test)]
mod tests;
