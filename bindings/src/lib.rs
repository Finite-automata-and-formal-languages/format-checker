use dandy::dfa::parse::DfaParseError;
use dandy::dfa::Dfa;
use dandy::grammar::parse::GrammarParseError;
use dandy::grammar::Grammar;
use dandy::nfa::parse::NfaParseError;
use dandy::nfa::Nfa;
use dandy::parser::{dfa, grammar, nfa, regex};
use dandy::regex::Regex;
use std::collections::HashMap;
use std::sync::LazyLock;
use wasm_bindgen::prelude::wasm_bindgen;

/// Expected format is
/// ```yaml
/// assignment-3-2: |
///   the code for
///   the automata
/// ...
/// ```
/// The contents of the file are supposed to be stored as a GitHub secret,
/// and are written to this file at deployment.
static SOLUTION_STR: &str = include_str!("../assignments.yaml");

static SOLUTION: LazyLock<HashMap<String, String>> =
    LazyLock::new(|| serde_yaml::from_str(&SOLUTION_STR).unwrap());

#[wasm_bindgen]
pub fn check_dfa(test: &str, input: &str) -> Result<(), String> {
    let proposed_dfa: Dfa = dfa(input)
        .map_err(|e| format!("Error parsing DFA: {e}"))?
        .try_into()
        .map_err(|e: DfaParseError| e.to_string())?;

    if test == "valid" {
        return Ok(());
    }

    let solution_dfa_src = SOLUTION
        .get(test)
        .ok_or(format!("No solution for {}", test))?;

    let solution_dfa: Dfa = dfa(solution_dfa_src)
        .map_err(|e| format!("Error parsing solution DFA: {e}"))?
        .try_into()
        .map_err(|e: DfaParseError| e.to_string())?;

    if let Some(sep) = solution_dfa.separable_from(&proposed_dfa) {
        let e = match sep {
            None => "The provided NFA has the wrong alphabet!".to_string(),
            Some(w) => format!(
                "The solution and the provided NFA disagree on the word {}",
                w
            ),
        };
        Err(e)
    } else {
        Ok(())
    }
}

#[wasm_bindgen]
pub fn check_nfa(test: &str, input: &str) -> Result<(), String> {
    let proposed_nfa: Nfa = nfa(input)
        .map_err(|e| format!("Error parsing NFA: {e}"))?
        .try_into()
        .map_err(|e: NfaParseError| e.to_string())?;

    if test == "valid" {
        return Ok(());
    }

    let solution_nfa_src = SOLUTION
        .get(test)
        .ok_or(format!("No solution for {}", test))?;

    let solution_nfa: Nfa = nfa(solution_nfa_src)
        .map_err(|e| format!("Error parsing solution NFA: {e}"))?
        .try_into()
        .map_err(|e: NfaParseError| e.to_string())?;

    if let Some(sep) = solution_nfa.separable_from(&proposed_nfa) {
        let e = match sep {
            None => "The provided NFA has the wrong alphabet!".to_string(),
            Some(w) => format!(
                "The solution and the provided NFA disagree on the word {}",
                w
            ),
        };
        Err(e)
    } else {
        Ok(())
    }
}

#[wasm_bindgen]
pub fn check_regex(test: &str, input: &str) -> Result<(), String> {
    let proposed_regex: Regex = regex(input).map_err(|e| format!("Error parsing regex: {e}"))?;

    if test == "valid" {
        return Ok(());
    }

    let solution_regex_src = SOLUTION
        .get(test)
        .ok_or(format!("No solution for {}", test))?;

    let solution_regex: Regex =
        regex(solution_regex_src).map_err(|e| format!("Error parsing solution Regex: {e}"))?;

    if let Some(sep) = solution_regex
        .to_nfa()
        .separable_from(&proposed_regex.to_nfa())
    {
        let e = match sep {
            None => "The provided Regex has the wrong alphabet!".to_string(),
            Some(w) => format!(
                "The solution and the provided Regex disagree on the word {}",
                w
            ),
        };
        Err(e)
    } else {
        Ok(())
    }
}

#[wasm_bindgen]
pub fn check_grammar(test: &str, input: &str) -> Result<(), String> {
    let _grammar: Grammar = grammar(input)
        .map_err(|e| format!("Error parsing CFG: {e}"))?
        .try_into()
        .map_err(|e: GrammarParseError| e.to_string())?;
    match test {
        "valid" => (),
        _ => return Err(format!("test {test} not implemented")),
    }
    Ok(())
}
