use super::error::BasmError;
use super::lexer::{Token, TokenType};
use super::node::{Node, NodeData};

use substring::Substring;

#[derive(Debug, Default)]
enum RootState {
    #[default]
    Root,
    SemVer,
    VM,
    Agent,
}

#[derive(Debug)]
enum ParsingState {
    Root(RootState),
}

impl Default for ParsingState {
    fn default() -> Self {
        ParsingState::Root(RootState::default())
    }
}

#[derive(Debug, Default)]
struct ParseData {
    token: Token,
    state: ParsingState,
    errors: Vec<BasmError>,

    root: Node,
    child_idx: Option<usize>,
    _grandchild_idx: Option<usize>,
}

fn unexpected_token_error(token: &Token, errors: &mut Vec<BasmError>) {
    let error = BasmError::UnexpectedTokenInTopLevel {
        line: token.line,
        data: token.data.clone(),
    };
    errors.push(error.clone());
}

// i know this code sucks.
// please fix this future me
// or some random person on github looking at this
fn parse_token(data: &mut ParseData) {
    match &mut data.state {
        ParsingState::Root(_) => parse_rootstate_token(data),
    }
}

fn parse_rootstate_token(data: &mut ParseData) {
    let token = &data.token;
    let ParsingState::Root(root_state) = &mut data.state;
    match root_state {
        RootState::Root => {
            match token.token_type {
                TokenType::Keyword => {
                    if ["sem_ver", "vm", "agent"].contains(&token.data.as_str()) {
                        // Decides the type of node.
                        let node_data = match token.data.as_str() {
                            "sem_ver" => NodeData::SemVer,
                            "vm" => NodeData::VM,
                            "agent" => NodeData::Agent,
                            _ => unreachable!(),
                        };

                        // Creates the node and sets data.child_idx to its index in root.branches
                        data.root.branches.push(Node {
                            data: node_data.clone(),
                            branches: Vec::new(),
                            line: token.line,
                        });
                        data.child_idx = Some(data.root.branches.len() - 1);

                        // Sets root_state to the appropriate state
                        *root_state = match token.data.as_str() {
                            "sem_ver" => RootState::SemVer,
                            "vm" => RootState::VM,
                            "agent" => RootState::Agent,
                            _ => unreachable!(),
                        };
                    } else {
                        unexpected_token_error(token, &mut data.errors);
                    }
                }
                TokenType::Literal => {
                    let mut chars = token.data.chars();

                    // Both assume there is at least one character in the symbol
                    let first_char = chars.next().unwrap();
                    let last_char = chars.last().unwrap_or(first_char);

                    // Decides the type of literal
                    let key_type: NodeData;
                    if first_char == '[' && last_char == ']' {
                        // Is target?
                        key_type = NodeData::Target;
                    } else if first_char == '{' && last_char == '}' {
                        // Is monitor?
                        key_type = NodeData::Monitor;
                    } else {
                        unexpected_token_error(token, &mut data.errors);
                        *root_state = RootState::Root;
                        return;
                    }

                    let trimmed_data = token.data.substring(1, token.data.len() - 1).to_owned(); // Removes first and last character

                    // Creates the node
                    data.root.branches.push(Node {
                        data: key_type,
                        branches: vec![Node {
                            data: NodeData::StringData(trimmed_data),
                            branches: Vec::new(),
                            line: token.line,
                        }],
                        line: token.line,
                    });
                }
                TokenType::Punctuator => unexpected_token_error(token, &mut data.errors),
                TokenType::Placeholder => unreachable!(),
            }
        }
        _ => {
            if token.token_type != TokenType::Literal {
                *root_state = RootState::Root;
                unexpected_token_error(token, &mut data.errors);
                return;
            }

            let mut chars = token.data.chars();

            // Both assume there is at least one character in the symbol
            let first_char = chars.next().unwrap();
            let last_char = chars.last().unwrap_or(first_char);

            if !(first_char == '"' && last_char == '"') {
                *root_state = RootState::Root;
                unexpected_token_error(token, &mut data.errors);
                return;
            }

            let trimmed_data = token.data.substring(1, token.data.len() - 1).to_owned(); // Removes first and last character

            let node_data = match root_state {
                RootState::SemVer => NodeData::SemVer,
                RootState::VM => NodeData::VM,
                RootState::Agent => NodeData::Agent,
                _ => unreachable!(),
            };

            // Creates the node
            data.root.branches.push(Node {
                data: node_data,
                branches: vec![Node {
                    data: NodeData::StringData(trimmed_data),
                    branches: Vec::new(),
                    line: token.line,
                }],
                line: token.line,
            });

            // Resets the state back to root
            *root_state = RootState::Root;
            data.child_idx = None;
        }
    }
}

pub fn parse(tokens: &Vec<Token>) -> Node {
    let mut data = ParseData::default();
    for token in tokens {
        data.token = token.clone();
        parse_token(&mut data);
    }
    todo!("review this function and check if it would actually parse things")
}
