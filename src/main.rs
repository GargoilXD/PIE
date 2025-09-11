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
    let expected: knowledge_base::Fact = knowledge_base::Fact::from_string("attack(a?, b?)");
    inference_engine.set_debug(false);
    if true {
        inference_engine.infer();
        println!("query after inference: {}\n{}", expected, inference_engine.query(&expected));
    } else {
        inference_engine.prove(&expected);
        println!("query after prove: {}\n{}", expected, inference_engine.query(&expected));
    }
}

#[cfg(test)]
mod tests;
