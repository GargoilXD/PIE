use std::vec;

mod inference_engine;
mod knowledge_base;

use crate::inference_engine::InferenceEngine;
use crate::knowledge_base::KnowledgeBase;

fn main() {
    let mut inference_engine: InferenceEngine = InferenceEngine::new(
        KnowledgeBase::from_strings(
            vec![
                "parent(john, mary)",
                "parent(mary, alice)",
            ],
            vec![
                ("parent(x?, y?) AND parent(y?, z?)", "grandparent(x?, z?)")
            ]
        )
    );
    let expected: knowledge_base::Fact = knowledge_base::Fact::from_string("grandparent(john, alice)");
    inference_engine.set_debug(false);
    inference_engine.infer();
    println!("query after inference: {}\n{}", expected, inference_engine.query(&expected));
    inference_engine.knowledge_base.clear_working_memory();
    inference_engine.prove(&expected);
    println!("query after proof: {}\n{}", expected, inference_engine.query(&expected));
}

#[cfg(test)]
mod tests;
