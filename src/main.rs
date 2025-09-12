use std::vec;

mod inference_engine;
mod knowledge_base;

use crate::inference_engine::InferenceEngine;
use crate::knowledge_base::KnowledgeBase;

fn main() {
    let mut inference_engine: InferenceEngine = InferenceEngine::new(
        KnowledgeBase::from_strings(
            vec![
                "unit_type(marine_1, infantry)",
                "unit_type(tank_1, vehicle)",
                "visible(marine_1)",
                "visible(tank_1)",
                "health(marine_1, 100)",
                "health(tank_1, 100)",
                "near(marine_1, tank_1)"
            ],
            vec![
                ("visible(unit?) & health(unit?, h?) & h? < 30", "retreat(unit?)"),
                ("visible(unit?) & unit_type(unit?, infantry) & near(unit?, enemy?) & unit_type(enemy?, vehicle)", "avoid(unit?, enemy?)"),
                ("visible(unit?) & health(unit?, h?) & h? > 80 & visible(enemy?) & unit? != enemy?", "attack(unit?, enemy?)"),
            ]
        )
    );
    inference_engine.set_debug(false);
    if false {
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
