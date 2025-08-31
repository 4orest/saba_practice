use alloc::string::String;
use alloc::vec::Vec;

pub struct JsLexer {
    pos: usize,
    input: Vec<char>,
}

impl JsLexer {
    pub fn new(js: String) -> Self {
        Self {
            pos: 0,
            input: js.chars().collect(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Token {
    /// https://262.ecma-international.org/#sec-punctuators
    Punctoator(char),
    /// https://262.ecma-international.org/#sec-litarals-numeric-literals
    Number(u64),
}
