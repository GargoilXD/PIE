use std::vec;

mod inference_engine;
mod knowledge_base;

use crate::inference_engine::InferenceEngine;
use crate::knowledge_base::KnowledgeBase;

fn main() {
    let mut inference_engine: InferenceEngine = InferenceEngine::new(
        KnowledgeBase::from_strings(
            vec![
                "gender(male)",
                "gender(female)",
                "person(kwame, male)",
                "person(ama, female)",
                "person(akosua, female)",
                "person(kofi, male)",
                "person(agyekum, male)",
                "person(osei, male)",
                "person(kwakwa, female)",
                "person(appiah, male)",
                "person(boatemaa, female)",
                "mother(boatemaa, akosua)",
                "mother(boatemaa, ama)",
                "mother(boatemaa, kofi)",
                "mother(ama, osei)"
            ],
            vec![
                ("mother(x?, y?) & mother(y?, z?)", "grandparent(x?, z?)"),
                ("mother(y?, z?) & mother(z?, x?)", "grandchild(x?, y?)"),
                ("person(x?, female) & mother(z?, x?) & mother(z?, y?) & x? != y?", "sister(x?, y?)"),
                ("person(x?, male) & mother(z?, x?) & mother(z?, y?) & x? != y?", "brother(x?, y?)")
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
        let fact: knowledge_base::Fact = knowledge_base::Fact::from_string("brother(kofi, osei)");
        inference_engine.prove(&fact);
        println!("Has query: {}\n{}", fact, inference_engine.query(&fact));
    }
}

#[cfg(test)]
mod tests;
