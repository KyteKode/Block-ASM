// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (C) 2025 KyteKode

// I honestly have no idea if this works or not, I haven't tested it.

#![allow(unused)]

use std::collections::HashSet;

use crate::errors::maybe_unreachable;
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
    pub werror: bool,
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
    TopLevel,

    XPos,
    YPos,

    PrototypeAnnotation,
    BlockPtrAnnotation,
    SubstackAnnotation,
    DoubleAnnotation,
    IntAnnotation,
    PosIntAnnotation,
    PosDoubleAnnotation,
    AngleAnnotation,
    ColorAnnotation,
    StringAnnotation,
    BroadcastAnnotation,
    VarAnnotation,
    ListAnnotation,

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

    Path,
    Format,

    Costume,
    BitmapRes,
    CenterX,
    CenterY,

    Sound,
    Rate,
    Samples,

    Broadcast,

    List,
    Item,

    NullLit,
    StringLit(String),
    NumLit(String),
    BoolLit(bool),

    IsStageProperty,
    VolumeProperty,
    LayerProperty,

    SpriteHeader(String),
}

#[derive(Debug, PartialEq, Eq)]
enum TokenizationMode {
    Normal,
    String,
    SpriteHeader,
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
        "shadow" => Token::Shadow,
        "top_level" => Token::TopLevel,

        "x_pos" => Token::XPos,
        "y_pos" => Token::YPos,

        "prototype" => Token::PrototypeAnnotation,
        "block_ptr" => Token::BlockPtrAnnotation,
        "substack" => Token::SubstackAnnotation,
        "double" => Token::DoubleAnnotation,
        "pos_double" => Token::PosDoubleAnnotation,
        "pos_int" => Token::PosIntAnnotation,
        "int" => Token::IntAnnotation,
        "broadcast" => Token::BroadcastAnnotation,
        "variable" => Token::VarAnnotation,
        "list" => Token::ListAnnotation,

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

        "path" => Token::Path,
        "format" => Token::Format,

        "costume" => Token::Costume,
        "bitmap_res" => Token::BitmapRes,
        "center_x" => Token::CenterX,
        "center_y" => Token::CenterY,

        "sound" => Token::Sound,
        "rate" => Token::Rate,
        "sample" => Token::Samples,

        "broadcast" => Token::Broadcast,

        "list" => Token::List,
        "item" => Token::Item,

        "null" => Token::NullLit,

        "is_stage" => Token::IsStageProperty,
        "volume" => Token::VolumeProperty,
        "layer" => Token::LayerProperty,

        misc => {
            let first = misc.chars().next();
            let last = misc.chars().last();

            if let (Some(ufirst), Some(ulast)) = (first, last) {
                if ufirst == '"' && ulast == '"' {
                    let chars: Vec<char> = misc.chars().collect();
                    let result: String = chars[1..chars.len() - 1].iter().collect();
                    return Token::StringLit(result);
                } else if ufirst == '[' && ulast == ']' {
                    let chars: Vec<char> = misc.chars().collect();
                    let result: String = chars[1..chars.len() - 1].iter().collect();
                    return Token::SpriteHeader(result);
                } else if misc == "true" || misc == "false" {
                    return Token::BoolLit(misc == "true");
                } else {
                    let parsed = misc.parse::<f64>();
                    match parsed {
                        Ok(_) => Token::NumLit(misc.to_string()),
                        Err(_) => {
                            throw_error(format!("{} is not a keyword, string, or number", misc))
                        }
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
        if ch == &'\n' {
            line += 1;
        }

        // Handles string literals
        if ch == &'"' {
            match mode {
                TokenizationMode::Normal => {
                    string_start = line;
                    mode = TokenizationMode::String;
                }
                TokenizationMode::String => {
                    if chars[idx] == '\\' {
                        s_token.push('"');
                        mode = TokenizationMode::String;
                    }
                    mode = TokenizationMode::Normal;
                }
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
                TokenizationMode::SpriteHeader => TokenizationMode::SpriteHeader,
            }
        }

        // Closes sprite headers
        if ch == &']' {
            s_token.push(']');
            mode = match mode {
                TokenizationMode::Normal => throw_error(format!(
                    "Line {}: Cannot close sprite header without opening it",
                    line
                )),
                TokenizationMode::String => TokenizationMode::String,
                TokenizationMode::SpriteHeader => TokenizationMode::Normal,
            }
        }

        // Handles other characters
        if (ch == &' ' || ch == &'\n') && mode == TokenizationMode::Normal {
            if s_token != "" {
                tokens.push(tokenize_string(s_token));
            }
            s_token = String::new();
        } else if ch == &'\n' && mode == TokenizationMode::SpriteHeader {
            throw_error(format!(
                "Line {}: Cannot use newline in sprite header",
                line
            ));
        } else {
            s_token.push(*ch);
        }
    }

    if mode == TokenizationMode::String {
        throw_error(format!(
            "Unterminated string on line {}",
            string_start.to_string()
        ));
    }

    tokens
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SB3Type {
    Prototype,
    BlockPtr, // used for shadow blocks and blocks inside blocks
    Substack, // used in boolean inputs and c blocks
    Double,
    PosDouble,
    PosInt,
    Int,
    Angle,
    Color,
    String,
    Broadcast,
    Var,
    List,
}

#[derive(Debug, Clone, PartialEq, Default)]
pub enum MonitorType {
    #[default]
    Normal,
    Big,
    Slider(SB3Type, f64, f64),
}

#[derive(Debug, Clone, PartialEq)]
pub enum Node {
    Root(Vec<Node>),

    // name, blocks, is stage, volume, layer
    Sprite(String, Vec<Node>, bool, f64, i64),

    NameProperty(String),
    IsStageProperty(bool),
    VolumeProperty(f64),

    // visible, uid, name, type, value, monitor type
    VarData(bool, String, String, SB3Type, String, MonitorType),

    // visible, values
    ListData(bool, Vec<String>),

    // uid, name
    BroadCastProperty(String, String),

    // name, path to costume, format, bitmap resolution, x center, y center
    Costume(String, String, String, f64, f64, f64),

    // name, path to sound, format, sampling rate, sample count
    Sound(String, String, String, f64, f64),

    // data, x, y
    Block(Vec<Node>, f64, f64),
    Uid(String),
    Parent(String),
    Next(String),
    In(String, Box<Node>),
    Field(String, Vec<Node>),

    NullData,
    PrototypeData(String),
    BlockPtrData(String),
    SubstackData(String),
    DoubleData(String),
    PosDoubleData(String),
    PosIntData(String),
    IntData(String),
    AngleData(String),
    ColorData(String),
    StringData(String),
    BroadcastData(String, String),
    VarInData(String, String),
    ListInData(String, String),
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
    BlockInBroadcastName(String, String),
    BlockInVarVal(String, String),
    BlockInListVal(String, String, Vec<String>),
    BlockFieldKey,
    BlockFieldVal,
    BlockMut,
    BlockShadow,
    BlockTopLevel,
}

fn parse_change_state(
    state: &mut ParsingState,
    checked_state: ParsingState,
    result_state: ParsingState,
    error: &str,
) {
    if *state == checked_state {
        *state = result_state;
    } else {
        throw_error(error.to_string());
    }
}

struct SpriteReferences<'a> {
    blocks: &'a mut Vec<Node>,
    is_stage: &'a mut bool,
    volume: &'a mut f64,
    layer: &'a mut i64,
}

struct BlockReferences<'a> {
    data: &'a mut Vec<Node>,
    x: &'a mut f64,
    y: &'a mut f64,
}

fn parse_token(token: &Token, root: &mut Node, state: &mut ParsingState) -> Node {
    let Node::Root(ref mut root_data) = *root else {
        unreachable!()
    };

    let mut sprite_data: Option<SpriteReferences> = None;
    let mut block_data: Option<BlockReferences> = None;

    match token {
        Token::SpriteHeader(name) => {
            root_data.push(Node::Sprite(name.clone(), vec![], false, 100.0, 1));
            if let Node::Sprite(_, blocks, is_stage, volume, layer) = root_data.last_mut().unwrap()
            {
                sprite_data = Some(SpriteReferences {
                    blocks,
                    is_stage,
                    volume,
                    layer,
                })
            } else {
                unreachable!()
            }

            parse_change_state(
                state,
                ParsingState::Root,
                ParsingState::Sprite,
                "Cannot use sprite header outside of global scope",
            );
        }
        Token::Block => {
            if let Some(unwrapped) = sprite_data {
                if *state == ParsingState::Sprite {
                    *state = ParsingState::Block;
                    unwrapped.blocks.push(Node::Block(vec![], 0.0, 0.0));
                } else {
                    throw_error("Cannot use block outside of sprite scope".to_string());
                }
            }
        }
        Token::Uid => {
            parse_change_state(
                state,
                ParsingState::Block,
                ParsingState::BlockUid,
                "Cannot use uid outside of block scope",
            );
        }
        Token::Opcode => {
            parse_change_state(
                state,
                ParsingState::Block,
                ParsingState::BlockOpcode,
                "Cannot use opcode outside of block scope",
            );
        }
        Token::Parent => {
            parse_change_state(
                state,
                ParsingState::Block,
                ParsingState::BlockParent,
                "Cannot use parent outside of block scope",
            );
        }
        Token::Next => {
            parse_change_state(
                state,
                ParsingState::Block,
                ParsingState::BlockNext,
                "Cannot use next outside of block scope",
            );
        }
        Token::In => {
            parse_change_state(
                state,
                ParsingState::Block,
                ParsingState::BlockInKey,
                "Cannot use in outside of block scope",
            );
        }
        Token::Field => {
            parse_change_state(
                state,
                ParsingState::Block,
                ParsingState::BlockFieldKey,
                "Cannot use field outside of block scope",
            );
        }
        Token::Mut => {
            parse_change_state(
                state,
                ParsingState::Block,
                ParsingState::BlockMut,
                "Cannot use mut outside of block scope",
            );
        }
        Token::Shadow => {
            parse_change_state(
                state,
                ParsingState::Block,
                ParsingState::BlockShadow,
                "Cannot use shadow outside of block scope",
            );
        }
        Token::TopLevel => {
            parse_change_state(
                state,
                ParsingState::Block,
                ParsingState::BlockTopLevel,
                "Cannot use top_level outside of block scope",
            );
        }
        Token::StringLit(data) => {
            if let ParsingState::BlockInVal(name, sb3_type) = state.clone() {
                if [SB3Type::Broadcast, SB3Type::Var, SB3Type::List].contains(&sb3_type) {
                    *state = ParsingState::BlockInBroadcastName(name, data.clone());
                } else {
                    if let Some(ref mut unwrapped) = block_data {
                        unwrapped.data.push(match sb3_type {
                            SB3Type::Prototype => Node::PrototypeData(data.clone()),
                            SB3Type::BlockPtr => Node::BlockPtrData(data.clone()),
                            SB3Type::Substack => Node::SubstackData(data.clone()),
                            SB3Type::Double => Node::DoubleData(data.clone()),
                            SB3Type::PosDouble => Node::PosDoubleData(data.clone()),
                            SB3Type::PosInt => Node::PosIntData(data.clone()),
                            SB3Type::Int => Node::IntData(data.clone()),
                            SB3Type::Angle => Node::AngleData(data.clone()),
                            SB3Type::Color => Node::ColorData(data.clone()),
                            SB3Type::String => Node::StringData(data.clone()),
                            _ => maybe_unreachable!(),
                        })
                    }
                }
            }
        }
        _ => {
            if let ParsingState::BlockInType(name) = state.clone() {
                *state = ParsingState::BlockInVal(
                    name,
                    match token {
                        Token::PrototypeAnnotation => SB3Type::Prototype,
                        Token::BlockPtrAnnotation => SB3Type::BlockPtr,
                        Token::SubstackAnnotation => SB3Type::Substack,
                        Token::DoubleAnnotation => SB3Type::Double,
                        Token::PosDoubleAnnotation => SB3Type::PosDouble,
                        Token::PosIntAnnotation => SB3Type::PosInt,
                        Token::IntAnnotation => SB3Type::Int,
                        Token::AngleAnnotation => SB3Type::Angle,
                        Token::ColorAnnotation => SB3Type::Color,
                        Token::StringAnnotation => SB3Type::String,
                        Token::BroadcastAnnotation => SB3Type::Broadcast,
                        Token::VarAnnotation => SB3Type::Var,
                        Token::ListAnnotation => SB3Type::List,
                        _ => throw_error("Must use type as second argument of input".to_string()),
                    },
                )
            }
        }
    }

    todo!("Finish parsing for single token")
}
