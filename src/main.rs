use std::{env, fs};

mod inference_engine;
mod knowledge_base;

#[cfg(test)]
mod tests;

use crate::inference_engine::InferenceEngine;
use crate::knowledge_base::{Fact, KnowledgeBase};

const DEFAULT_FILE: &str = "./examples/default.txt";

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut file_path: Option<String> = None;
    let mut debug: bool = false;
    let mut query_mode: bool = false;
    let mut prove_fact: Option<String> = None;

    let mut index: usize = 1;
    while index < args.len() {
        match args[index].as_str() {
            "--file" | "-f" => {
                if index + 1 < args.len() {
                    file_path = Some(args[index + 1].clone());
                    index += 1;
                } else {
                    eprintln!("Error: --file requires a path argument");
                    return;
                }
            }
            "--debug" | "-d" => debug = true,
            "--query" | "-q" => query_mode = true,
            "--prove" | "-p" => {
                if index + 1 < args.len() {
                    prove_fact = Some(args[index + 1].clone());
                    index += 1;
                } else {
                    eprintln!("Error: --prove requires a fact argument");
                    return;
                }
            }
            "--help" | "-h" => {
                print_help();
                return;
            }
            _ => {
                if file_path.is_none() && !args[index].starts_with("--") && !args[index].starts_with("-") {
                    file_path = Some(args[index].clone());
                } else {
                    eprintln!("Unknown argument: {}", args[index]);
                    print_help();
                    return;
                }
            }
        }
        index += 1;
    }
    let knowledge_base: KnowledgeBase = if let Some(path) = file_path {
        match parse_file(&path) {
            Ok(knowledge_base) => knowledge_base,
            Err(error) => {
                eprintln!("Error reading file '{}': {}", path, error);
                return;
            }
        }
    } else {
        match parse_file(DEFAULT_FILE) {
            Ok(knowledge_base) => knowledge_base,
            Err(error) => {
                eprintln!("Error reading file '{}': {}", DEFAULT_FILE, error);
                return;
            }
        }
    };

    let mut inference_engine: InferenceEngine = InferenceEngine::new(knowledge_base);
    inference_engine.set_debug(debug);

    if query_mode {
        use std::io::{self, Write};

        println!("PIE 0.1.0");
        println!("Type 'quit' to exit.");

        inference_engine.set_debug(false);
        inference_engine.infer();

        loop {
            print!("> ");
            
            let mut input: String = String::new();
            io::stdout().flush().unwrap();
            io::stdin().read_line(&mut input).unwrap();
            
            let input: &str = input.trim();
            if input.eq_ignore_ascii_case("quit") || input.eq_ignore_ascii_case("exit") { break; }
            if input.is_empty() { continue; }
            
            match Fact::parse(input) {
                Ok(fact) => println!("   {}", inference_engine.query(&fact).replace("\n", "\n   ")),
                Err(error) => eprintln!("  Error: {}\n", error)
            }
        }
    } else if let Some(fact_str) = prove_fact {
        match Fact::parse(&fact_str) {
            Ok(fact) => println!("{} is {}", fact, inference_engine.prove(&fact)),
            Err(error) => eprintln!("Error parsing fact '{}': {}", fact_str, error)
        }
    } else {
        inference_engine.infer();
        if inference_engine.knowledge_base.working_memory.is_empty() {
            println!("No new facts")
        } else {
            println!("New facts:");
            for fact in &inference_engine.knowledge_base.working_memory {
                println!("  {}", fact);
            }
        }
    }
}

fn parse_file(file_path: &str) -> Result<KnowledgeBase, String> {
    let content: String = fs::read_to_string(file_path).map_err(|error| format!("Failed to read file: {}", error))?;

    let mut rules: Vec<(&str, &str)> = Vec::new();
    let mut facts: Vec<&str> = Vec::new();

    for (line_number, line) in content.lines().enumerate() {
        let line: &str = line.trim();
        if line.is_empty() || line.starts_with('#') { continue; }
        if line.contains("->") {
            let parts: Vec<&str> = line.split("->").collect();
            if parts.len() != 2 {
                return Err(format!("Line {}: Invalid rule syntax", line_number + 1));
            }
            let head: &str = parts[1].trim().trim_end_matches('.').trim();
            let body: &str = parts[0].trim();
            rules.push((body, head));
        } else if line.ends_with('.') {
            let fact: &str = line.trim_end_matches('.').trim();
            if !fact.is_empty() {
                facts.push(fact);
            }
        } else {
            return Err(format!("Line {}: Invalid syntax - must end with period or contain implication", line_number + 1));
        }
    }
    Ok(KnowledgeBase::from_strings(facts, rules)?)
}

fn print_help() {
    println!("Usage: pie <path> [OPTIONS]\n");
    println!("OPTIONS:");
    println!("  --file <path>    Path to knowledge base file (optional if path is first argument)");
    println!("  --debug          Enable debug mode");
    println!("  --query          Enter interactive query mode, type 'quit' to exit");
    println!("  --prove <fact>   Prove a specific fact");
    println!("  --help           Show this help message\n");
    println!("FILE FORMAT:");
    println!("  Facts:    parent(anna, bob).");
    println!("  Rules:    parent(x?, y?) & parent(y?, z?) -> grandparent(x?, z?).");
    println!("  Comments: Lines starting with #");
}
