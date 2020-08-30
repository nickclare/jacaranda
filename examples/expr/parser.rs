use super::*;

#[derive(Debug)]
pub struct ParseError; // for this simple example, we don't really care what the error is

pub fn parse(_text: &str) -> Result<Tree<AstNode>, ParseError> {
    unimplemented!()
}

pub struct ParseState<'a> {
    pub input: &'a [u8],
    pub index: usize,
    pub tree: Tree<AstNode>,
}

pub type ParseResult<T> = Result<T, ParseError>;

impl<'a> ParseState<'a> {
    // simple matchers

    // match input character
    pub fn match_byte(&mut self, c: u8) -> ParseResult<()> {
        if self.input[self.index] == c.into() {
            self.index += 1;
            Ok(())
        } else {
            Err(ParseError)
        }
    }

    pub fn match_char(&mut self, c: char) -> ParseResult<()> {
        self.match_byte(c as u8)
    }

    pub fn match_digit(&mut self) -> ParseResult<i64> {
        let c = self.input[self.index] as char;
        c.to_digit(10)
            .map(|d| {
                self.index += 1;
                d as i64
            })
            .ok_or(ParseError)
    }

    pub fn match_operator(&mut self) -> ParseResult<Operator> {
        let c = self.input[self.index] as char;
        let op = match c {
            '+' => Some(Operator::Add),
            '-' => Some(Operator::Sub),
            '*' => Some(Operator::Mul),
            '/' => Some(Operator::Div),
            _ => None,
        };

        op.ok_or(ParseError)
    }

    pub fn peek(&self) -> char {
        self.input[self.index] as char
    }

    // parser functions
    pub fn parse_literal(&mut self, parent: NodeIndex) -> ParseResult<NodeIndex> {
        let v = self.match_digit()?;
        self.tree.add(parent, AstNode::Literal(v)).ok_or(ParseError)
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn match_char_test() {}
}
