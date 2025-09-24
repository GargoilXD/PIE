use super::*;
#[test]
fn atomic_fact_test() {
    let mut inference_engine: InferenceEngine = InferenceEngine::new(
        KnowledgeBase::from_strings(
            vec![
                "player_nearby",
                "has_ammo"
            ],
            vec![
                ("player_nearby & has_ammo", "should_attack")
            ]
        ).expect("Impossible")
    );
    inference_engine.infer();
    assert!(inference_engine.knowledge_base.has_fact(&Fact::parse("should_attack").expect("Impossible")));
}
#[test]
fn atomic_fact_negation_test() {
    let mut inference_engine: InferenceEngine = InferenceEngine::new(
        KnowledgeBase::from_strings(
            vec![
                "player_nearby",
                "!has_ammo"
            ],
            vec![
                ("player_nearby & !has_ammo", "should_attack")
            ]
        ).expect("Impossible")
    );
    inference_engine.infer();
    assert!(inference_engine.query(&Fact::parse("should_attack").expect("Impossible")).len() > 0);
}
#[test]
fn predicate_fact_test() {
    let mut inference_engine: InferenceEngine = InferenceEngine::new(
        KnowledgeBase::from_strings(
            vec![
                "parent(john, mary)",
                "parent(mary, alice)"
            ],
            vec![
                ("parent(x?, y?) & parent(y?, z?)", "grandparent(x?, z?)")
            ]
        ).expect("Impossible")
    );
    inference_engine.infer();
    assert!(inference_engine.knowledge_base.has_fact(&Fact::parse("grandparent(john, alice)").expect("Impossible")));
}
#[test]
fn complex_predicate_fact_test() {
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
        ).expect("Impossible")
    );
    inference_engine.infer();
    assert!(inference_engine.query(&Fact::parse("sister(x?, y?)").expect("Impossible")).len() > 0);
}
#[test]
fn qualifiers_test() {
    let mut inference_engine: InferenceEngine = InferenceEngine::new(
        KnowledgeBase::from_strings(
            vec![
                "is_student(linda)",
                "has_car(linda)"
            ],
            vec![
                ("has_car(x?)", "has_ticket(x?)")
            ]
        ).expect("Impossible")
    );
    assert!(inference_engine.prove(&Fact::parse("has_ticket(linda)").expect("Impossible")));
}
#[test]
fn negation_with_predicates_test() {
    let mut inference_engine: InferenceEngine = InferenceEngine::new(
        KnowledgeBase::from_strings(
            vec![
                "visible(unit_123)",
                "has_ability(unit_123, cloak)",
            ],
            vec![
                ("visible(unit?) & !has_ability(unit?, cloak)", "can_target(unit?)"),
                ("visible(unit?) & has_ability(unit?, cloak)", "cannot_target(unit?)"),
            ]
        ).expect("Impossible")
    );
    let can_target: Fact = Fact::parse("can_target(unit_123)").expect("Impossible");
    let cannot_target: Fact = Fact::parse("cannot_target(unit_123)").expect("Impossible");
    inference_engine.infer();
    assert!(!inference_engine.knowledge_base.has_fact(&can_target));
    assert!(inference_engine.knowledge_base.has_fact(&cannot_target));
}
#[test]
fn nested_negation_test() {
    let mut inference_engine: InferenceEngine = InferenceEngine::new(
        KnowledgeBase::from_strings(
            vec![
                "visible(zergling_1)",
            ],
            vec![
                ("visible(unit?) & detected(unit?)", "can_attack(unit?)"),
            ]
        ).expect("Impossible")
    );
    let zergling_attackable: Fact = Fact::parse("can_attack(zergling_1)").expect("Impossible");
    inference_engine.infer();
    assert!(!inference_engine.knowledge_base.has_fact(&zergling_attackable));
}