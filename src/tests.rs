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
    assert_eq!(inference_engine.query(&Fact::from_string("should_attack")).len() > 0, true);
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
#[test]
fn test5() {
    let mut inference_engine: InferenceEngine = InferenceEngine::new(
        KnowledgeBase::from_strings(
            vec![
                "linda",
                "is_student(linda)",
                "has_car(linda)"
            ],
            vec![
                (&vec!["has_car(x?)"], "has_ticket(x?)")
            ]
        )
    );
    assert_eq!(inference_engine.prove(&Fact::from_string("has_ticket(linda)")), true);
}
#[test]
fn test_negation_with_predicates() {
    let mut inference_engine: InferenceEngine = InferenceEngine::new(
        KnowledgeBase::from_strings(
            vec![
                "visible(unit_123)",
                "has_ability(unit_123, cloak)",
            ],
            vec![
                (&vec!["visible(unit?)", "!has_ability(unit?, cloak)"], "can_target(unit?)"),
                (&vec!["visible(unit?)", "has_ability(unit?, cloak)"], "cannot_target(unit?)"),
            ]
        )
    );
    let can_target: Fact = Fact::from_string("can_target(unit_123)");
    let cannot_target: Fact = Fact::from_string("cannot_target(unit_123)");

    inference_engine.infer();

    assert!(!inference_engine.knowledge_base.has_fact(&can_target));
    assert!(inference_engine.knowledge_base.has_fact(&cannot_target));
}
#[test]
fn test_negation_multiple_units() {
    let mut inference_engine: InferenceEngine = InferenceEngine::new(
        KnowledgeBase::from_strings(
            vec![
                "visible(marine_1)",
                "visible(ghost_1)",
                "has_ability(ghost_1, cloak)",
            ],
            vec![
                (&vec!["visible(unit?)", "!has_ability(unit?, cloak)"], "can_target(unit?)"),
                (&vec!["visible(unit?)", "has_ability(unit?, cloak)"], "cannot_target(unit?)"),
            ]
        )
    );
    let marine_targetable: Fact = Fact::from_string("can_target(marine_1)");
    let ghost_targetable: Fact = Fact::from_string("can_target(ghost_1)");

    inference_engine.infer();

    assert!(inference_engine.knowledge_base.has_fact(&marine_targetable));
    assert!(!inference_engine.knowledge_base.has_fact(&ghost_targetable));
}
#[test]
fn test_nested_negation() {
    let mut inference_engine: InferenceEngine = InferenceEngine::new(
        KnowledgeBase::from_strings(
            vec![
                "visible(zergling_1)",
            ],
            vec![
                (&vec!["visible(unit?)", "detected(unit?)"], "can_attack(unit?)"),
            ]
        )
    );
    // Since we don't have any detected facts, the double negation should evaluate to false
    let zergling_attackable: Fact = Fact::from_string("can_attack(zergling_1)");

    inference_engine.infer();

    // Should not be attackable because the double negation fails
    assert!(!inference_engine.knowledge_base.has_fact(&zergling_attackable));
}