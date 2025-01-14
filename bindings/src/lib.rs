use dandy::dfa::Dfa;
use dandy::dfa::parse::DfaParseError;
use dandy::nfa::Nfa;
use dandy::nfa::parse::NfaParseError;
use dandy::grammar::Grammar;
use dandy::grammar::parse::GrammarParseError;
use dandy::regex::Regex;
use dandy::parser::{dfa, nfa, regex, grammar};
use wasm_bindgen::prelude::wasm_bindgen;

trait AcceptsGraphemes {
    fn accepts_graphemes(&self, word: &str) -> bool;

    fn should_accept(&self, word: &str) -> Result<(), String> {
        if !self.accepts_graphemes(word) {
            return Err(format!("The word `{word}` should be accepted, but it is rejected."))
        }
        Ok(())
    }

    fn should_reject(&self, word: &str) -> Result<(), String> {
        if self.accepts_graphemes(word) {
            return Err(format!("The word `{word}` should be rejected, but it is accepted."))
        }
        Ok(())
    }
}

impl AcceptsGraphemes for Dfa {
    fn accepts_graphemes(&self, word: &str) -> bool {
        self.accepts_graphemes(word)
    }
}

impl AcceptsGraphemes for Nfa {
    fn accepts_graphemes(&self, word: &str) -> bool {
        self.accepts_graphemes(word)
    }
}

impl AcceptsGraphemes for Regex {
    fn accepts_graphemes(&self, word: &str) -> bool {
        self.clone().to_nfa().accepts_graphemes(word)
    }
}

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
    let nfa: Nfa = nfa(input)
        .map_err(|e| format!("Error parsing NFA: {e}"))?
        .try_into()
        .map_err(|e: NfaParseError| e.to_string())?;
    match test {
        "valid" => (),
        "assignment-2-3-1" => {
            nfa.should_accept("abaab")?;
            nfa.should_accept("babbabb")?;
            nfa.should_reject("ababab")?;
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
            nfa.should_accept("101")?;
            nfa.should_accept("10101")?;
            nfa.should_reject("11")?;
            nfa.should_reject("1001")?;
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
