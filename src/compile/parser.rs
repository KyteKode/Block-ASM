use super::lexer::{Token, TokenType};
use super::error::{throw_errors, BasmError};
use super::node::{Node, NodeData};

use substring::Substring;

#[derive(Debug, Default)]
struct BasmParser {
    tokens: Vec<Token>,
    root: Node,
    errors: Vec<BasmError>,
    idx: usize
}

impl BasmParser {
    fn unexpected_token_error(&mut self) {
        let token = self.tokens[self.idx].clone();

        self.errors.push(BasmError::UnexpectedTokenInTopLevel {
            line: token.line,
            data: token.data.clone()
        })
    }

    fn parse_top_level(&mut self) {
        // Unfortunately due to the borrow checker, I cannot use a for loop.
        while self.idx < self.tokens.len() {
            let token = self.tokens[self.idx].clone();

            match token.token_type {
                TokenType::Keyword => {
                    if ["sem_ver", "vm", "agent"].contains(&token.data.as_str()) {
                        self.parse_metadata();
                    } else {
                        self.unexpected_token_error();
                    }
                },
                TokenType::Literal => {
                    let mut chars = token.data.chars();
                    let first_char = chars.next().unwrap();
                    let last_char = chars.last().unwrap_or(first_char);

                    if (first_char == '[' && last_char == ']') || // Is target header?
                        (first_char == '{' && last_char == '}') // Is monitor header?
                    {
                        todo!("implement parse_header");
                        //self.parse_header();
                    }
                },
                TokenType::Punctuator => {
                    self.unexpected_token_error();
                }
            }

            self.idx += 1;
        }
    }

    fn parse_metadata(&mut self) {
        let token = self.tokens[self.idx].clone();

        let new_key = self.root.branches.ref_push(Node {
            data: match token.data.as_str() {
                "sem_ver" => NodeData::SemVer,
                "vm" => NodeData::VM,
                "agent" => NodeData::Agent,
                _ => unreachable!()
            },
            branches: Vec::new(),
            line: token.line
        });

        let mut chars = token.data.chars();

        // Both assumes there is at least one character in the symbol
        let first_char = chars.next().unwrap();
        let last_char = chars.last().unwrap_or(first_char);

        if first_char == '"' && last_char == ']' {
            let data = token.data;
            new_key.branches.push(Node {
                data: NodeData::StringData(
                    data.substring(1, data.len() - 1).to_owned()
                ),
                branches: Vec::new(),
                line: token.line
            })
        }

        self.idx += 1;
    }
}

// Initializes the BasmParser and parses the tokens.
pub fn parse(tokens: &Vec<Token>) -> Node {
    let mut basm_parser = BasmParser {
        tokens: tokens.clone(),
        ..Default::default()
    };

    basm_parser.parse_top_level();

    // Throws all queued errors
    if !basm_parser.errors.is_empty() {
        throw_errors(basm_parser.errors);
    }

    basm_parser.root
}





// Adds custom function for vectors.
// It pushes data to the vector and returns the reference to that data.
trait VecPushMut<T> {
    fn ref_push(&mut self, value: T) -> &mut T;
}

impl<T> VecPushMut<T> for Vec<T> {
    fn ref_push(&mut self, value: T) -> &mut T {
        self.push(value);
        self.last_mut().unwrap()
    }
}
