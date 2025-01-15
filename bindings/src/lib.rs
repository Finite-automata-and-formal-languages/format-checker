use dandy::dfa::Dfa;
use dandy::dfa::parse::DfaParseError;
use dandy::nfa::Nfa;
use dandy::nfa::parse::NfaParseError;
use dandy::grammar::Grammar;
use dandy::grammar::parse::GrammarParseError;
use dandy::regex::Regex;
use dandy::parser::{dfa, nfa, regex, grammar};
use std::collections::HashMap;
use std::sync::LazyLock;
use std::sync::Mutex;
use wasm_bindgen::prelude::wasm_bindgen;

static SOLUTION_STR: &str = include_str!("../assignments.yaml");

static SOLUTION: LazyLock<HashMap<String, String>> = LazyLock::new(|| {
    serde_yaml::from_str(&SOLUTION_STR).unwrap()
});

#[wasm_bindgen]
pub fn check_dfa(test: &str, input: &str) -> Result<(), String> {
    let _dfa: Dfa = dfa(input)
        .map_err(|e| format!("Error parsing DFA: {e}"))?
        .try_into()
        .map_err(|e: DfaParseError| e.to_string())?;
    match test {
        "valid" => (),
        _ => return Err(format!("test {test} not implemented"))
    }
    Ok(())
}

#[wasm_bindgen]
pub fn check_nfa(test: &str, input: &str) -> Result<(), String> {
    let proposed_nfa: Nfa = nfa(input)
        .map_err(|e| format!("Error parsing NFA: {e}"))?
        .try_into()
        .map_err(|e: NfaParseError| e.to_string())?;
    match test {
        "valid" => (),
        s => {
            let solution_nfa: Nfa = nfa(s)
                .map_err(|e| format!("Error parsing solution NFA: {e}"))?
                .try_into()
                .map_err(|e: NfaParseError| e.to_string())?;
            // let x = solution_nfa.separable_from(&proposed_nfa);
        }
        _ => return Err(format!("test {test} not implemented"))
    }
    Ok(())
}

#[wasm_bindgen]
pub fn check_regex(test: &str, input: &str) -> Result<(), String> {
    let regex: Regex = regex(input)
        .map_err(|e| format!("Error parsing regex: {e}"))?;
    let nfa = regex.to_nfa();
    match test {
        "valid" => (),
        "assignment-3-2" => {
            
        }
        _ => return Err(format!("test {test} not implemented"))
    }
    Ok(())
}

#[wasm_bindgen]
pub fn check_grammar(test: &str, input: &str) -> Result<(), String> {
    let _grammar: Grammar = grammar(input)
        .map_err(|e| format!("Error parsing CFG: {e}"))?
        .try_into()
        .map_err(|e: GrammarParseError| e.to_string())?;
    match test {
        "valid" => (),
        _ => return Err(format!("test {test} not implemented"))
    }
    Ok(())
}
