#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Token {
    /// https://262.ecma-international.org/#sec-punctuators
    Punctoator(char),
    /// https://262.ecma-international.org/#sec-litarals-numeric-literals
    Number(u64),
}
