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
                //"!has_ammo"
            ],
            vec![
                (&vec!["player_nearby", "has_ammo"], "attack"),
                (&vec!["player_nearby", "!has_ammo"], "retreat"),
                (&vec!["!player_nearby"], "patrol"),
            ]
        )
    );
    let fact: knowledge_base::Fact = knowledge_base::Fact::from_string("retreat");
    if false {
        inference_engine.infer();
        println!("has_fact: {}? {}", fact, inference_engine.query(&fact).len() > 0);
    } else {
        println!("{} is {}", fact, inference_engine.prove(&fact));
    }
}

#[cfg(test)]
mod tests;
