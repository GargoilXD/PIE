mod inference_engine;
mod knowledge_base;
mod rule;
mod fact;

use crate::inference_engine::*;
use crate::knowledge_base::KnowledgeBase;
use crate::fact::*;
use crate::rule::*;

use std::vec;


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
    //inference_engine.infer();
    let fact: Fact = Fact::from_string("grandparent(john, alice)");
    println!("{} is {}", fact, inference_engine.prove(&fact));
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test1() {
        let mut knowledge_base: KnowledgeBase = KnowledgeBase::new();
        knowledge_base.add_axiomatic_fact(Fact::from_string("player_nearby"));
        knowledge_base.add_axiomatic_fact(Fact::from_string("has_ammo"));
        knowledge_base.add_axiomatic_rule(
            Rule::from_string(
                vec!["player_nearby", "has_ammo"],
                "should_attack"
            )
        );
        let mut inference_engine: InferenceEngine = InferenceEngine::new(knowledge_base);
        inference_engine.infer();
        assert_eq!(inference_engine.knowledge_base.all_facts().len(), 3);
    }
    #[test]
    fn test2() {
        let mut knowledge_base: KnowledgeBase = KnowledgeBase::new();
        knowledge_base.add_axiomatic_fact(Fact::from_string("player_nearby"));
        knowledge_base.add_axiomatic_fact(Fact::from_string("!has_ammo"));
        knowledge_base.add_axiomatic_rule(
            Rule::from_string(
                vec!["player_nearby", "!has_ammo"],
                "should_attack"
            )
        );
        let mut inference_engine: InferenceEngine = InferenceEngine::new(knowledge_base);
        inference_engine.infer();
        assert_eq!(inference_engine.knowledge_base.all_facts().len(), 3);
    }
    #[test]
    fn test3() {
        let mut knowledge_base: KnowledgeBase = KnowledgeBase::new();
        knowledge_base.add_axiomatic_fact(Fact::from_string("parent(john, mary)"));
        knowledge_base.add_axiomatic_fact(Fact::from_string("parent(mary, alice)"));
        knowledge_base.add_axiomatic_rule(
            Rule::from_string(
                vec!["parent(x?, y?)", "parent(y?, z?)"],
                "grandparent(x?, z?)"
            )
        );
        let mut inference_engine: InferenceEngine = InferenceEngine::new(knowledge_base);
        inference_engine.infer();
        assert_eq!(inference_engine.knowledge_base.all_facts().len(), 3);
    }
    #[test]
    fn test4() {
        let mut knowledge_base: KnowledgeBase = KnowledgeBase::new();
        knowledge_base.add_axiomatic_fact(Fact::from_string("gender(male)"));
        knowledge_base.add_axiomatic_fact(Fact::from_string("gender(female)"));
        knowledge_base.add_axiomatic_fact(Fact::from_string("person(kwame, male)"));
        knowledge_base.add_axiomatic_fact(Fact::from_string("person(ama, female)"));
        knowledge_base.add_axiomatic_fact(Fact::from_string("person(akosua, female)"));
        knowledge_base.add_axiomatic_fact(Fact::from_string("person(kofi, male)"));
        knowledge_base.add_axiomatic_fact(Fact::from_string("person(agyekum, male)"));
        knowledge_base.add_axiomatic_fact(Fact::from_string("person(osei, male)"));
        knowledge_base.add_axiomatic_fact(Fact::from_string("person(kwakwa, female)"));
        knowledge_base.add_axiomatic_fact(Fact::from_string("person(appiah, male)"));
        knowledge_base.add_axiomatic_fact(Fact::from_string("person(boatemaa, female)"));
        knowledge_base.add_axiomatic_fact(Fact::from_string("mother(boatemaa, akosua)"));
        knowledge_base.add_axiomatic_fact(Fact::from_string("mother(boatemaa, ama)"));
        knowledge_base.add_axiomatic_fact(Fact::from_string("mother(boatemaa, kofi)"));
        knowledge_base.add_axiomatic_fact(Fact::from_string("mother(ama, osei)"));
        knowledge_base.add_axiomatic_rule(
            Rule::from_string(
                vec!["mother(x?, y?)", "mother(y?, z?)"],
                "grandparent(x?, z?)"
            )
        );
        knowledge_base.add_axiomatic_rule(
            Rule::from_string(
                vec!["mother(y?, z?)", "mother(z?, x?)"],
                "grandchild(x?, y?)"
            )
        );
        knowledge_base.add_axiomatic_rule(
                Rule::from_string(
                vec!["person(x?, female)", "mother(z?, x?)", "mother(z?, y?)"],
                "sister(x?, y?)"
            )
        );
        knowledge_base.add_axiomatic_rule(Rule::from_string(
            vec!["person(x?, male)", "mother(z?, x?)", "mother(z?, y?)"],
            "brother(x?, y?)"
        ));
        let mut inference_engine: InferenceEngine = InferenceEngine::new(knowledge_base);
        //inference_engine.infer();
        assert_eq!(inference_engine.prove(&Fact::from_string("sister(ama, akosua)")), true);
        //assert_eq!(inference_engine.get_facts().len(), 27);
    }
}
