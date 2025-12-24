// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (C) 2025 KyteKode

// I honestly have no idea if this works or not, I haven't tested it.

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

    XPos,
    YPos,

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

    DataBlock,
    Name,

    VarData,
    IsCloud,
    Mode,
    SpriteName,
    Value,
    Width,
    Height,
    Visible,
    SliderMin,
    SliderMax,
    IsDiscrete,

    NullLit,
    StringLit(String),
    NumLit(String),
    BoolLit(bool),
    IsStageProperty,
    ListData,
    ListGlobalScopeProperty,
    BroadcastProperty,
    CurrentCostumeProperty,
    VectorCostumeAsset,
    BitmapCostumeAsset,
    SoundAsset,
    VolumeProperty,
    SpriteHeader(String),

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
        "shadow" => Token::Shadow,

        "x_pos" => Token::XPos,
        "y_pos" => Token::YPos,

        "string" => Token::StringDecl,
        "double" => Token::DoubleDecl,
        "int" => Token::IntDecl,
        "list_idx" => Token::ListIdxDecl,
        "wait" => Token::WaitDecl,
        "block_ptr" => Token::BlockPtrDecl,
        "substack" => Token::SubstackDecl,
        "received_broadcast" => Token::ReceivedBroadcastDecl,
        "data_ptr" => Token::DataPtrDecl,

        "data_block" => Token::DataBlock,
        "name" => Token::Name,

        "end" => Token::End,

        "var" => Token::VarData,
        "is_cloud" => Token::IsCloud,
        "mode" => Token::Mode,
        "sprite_name" => Token::SpriteName,
        "value" => Token::Value,
        "width" => Token::Width,
        "height" => Token::Height,
        "visible" => Token::Visible,
        "slider_min" => Token::SliderMin,
        "slider_max" => Token::SliderMax,
        "is_discrete" => Token::IsDiscrete,

        "null" => Token::NullLit,
        ".isStage" => Token::IsStageProperty,
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
pub enum SB3Type {
    String,
    Double,
    Int,
    ListIdx,
    Wait, // used in wait blocks
    BlockPtr, // used for shadow blocks and blocks inside blocks
    Substack, // used in boolean inputs and c blocks
    RecievedBroadcast,
    DataPtr // used in variable, list, and sent broadcast dropdowns
}

#[derive(Debug, Clone, PartialEq, Default)]
pub enum MonitorType {
    #[default]
    Normal,
    Big,
    Slider(SB3Type, f64, f64)
}

#[derive(Debug, Clone, PartialEq)]
pub enum Node {
    Root(Vec<Node>),
    Sprite(Vec<Node>),
    NameProperty(String),
    IsStageProperty(bool),
    // visible, uid, name, type, value, monitor type
    VariableData(bool, String, String, SB3Type, String, MonitorType),
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
    DoubleData(f64),
    IntData(i64),
    ListIdxData(i64),
    WaitData(f64),
    BlockPtrData(String),
    SubstackData(String),
    ReceivedBroadcastData(String),
    DataPtrData(String)
}

#[derive(Debug, PartialEq, Eq, Clone)]
enum ParsingState {
    Root,
    Sprite,
    Block,
    BlockUid,
    BlockOpcode,
    BlockParent,
    BlockNext,
    BlockInKey,
    BlockInType(String),
    BlockInVal(String, SB3Type),
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
    let Node::Root(ref mut root_data) = *root else {
        unreachable!()
    };

    let mut sprite_data: Option<&mut Vec<Node>> = None;
    let mut block_data: Option<&mut Vec<Node>> = None;

    match token {
        Token::SpriteHeader(name) => {
            root_data.push(Node::Sprite(vec![]));
            if let Node::Sprite(data) = root_data.last_mut().unwrap() {
                sprite_data = Some(data);
            } else {
                unreachable!()
            }

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
        Token::StringLit(data) => {
            match state {
                ParsingState::BlockInVal(name, val_type) => {
                    if [
                        SB3Type::String,
                        SB3Type::BlockPtr,
                        SB3Type::Substack,
                        SB3Type::RecievedBroadcast,
                        SB3Type::DataPtr
                    ].contains(&val_type) {
                        let value = match val_type {
                            SB3Type::String => Node::StringData(data.clone()),
                            SB3Type::BlockPtr => Node::BlockPtrData(data.clone()),
                            SB3Type::Substack => Node::SubstackData(data.clone()),
                            SB3Type::RecievedBroadcast => Node::ReceivedBroadcastData(data.clone()),
                            SB3Type::DataPtr => Node::DataPtrData(data.clone()),
                            _ => unreachable!()
                        };

                        if let Some(ref mut unwrapped_block_data) = block_data {
                            unwrapped_block_data.push(Node::In(name.clone(), Box::new(value)));
                        }
                    }
                }
                _ => todo!()
            }
        },
        Token::NumLit(data) => {
            match state {
                ParsingState::BlockInVal(name, val_type) => {
                    if [
                        SB3Type::Double,
                        SB3Type::Int,
                        SB3Type::ListIdx,
                        SB3Type::Wait,
                    ].contains(&val_type) {
                        let float_parsed = data.clone().parse::<f64>();
                        let int_parsed = data.clone().parse::<i64>();

                        let value = match val_type {
                            SB3Type::Double => {
                                if float_parsed.is_ok() {
                                    Node::DoubleData(float_parsed.unwrap())
                                } else {
                                    throw_error(format!("Could not parse value {} as a float (double)", data))
                                }
                            },
                            SB3Type::Int => {
                                if int_parsed.is_ok() {
                                    Node::IntData(int_parsed.unwrap())
                                } else {
                                    throw_error(format!("Could not parse value {} as an int (int)", data))
                                }
                            },
                            SB3Type::ListIdx => {
                                if int_parsed.is_ok() {
                                    Node::ListIdxData(int_parsed.unwrap())
                                } else {
                                    throw_error(format!("Could not parse value {} as an int (list_idx)", data))
                                }
                            },
                            SB3Type::Wait => {
                                if float_parsed.is_ok() {
                                    Node::WaitData(float_parsed.unwrap())
                                } else {
                                    throw_error(format!("Could not parse value {} as a float (double)", data))
                                }
                            },
                            _ => unreachable!()
                        };

                        if let Some(ref mut unwrapped_block_data) = block_data {
                            unwrapped_block_data.push(Node::In(name.clone(), Box::new(value)));
                        }
                    }
                }
                _ => todo!()
            }
        }
        _ => {
            if let ParsingState::BlockInType(name) = state.clone() {
                *state = ParsingState::BlockInVal(name, match token {
                    Token::StringDecl => SB3Type::String,
                    Token::DoubleDecl => SB3Type::Double,
                    Token::IntDecl => SB3Type::Int,
                    Token::ListIdxDecl => SB3Type::ListIdx,
                    Token::WaitDecl => SB3Type::Wait,
                    Token::BlockPtrDecl => SB3Type::BlockPtr,
                    Token::SubstackDecl => SB3Type::Substack,
                    Token::ReceivedBroadcastDecl => SB3Type::RecievedBroadcast,
                    Token::DataPtrDecl => SB3Type::DataPtr,
                    _ => unreachable!()
                });
            }
        }
    }

    todo!("Finish parsing for single token")
}