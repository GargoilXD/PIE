mod inference_engine;
mod rule;
mod fact;

use crate::inference_engine::*;
use crate::fact::*;
use crate::rule::*;

use std::vec;


fn main() {
    let mut inference_engine: InferenceEngine = InferenceEngine::new();
    inference_engine.add_fact(Fact::from_string("player_nearby"));
    inference_engine.add_fact(Fact::from_string("!has_ammo"));
    inference_engine.add_rule(
        Rule::from_string(
            vec!["player_nearby", "has_ammo"],
            "attack"
        )
    );
    inference_engine.add_rule(
        Rule::from_string(
            vec!["player_nearby", "!has_ammo"],
            "retreat"
        )
    );
    inference_engine.infer();
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test1() {
        let mut inference_engine: InferenceEngine = InferenceEngine::new();
        inference_engine.add_fact(Fact::from_string("player_nearby"));
        inference_engine.add_fact(Fact::from_string("has_ammo"));
        inference_engine.add_rule(
            Rule::from_string(
                vec!["player_nearby", "has_ammo"],
                "should_attack"
            )
        );
        inference_engine.infer();
        assert_eq!(inference_engine.get_facts().len(), 3);
    }
    #[test]
    fn test2() {
        let mut inference_engine: InferenceEngine = InferenceEngine::new();
        inference_engine.add_fact(Fact::from_string("player_nearby"));
        inference_engine.add_fact(Fact::from_string("!has_ammo"));
        inference_engine.add_rule(
            Rule::from_string(
                vec!["player_nearby", "!has_ammo"],
                "should_attack"
            )
        );
        inference_engine.infer();
        assert_eq!(inference_engine.get_facts().len(), 3);
    }
    #[test]
    fn test3() {
        let mut inference_engine: InferenceEngine = InferenceEngine::new();
        inference_engine.add_fact(Fact::from_string("parent(john, mary)"));
        inference_engine.add_fact(Fact::from_string("parent(mary, alice)"));
        inference_engine.add_rule(
            Rule::from_string(
                vec!["parent(x?, y?)", "parent(y?, z?)"],
                "grandparent(x?, z?)"
            )
        );
        inference_engine.infer();
        assert_eq!(inference_engine.get_facts().len(), 3);
    }
    #[test]
    fn test4() {
        let mut inference_engine: InferenceEngine = InferenceEngine::new();
        inference_engine.add_fact(Fact::from_string("gender(male)"));
        inference_engine.add_fact(Fact::from_string("gender(female)"));
        inference_engine.add_fact(Fact::from_string("person(kwame, male)"));
        inference_engine.add_fact(Fact::from_string("person(ama, female)"));
        inference_engine.add_fact(Fact::from_string("person(akosua, female)"));
        inference_engine.add_fact(Fact::from_string("person(kofi, male)"));
        inference_engine.add_fact(Fact::from_string("person(agyekum, male)"));
        inference_engine.add_fact(Fact::from_string("person(osei, male)"));
        inference_engine.add_fact(Fact::from_string("person(kwakwa, female)"));
        inference_engine.add_fact(Fact::from_string("person(appiah, male)"));
        inference_engine.add_fact(Fact::from_string("person(boatemaa, female)"));
        inference_engine.add_fact(Fact::from_string("mother(boatemaa, akosua)"));
        inference_engine.add_fact(Fact::from_string("mother(boatemaa, ama)"));
        inference_engine.add_fact(Fact::from_string("mother(boatemaa, kofi)"));
        inference_engine.add_fact(Fact::from_string("mother(ama, osei)"));
        inference_engine.add_rule(
            Rule::from_string(
                vec!["mother(x?, y?)", "mother(y?, z?)"],
                "grandparent(x?, z?)"
            )
        );
        inference_engine.add_rule(
            Rule::from_string(
                vec!["mother(y?, z?)", "mother(z?, x?)"],
                "grandchild(x?, y?)"
            )
        );
        inference_engine.add_rule(
                Rule::from_string(
                vec!["person(x?, female)", "mother(z?, x?)", "mother(z?, y?)"],
                "sister(x?, y?)"
            )
        );
        inference_engine.add_rule(Rule::from_string(
            vec!["person(x?, male)", "mother(z?, x?)", "mother(z?, y?)"],
            "brother(x?, y?)"
        ));
        inference_engine.infer();
        assert_eq!(inference_engine.get_facts().len(), 27);
    }
}
