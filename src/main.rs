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
                "parent(mary, alice)"
            ],
            vec![
                (&vec!["parent(x?, y?)", "parent(y?, z?)"], "grandparent(x?, z?)")
            ]
        )
    );
    let fact: knowledge_base::Fact = knowledge_base::Fact::from_string("grandparent(john, alice)");
    if true {
        inference_engine.infer();
        println!("has_fact: {}? {}", fact, inference_engine.knowledge_base.has_fact(&fact));
    } else {
        println!("{} is {}", fact, inference_engine.prove(&fact));
    }
}

#[cfg(test)]
mod tests {
    use crate::knowledge_base::Fact;

    use super::*;
    #[test]
    fn test1() {
        let mut inference_engine: InferenceEngine = InferenceEngine::new(
            KnowledgeBase::from_strings(
                vec![
                    "player_nearby",
                    "has_ammo"
                ],
                vec![
                    (&vec!["player_nearby", "has_ammo"], "should_attack")
                ]
            )
        );
        inference_engine.infer();
        assert_eq!(inference_engine.knowledge_base.has_fact(&Fact::from_string("should_attack")), true);
    }
    #[test]
    fn test2() {
        let mut inference_engine: InferenceEngine = InferenceEngine::new(
            KnowledgeBase::from_strings(
                vec![
                    "player_nearby",
                    "!has_ammo"
                ],
                vec![
                    (&vec!["player_nearby", "!has_ammo"], "should_attack")
                ]
            )
        );
        inference_engine.infer();
        assert_eq!(inference_engine.knowledge_base.all_facts().len(), 3);
    }
    #[test]
    fn test3() {
        let mut inference_engine: InferenceEngine = InferenceEngine::new(
            KnowledgeBase::from_strings(
                vec![
                    "parent(john, mary)",
                    "parent(mary, alice)"
                ],
                vec![
                    (&vec!["parent(x?, y?)", "parent(y?, z?)"], "grandparent(x?, z?)")
                ]
            )
        );
        inference_engine.infer();
        assert_eq!(inference_engine.knowledge_base.has_fact(&Fact::from_string("grandparent(john, alice)")), true);
    }
    #[test]
    fn test4() {
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
                    (&vec!["mother(x?, y?)", "mother(y?, z?)"], "grandparent(x?, z?)"),
                    (&vec!["mother(y?, z?)", "mother(z?, x?)"], "grandchild(x?, y?)"),
                    (&vec!["person(x?, female)", "mother(z?, x?)", "mother(z?, y?)"], "sister(x?, y?)"),
                    (&vec!["person(x?, male)", "mother(z?, x?)", "mother(z?, y?)"], "brother(x?, y?)")
                ]
            )
        );
        assert_eq!(inference_engine.prove(&Fact::from_string("sister(ama, akosua)")), true);
    }
}
