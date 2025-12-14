// Copyright (C) 2025 KyteKode

// i honestly have no idea if this works or not, i havent tested it

#![allow(unused)]

use std::collections::HashSet;

use crate::errors::throw_error;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum WarningType {
    Op,
    Uid,
    Parent,
    Next,
    In,
    Field,
    Mut,
    Shadow,
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct CompilationData {
    pub outname: String,
    pub source: String,

    pub version: bool,
    pub verbose: bool,
    pub log: bool,

    pub stdout: bool,

    pub reverse: bool,

    pub warn: HashSet<WarningType>,
    pub no_warn: HashSet<WarningType>,
    pub wall: bool,
    pub werror: bool
}


#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Token {
    Block,
    Uid,
    Opcode,
    Parent,
    Next,
    In,
    Field,
    Mut,
    Shadow,
    StringDecl,
    DoubleDecl,
    IntDecl,
    ListIdxDecl,
    WaitDecl,
    BlockPtrDecl,
    SubstackDecl,
    ReceivedBroadcastDecl,
    DataPtrDecl,
    End,
    NullLit,
    StringLit(String),
    NumLit(String),
    BoolLit(bool),
    IsStageProperty,
    NameProperty,
    VarData,
    BigMonitorProperty,
    SliderMonitorProperty,
    VarGlobalScopeProperty,
    VarCloudScopeProperty,
    ListData,
    ListGlobalScopeProperty,
    BroadcastProperty,
    CurrentCostumeProperty,
    VectorCostumeAsset,
    BitmapCostumeAsset,
    SoundAsset,
    VolumeProperty,
    SpriteHeader(String)
}

#[derive(Debug, PartialEq, Eq)]
enum TokenizationMode {
    Normal,
    String,
    SpriteHeader
}

#[allow(clippy::needless_return)]
fn tokenize_string(s_token: String) -> Token {
    return match &*s_token {
        "block" => Token::Block,
        "uid" => Token::Uid,
        "opcode" => Token::Opcode,
        "parent" => Token::Parent,
        "next" => Token::Next,
        "in" => Token::In,
        "field" => Token::Field,
        "mut" => Token::Mut,
        "string" => Token::StringDecl,
        "double" => Token::DoubleDecl,
        "int" => Token::IntDecl,
        "list_idx" => Token::ListIdxDecl,
        "wait" => Token::WaitDecl,
        "block_ptr" => Token::BlockPtrDecl,
        "substack" => Token::SubstackDecl,
        "received_broadcast" => Token::ReceivedBroadcastDecl,
        "data_ptr" => Token::DataPtrDecl,
        "end" => Token::End,
        "null" => Token::NullLit,
        ".isStage" => Token::IsStageProperty,
        ".name" => Token::NameProperty,
        ".var" => Token::VarData,
        ".bigmonitor" => Token::BigMonitorProperty,
        ".slidermonitor" => Token::SliderMonitorProperty,
        ".varglobalscope" => Token::VarGlobalScopeProperty,
        ".varcloudscope" => Token::VarCloudScopeProperty,
        ".list" => Token::ListData,
        ".listglobalscope" => Token::ListGlobalScopeProperty,
        ".broadcast" => Token::BroadcastProperty,
        ".currentCostume" => Token::CurrentCostumeProperty,
        ".costumeVector" => Token::VectorCostumeAsset,
        ".costumeBitmap" => Token::BitmapCostumeAsset,
        ".sound" => Token::SoundAsset,
        ".volume" => Token::VolumeProperty,
        misc => {
            let first = misc.chars().next();
            let last = misc.chars().last();

            if let (Some(ufirst), Some(ulast)) = (first, last) {
                if ufirst == '"' && ulast == '"' {
                    let chars: Vec<char> = misc.chars().collect();
                    let result: String = chars[1..chars.len()-1].iter().collect();
                    return Token::StringLit(result);
                } else if ufirst == '[' && ulast == ']' {
                    let chars: Vec<char> = misc.chars().collect();
                    let result: String = chars[1..chars.len()-1].iter().collect();
                    return Token::SpriteHeader(result);
                } else if misc == "true" || misc == "false" {
                    return Token::BoolLit(misc == "true");
                } else {
                    let parsed = misc.parse::<f64>();
                    match parsed {
                        Ok(_) => Token::NumLit(misc.to_string()),
                        Err(_) => throw_error(format!("{} is not a keyword, string, or number", misc))
                    }
                }
            } else {
                unreachable!()
            }
        }
    };
}

pub fn tokenize(basm_code: &str) -> Vec<Token> {
    let mut tokens: Vec<Token> = Vec::new();

    let chars = basm_code.chars().collect::<Vec<_>>();
    let chars_iter = chars.iter();

    let mut s_token = String::new();

    let mut mode = TokenizationMode::Normal;

    let mut string_start = 0;
    let mut line = 1;

    for (idx, ch) in chars_iter.enumerate() {
        if ch == &'\n' { line += 1; }

        // Handles string literals
        if ch == &'"' {
            match mode {
                TokenizationMode::Normal => {
                    string_start = line;
                    mode = TokenizationMode::String;
                },
                TokenizationMode::String => {
                    if chars[idx] == '\\' {
                        s_token.push('"');
                        mode = TokenizationMode::String;
                    }
                    mode = TokenizationMode::Normal;
                },
                TokenizationMode::SpriteHeader => {
                    s_token.push('"');
                    mode = TokenizationMode::SpriteHeader;
                }
            }
        }

        // Opens sprite headers
        if ch == &'[' {
            s_token.push('[');
            mode = match mode {
                TokenizationMode::Normal => TokenizationMode::SpriteHeader,
                TokenizationMode::String => TokenizationMode::String,
                TokenizationMode::SpriteHeader => TokenizationMode::SpriteHeader
            }
        }

        // Closes sprite headers
        if ch == &']' {
            s_token.push(']');
            mode = match mode {
                TokenizationMode::Normal => throw_error(format!("Line {}: Cannot close sprite header without opening it", line)),
                TokenizationMode::String => TokenizationMode::String,
                TokenizationMode::SpriteHeader => TokenizationMode::Normal
            }
        }

        // Handles other characters
        if (ch == &' ' || ch == &'\n') && mode == TokenizationMode::Normal {
            if s_token != "" {
                tokens.push(tokenize_string(s_token));
            }
            s_token = String::new();
        } else if ch == &'\n' && mode == TokenizationMode::SpriteHeader {
            throw_error(format!("Line {}: Cannot use newline in sprite header", line));
        } else {
            s_token.push(*ch);
        }
    }

    if mode == TokenizationMode::String { throw_error(format!("Unterminated string on line {}", string_start.to_string())); }

    tokens
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Type {
    String,
    Double,
    Int,
    ListIdx,
    Wait,
    BlockPtr,
    Substack,
    RecievedBroadcast,
    DataPtr
}

#[derive(Debug, Clone, PartialEq, Default)]
pub enum MonitorType {
    #[default]
    Normal,
    Big,
    Slider(Type, f64, f64)
}

#[derive(Debug, Clone, PartialEq)]
pub enum Node {
    Root(Vec<Node>),
    Sprite(Vec<Node>),
    NameProperty(String),
    IsStageProperty(bool),
    // visible, uid, name, type, value, monitor type
    VariableData(bool, String, String, Type, String, MonitorType),
    // visible, values
    ListData(bool, Vec<String>),
    // uid, name
    BroadCastProperty(String, String),
    // name, path to svg, x center, y center
    CostumeVector(String, String, f64, f64),
    // TODO: do bitmap costume and sound
    VolumeProperty(f64),
    Block(Vec<Node>),
    Uid(String),
    Parent(String),
    Next(String),
    In(String, Box<Node>),
    Field(String, String),
    NullData,
    StringData(String),
    DoubleSata(f64),
    IntData(i64),
    ListIdxData(i64),
    WaitData(f64),
    BlockPtrData(String),
    SubstackData(String),
    ReceivedBroadcastData(String),
    DataPtrData(String)
}

#[derive(Debug, PartialEq, Eq)]
enum ParsingState {
    Root,
    Sprite
}

fn parse_token(token: &Token, root: &mut Node, state: &ParsingState) -> Node {
    let Node::Root(ref mut children) = *root else {
        unreachable!()
    };

    match token {
        Token::SpriteHeader(name) => {
            if *state == ParsingState::Root {
            }
        },
        _ => todo!("Parse other tokens")
    }

    todo!("Finish parsing for single token")
}

pub fn parse(tokens: &Vec<Token>) -> Node {
    let mut root = Node::Root(Vec::new());

    todo!("Add actual parsing");
}

// old parsing code from before sprites were added to basm
/*pub enum Node {
    Root(Vec<Node>),
    Block(Vec<Node>),
    Uid(String),
    Opcode(String),
    Parent(String),
    Next(String),
    In(String, Box<Node>),
    Field(String, Box<Node>),
    NullData,
    StringData(String),
    DoubleData(f64),
    IntData(i64),
    ListIdxData(i64),
    WaitData(f64),
    BlockPtrData(String),
    SubstackData(String),
    ReceivedBroadcastData(String),
    DataPtrData(String),
    Placeholder
}

#[derive(PartialEq)]
enum Type {
    String,
    Double,
    Int,
    ListIdx,
    Wait,
    BlockPtr,
    Substack,
    ReceivedBroadcast,
    DataPtr
}

#[derive(PartialEq)]
enum ParsingState {
    Normal,
    Block,
    BlockUid,
    BlockOpcode,
    BlockParent,
    BlockNext,
    BlockInKey,
    BlockInType,
    BlockInVal(Type),
    BlockFieldKey,
    BlockFieldType,
    BlockFieldVal(Type),
    BlockMut
}
fn parse_type_annotation(state: &mut ParsingState, parsing_type: Type, name: &str) {
    if *state == ParsingState::BlockInType {
        *state = ParsingState::BlockInVal(parsing_type);
    } else if *state == ParsingState::BlockFieldType {
        *state = ParsingState::BlockFieldVal(parsing_type);
    } else {
        error(format!("The {} type annotation cannot be used outside input and field data", name))
    }
}

#[allow(unused_assignments)]
fn parse(tokens: Vec<Token>) -> Node {
    let mut ast = Node::Root(vec![]);
    let mut state = ParsingState::Normal;

    if let Node::Root(ref mut root_vec) = ast {
        for token in tokens {
            match token {
                Token::Block => {
                    if state == ParsingState::Normal {
                        state = ParsingState::Block;
                        root_vec.push(Node::Block(vec![]));
                    } else {
                        error("Cannot define block inside block".to_string());
                    }
                },
                Token::Uid => {
                    if state == ParsingState::Block {
                        state = ParsingState::BlockUid;
                    } else {
                        error("Cannot use uid outside of block".to_string());
                    }
                },
                Token::Opcode => {
                    if state == ParsingState::Block {
                        state = ParsingState::BlockOpcode;
                    } else {
                        error("Cannot use opcode outside of block".to_string());
                    }
                },
                Token::Parent => {
                    if state == ParsingState::Block {
                        state = ParsingState::BlockParent;
                    } else {
                        error("Cannot use parent outside of block".to_string());
                    }
                },
                Token::Next => {
                    if state == ParsingState::Block {
                        state = ParsingState::BlockNext;
                    } else {
                        error("Cannot use next outside of block".to_string())
                    }
                },
                Token::In => {
                    if state == ParsingState::Block {
                        state = ParsingState::BlockInKey;
                    } else {
                        error("Cannot use in outside of block".to_string());
                    }
                },
                Token::StringDecl => parse_type_annotation(&mut state, Type::String, "string"),
                Token::DoubleDecl => parse_type_annotation(&mut state, Type::Double, "double"),
                Token::IntDecl => parse_type_annotation(&mut state, Type::Int, "int"),
                Token::ListIdxDecl => parse_type_annotation(&mut state, Type::ListIdx, "list_idx"),
                Token::WaitDecl => parse_type_annotation(&mut state, Type::Wait, "wait"),
                Token::BlockPtrDecl => parse_type_annotation(&mut state, Type::BlockPtr, "block_ptr"),
                Token::SubstackDecl => parse_type_annotation(&mut state, Type::Substack, "substack"),
                Token::ReceivedBroadcastDecl => parse_type_annotation(&mut state, Type::ReceivedBroadcast, "received_broadcast"),
                Token::DataPtrDecl => parse_type_annotation(&mut state, Type::DataPtr, "data_ptr"),
                Token::StringLit(data) => {
                    if let Some(block) = root_vec.last_mut() {
                        if let Node::Block(block_contents) = block {
                            block_contents.push(match state {
                                ParsingState::BlockUid => Node::Uid(data),
                                ParsingState::BlockOpcode => Node::Opcode(data),
                                ParsingState::BlockParent => Node::Parent(data),
                                ParsingState::BlockNext => Node::Next(data),
                                ParsingState::BlockInKey => {
                                    state = ParsingState::BlockInType;
                                    Node::In(data, Box::new(Node::Placeholder))
                                },
                                ParsingState::BlockInVal(val_type) => {
                                    state = ParsingState::Block;
                                    if let Some(Node::In(_, boxed_data)) = block_contents.last_mut() {
                                        *boxed_data = Box::new(match val_type {
                                            Type::String => Node::StringData(data),
                                            Type::BlockPtr => Node::BlockPtrData(data),
                                            Type::Substack => Node::SubstackData(data),
                                            Type::ReceivedBroadcast => Node::ReceivedBroadcastData(data),
                                            Type::DataPtr => Node::DataPtrData(data),
                                            _ => { Node::Placeholder } //Placeholder, give error code
                                        });
                                    }
                                    return Node::Placeholder;
                                },
                                _ => { Node::Placeholder } // here for the same reason as the one down there
                            });
                        }
                    }
                    state = ParsingState::Block;
                },
                _ => {} // here so rust doesn't scream when im not finished with all the arms
            }
        }
    }

    ast
}

#[allow(unused)]
fn compile(basm_code: &str) {
    let tokens: Vec<Token> = tokenize(basm_code);
    //let ast: Node = parse(tokens);
    //wip
} */