use std::vec;

mod inference_engine;
mod knowledge_base;

use crate::inference_engine::InferenceEngine;
use crate::knowledge_base::KnowledgeBase;

fn main() {
    let mut inference_engine: InferenceEngine = InferenceEngine::new(
        KnowledgeBase::from_strings(
            vec![
                "intelligence_report(scout_1, sighted(enemy_tanks, moving_toward(northern_base))",
            ],
            vec![
                (&vec!["intelligence_report(source?, sighted(unit_type?, moving_toward(location?))"], "alert_defenses(location?)"),
            ]
        )
    );
    let fact: knowledge_base::Fact = knowledge_base::Fact::from_string("alert_defenses(northern_base)");
    if true { inference_engine.infer(); } else { inference_engine.prove(&fact); }
    println!("query: {}\n{}", fact, inference_engine.query(&fact));
}

#[cfg(test)]
mod tests;
