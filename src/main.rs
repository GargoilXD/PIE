use std::vec;

mod inference_engine;
mod knowledge_base;

use crate::inference_engine::InferenceEngine;
use crate::knowledge_base::KnowledgeBase;

fn main() {
    let mut inference_engine: InferenceEngine = InferenceEngine::new(
        KnowledgeBase::from_strings(
            vec![
                "role(alchemist, villager)",
                "role(baker, villager)",
                "role(bard, villager)",
                "role(bishop, villager)",
                "role(confessor, villager)",
                "role(dreamer, villager)",
                "role(druid, villager)",
                "role(empress, villager)",
                "role(enlightened, villager)",
                "role(fortune_teller, villager)",
                "role(gem_crafter, villager)",
                "role(hunter, villager)",
                "role(jester, villager)",
                "role(judge, villager)",
                "role(knight, villager)",
                "role(knitter, villager)",
                "role(lover, villager)",
                "role(medium, villager)",
                "role(oracle, villager)",
                "role(poet, villager)",
                "role(scout, villager)",
                "role(slayer, villager)",
                "role(witness, villager)",
                
                "role(drunk, outcast)",
                "role(wretch, outcast)",
                "role(bombardier, outcast)",
                "role(deppelganger, outcast)",
                "role(plague_doctor, outcast)",
                
                "role(counsellor, minion)",
                "role(witch, minion)",
                "role(minion, minion)",
                "role(poisioner, minion)",
                "role(twin_minion, minion)",
                "role(shaman, minion)",
                "role(puppeteer, minion)",
                "role(puppet, minion)",
                
                "role(baa, demon)",
                "role(pooka, demon)",
                "role(lilis, demon)",

                "good(villager)",
                "good(outcast)",
                "evil(minion)",
                "evil(demon)",

                "is_role(1, medium)",
                "is_role(2, medium)",
                "is_role(3, lover)",
                "is_role(4, jester)",
                "is_role(5, judge)",
                "is_role(6, bishop)",
                "is_role(7, oracle)",
                "is_role(8, bombardier)",

                "evil_adjacent(0, 2, 4)",

                "says(1, is_role(5, judge))",
                "says(2, is_role(2, jestor))",
                "says(3, evil_adjacent(0, 2, 4))",
                //"says(4, good(2), good(4))",
                //"says(5, good(2), good(4))",
                "says(6, role(1, minion), role(4, outcast), role(6, villager))",
                "says(7, good(2), good(4))",
                "says(8, good(2), good(4))",
                "says(3, good(2), good(4))",

            ],
            vec![
                ("says(a?, is_role(b?, c?)) & is_role(b?, c?)", "good(a?)"),
                ("says(a?, evil_adjacent(b?, c?, d?)) & [b? = 0 & ]", "good(a?)"),
                ("says(a?, good(b?), good(c?)) & good(b?) & good(c?)", "good(a?)"),
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
        let fact: knowledge_base::Fact = knowledge_base::Fact::from_string("avoid(marine_1, tank_1)");
        inference_engine.prove(&fact);
        println!("Has query: {}\n{}", fact, inference_engine.query(&fact));
    }
}

#[cfg(test)]
mod tests;
