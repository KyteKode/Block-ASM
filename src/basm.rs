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
fn tokenize_string(
    s_token: String
) -> Token {
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

pub fn tokenize(
    basm_code: &str
) -> Vec<Token> {
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
    Sprite,
    Block,
    BlockUid,
    BlockOpcode,
    BlockParent,
    BlockNext,
    BlockInKey,
    BlockInType,
    BlockInVal,
    BlockFieldKey,
    BlockFieldVal,
    BlockMut,
    BlockShadow,
}

fn parse_change_state(
    state: &mut ParsingState,
    checked_state: ParsingState,
    result_state: ParsingState,
    error: &str
) {
    if *state == checked_state {
        *state = result_state;
    } else {
        throw_error(error.to_string());
    }
}

fn parse_token(
    token: &Token,
    root: &mut Node,
    state: &mut ParsingState
) -> Node {
    let Node::Root(ref mut children) = *root else {
        unreachable!()
    };

    match token {
        Token::SpriteHeader(name) => {
            parse_change_state(
                state,
                ParsingState::Root,
                ParsingState::Sprite,
                "Cannot use sprite header outside of global scope"
            );
        },
        Token::Block => {
            parse_change_state(
                state,
                ParsingState::Sprite,
                ParsingState::Block,
                "Cannot use block outside of sprite scope"
            );
        },
        Token::Uid => {
            parse_change_state(
                state,
                ParsingState::Block,
                ParsingState::BlockUid,
                "Cannot use uid outside of block scope"
            );
        },
        Token::Opcode => {
            parse_change_state(
                state,
                ParsingState::Block,
                ParsingState::BlockOpcode,
                "Cannot use opcode outside of block scope"
            );
        },
        Token::Parent => {
            parse_change_state(
                state,
                ParsingState::Block,
                ParsingState::BlockParent,
                "Cannot use parent outside of block scope"
            );
        },
        Token::Next => {
            parse_change_state(
                state,
                ParsingState::Block,
                ParsingState::BlockNext,
                "Cannot use next outside of block scope"
            );
        },
        Token::In => {
            parse_change_state(
                state,
                ParsingState::Block,
                ParsingState::BlockInKey,
                "Cannot use in outside of block scope"
            );
        },
        Token::Field => {
            parse_change_state(
                state,
                ParsingState::Block,
                ParsingState::BlockFieldKey,
                "Cannot use field outside of block scope"
            );
        },
        Token::Mut => {
            parse_change_state(
                state,
                ParsingState::Block,
                ParsingState::BlockMut,
                "Cannot use mut outside of block scope"
            );
        },
        Token::Shadow => {
            parse_change_state(
                state,
                ParsingState::Block,
                ParsingState::BlockShadow,
                "Cannot use shadow outside of block scope"
            );
        },
        Token::StringDecl => {
            if *state == ParsingState::BlockInType {
                todo!("Parse StringDecl for inputs");
                *state == ParsingState::BlockInVal;
            }
        }
        _ => todo!("Parse other tokens")
    }

    todo!("Finish parsing for single token")
}

pub fn parse(
    tokens: &Vec<Token>
) -> Node {
    let mut root = Node::Root(Vec::new());

    todo!("Add actual parsing")
}