// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (C) 2025 KyteKode

use std::alloc::LayoutErr;
use std::collections::HashMap;
use std::mem::take;

use super::errors::*;
use super::lexer::{self, get_token_name, Token};

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

    Sprite {
        name: String,
        blocks: Vec<Node>,
        stage: bool,
        volume: f64,
        layer: i64,
    },

    NameProperty(String),
    IsStageProperty(bool),
    VolumeProperty(f64),

    Var {
        visible: bool,
        uid: String,
        name: String,
        val_type: SB3Type,
        value: String,
        monitor_type: MonitorType,
    },

    List {
        visible: bool,
        data: Vec<String>,
    },

    Broadcast {
        uid: String,
        name: String,
    },

    Costume(Vec<Node>),
    CostumeName(String),
    CostumePath(String),
    CostumeFormat(String),
    BitmapRes(f64),
    XCenter(f64),
    YCenter(f64),

    Sound {
        name: String,
        path: String,
        format: String,
        sample_rate: f64,
        sample_count: f64,
    },

    Block(Vec<Node>, f64, f64),
    Uid(String),
    Opcode(String),
    Parent(String),
    Next(String),
    In(String, Box<Node>),
    Field(String, Vec<String>),
    Mut(String, String),
    TopLevel(bool),

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
    BlockInDataVal(String, SB3Type, String),
    BlockFieldKey,
    BlockFieldVal(String, Vec<String>),
    BlockMutKey,
    BlockMutVal(String),
    BlockShadow,
    BlockTopLevel,
    BlockXPos,
    BlockYPos,

    Costume,
    CostumeName,
    CostumePath,
    CostumeFormat,
    CostumeBitmapRes,
    CostumeCenterX,
    CostumeCenterY,
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

struct ParseData<'a> {
    root_data: &'a mut Vec<Node>,
    sprite_data: Option<SpriteReferences<'a>>,
    block_data: Option<BlockReferences<'a>>,
    costume_data: Option<&'a mut Vec<Node>>,
}

fn handle_string_lit(token: &mut Token) -> String {
    if let Token::StringLit(mut str_data) = take(token) {
        return str_data;
    } else {
        throw_error(format!(
            "Expected string literal, got {}",
            get_token_name(token)
        ))
    }
}

fn handle_bool_lit(token: &mut Token) -> bool {
    if let Token::BoolLit(mut bool_data) = take(token) {
        return bool_data;
    } else {
        throw_error(format!(
            "Expected boolean literal, got {}",
            get_token_name(token)
        ))
    }
}

fn handle_num_lit(token: &mut Token) -> f64 {
    if let Token::NumLit(mut num_data) = take(token) {
        let parsed = num_data.parse::<f64>();
        match parsed {
            Ok(result) => return result,
            Err(_) => throw_error(format!("Could not parse {} as float", num_data)),
        }
    } else {
        throw_error(format!(
            "Expected number literal, got {}",
            get_token_name(token)
        ))
    }
}

fn parse_token<'a>(
    token: &mut Token,
    data: &'a mut ParseData<'a>,
    state: &mut ParsingState,
) -> Node {
    let mut root_data = &mut data.root_data;
    let mut sprite_data = &mut data.sprite_data;
    let mut block_data = &mut data.block_data;
    let mut costume_data = &mut data.costume_data;

    match state {
        ParsingState::Root => {
            if let Token::SpriteHeader(name) = token {
                root_data.push(Node::Sprite {
                    name: take(name),
                    blocks: vec![],
                    stage: false,
                    volume: 100.0,
                    layer: 1,
                });
                if let Node::Sprite {
                    name: _,
                    blocks: blocks,
                    stage: is_stage,
                    volume: volume,
                    layer: layer,
                } = root_data.last_mut().unwrap()
                {
                    *sprite_data = Some(SpriteReferences {
                        blocks,
                        is_stage,
                        volume,
                        layer,
                    })
                }
                *state = ParsingState::Sprite
            } else if token == &Token::SpriteEnd {
                *state = ParsingState::Root;
                *sprite_data = None;
                *block_data = None;
            } else {
                throw_error(format!(
                    "Unexpected {} in top level scope",
                    get_token_name(token)
                ));
            }
        }
        ParsingState::Sprite => {
            if let Some(unwrapped) = sprite_data {
                match token {
                    Token::Block => {
                        unwrapped.blocks.push(Node::Block(vec![], 0.0, 0.0));
                        if let Node::Block(data, x, y) = unwrapped.blocks.last_mut().unwrap() {
                            *block_data = Some(BlockReferences { data, x, y })
                        }
                        *state = ParsingState::Block;
                    }
                    Token::Costume => {
                        unwrapped.blocks.push(Node::Costume(vec![]));
                        if let Node::Costume(data) = unwrapped.blocks.last_mut().unwrap() {
                            *costume_data = Some(data)
                        }
                        *state = ParsingState::Costume;
                    }
                    _ => todo!("Handle other sprite level tokens (var, list, costume, sound)"),
                }
            } else {
                maybe_unreachable!();
            }
        }
        ParsingState::Block => {
            *state = match token {
                Token::Uid => ParsingState::BlockUid,
                Token::Opcode => ParsingState::BlockOpcode,
                Token::Parent => ParsingState::BlockParent,
                Token::Next => ParsingState::BlockNext,
                Token::In => ParsingState::BlockInKey,
                Token::Field => ParsingState::BlockFieldKey,
                Token::Mut => ParsingState::BlockMutKey,
                Token::TopLevel => ParsingState::BlockTopLevel,
                Token::XPos => ParsingState::BlockXPos,
                Token::YPos => ParsingState::BlockYPos,
                _ => throw_error(format!(
                    "Unexpected {} in block scope",
                    get_token_name(token)
                )),
            }
        }
        ParsingState::BlockUid => {
            if let Some(unwrapped) = block_data {
                unwrapped.data.push(Node::Uid(handle_string_lit(token)));
                *state = ParsingState::Block;
            } else {
                maybe_unreachable!();
            }
        }
        ParsingState::BlockOpcode => {
            if let Some(unwrapped) = block_data {
                unwrapped.data.push(Node::Opcode(handle_string_lit(token)));
                *state = ParsingState::Block;
            } else {
                maybe_unreachable!();
            }
        }
        ParsingState::BlockInKey => *state = ParsingState::BlockInType(handle_string_lit(token)),
        ParsingState::BlockInType(key) => {
            let sb3_type = match token {
                Token::PrototypeAnnotation => SB3Type::Prototype,
                Token::BlockPtrAnnotation => SB3Type::BlockPtr,
                Token::SubstackAnnotation => SB3Type::Substack,
                Token::DoubleAnnotation => SB3Type::Double,
                Token::IntAnnotation => SB3Type::Int,
                Token::PosIntAnnotation => SB3Type::PosInt,
                Token::PosDoubleAnnotation => SB3Type::PosDouble,
                Token::AngleAnnotation => SB3Type::Angle,
                Token::ColorAnnotation => SB3Type::Color,
                Token::StringAnnotation => SB3Type::String,
                Token::BroadcastAnnotation => SB3Type::Broadcast,
                Token::VarAnnotation => SB3Type::Var,
                Token::ListAnnotation => SB3Type::List,
                _ => throw_error(format!(
                    "Expected type annotation, got {}",
                    get_token_name(token)
                )),
            };
            *state = ParsingState::BlockInVal(take(key), sb3_type);
        }
        ParsingState::BlockInVal(key, sb3_type) => {
            if let Some(unwrapped) = block_data {
                let str_data = handle_string_lit(token);
                if [SB3Type::Broadcast, SB3Type::Var, SB3Type::List].contains(sb3_type) {
                    *state = ParsingState::BlockInDataVal(take(key), *sb3_type, str_data);
                } else {
                    let data = match sb3_type {
                        SB3Type::Prototype => Node::PrototypeData,
                        SB3Type::BlockPtr => Node::BlockPtrData,
                        SB3Type::Substack => Node::SubstackData,
                        SB3Type::Double => Node::DoubleData,
                        SB3Type::Int => Node::IntData,
                        SB3Type::PosInt => Node::PosIntData,
                        SB3Type::PosDouble => Node::PosDoubleData,
                        SB3Type::Angle => Node::AngleData,
                        SB3Type::Color => Node::ColorData,
                        SB3Type::String => Node::StringData,
                        _ => unreachable!(),
                    }(str_data);

                    unwrapped.data.push(Node::In(take(key), Box::new(data)));
                    *state = ParsingState::Block;
                }
            } else {
                maybe_unreachable!();
            }
        }
        ParsingState::BlockInDataVal(key, sb3_type, uid) => {
            if let Some(unwrapped) = block_data {
                let str_data = handle_string_lit(token);
                let data = match sb3_type {
                    SB3Type::Broadcast => Node::BroadcastData,
                    SB3Type::Var => Node::VarInData,
                    SB3Type::List => Node::ListInData,
                    _ => unreachable!(),
                }(take(uid), str_data);

                unwrapped.data.push(Node::In(take(key), Box::new(data)));
                *state = ParsingState::Block;
            } else {
                maybe_unreachable!();
            }
        }
        ParsingState::BlockFieldKey => {
            if let Token::StringLit(mut strdata) = take(token) {
                *state = ParsingState::BlockFieldVal(strdata, vec![]);
            } else {
                throw_error(format!(
                    "Expected string literal, got {}",
                    get_token_name(token)
                ))
            }
        }
        ParsingState::BlockFieldVal(key, data) => {
            if let Some(unwrapped) = block_data {
                if let Token::StringLit(mut strdata) = take(token) {
                    data.push(strdata);
                } else if *token == Token::Semicolon {
                    unwrapped.data.push(Node::Field(take(key), take(data)))
                } else {
                    throw_error(format!(
                        "Expected string literal or semicolon, got {}",
                        get_token_name(token)
                    ))
                }
                *state = ParsingState::Block;
            }
        }
        ParsingState::BlockMutKey => *state = ParsingState::BlockMutVal(handle_string_lit(token)),
        ParsingState::BlockMutVal(key) => {
            if let Some(unwrapped) = block_data {
                let str_data = handle_string_lit(token);
                unwrapped.data.push(Node::Mut(take(key), str_data));
            }
        }
        ParsingState::BlockTopLevel => {
            if let Some(unwrapped) = block_data {
                unwrapped.data.push(Node::TopLevel(handle_bool_lit(token)));
                *state = ParsingState::Block;
            }
        }
        ParsingState::BlockXPos => {
            if let Some(unwrapped) = block_data {
                *unwrapped.x = handle_num_lit(token);
                *state = ParsingState::Block;
            }
        }
        ParsingState::BlockYPos => {
            if let Some(unwrapped) = block_data {
                *unwrapped.y = handle_num_lit(token);
                *state = ParsingState::Block;
            }
        }
        ParsingState::Costume => {
            *state = match token {
                Token::Name => ParsingState::CostumeName,
                Token::Path => ParsingState::CostumePath,
                Token::Format => ParsingState::CostumeFormat,
                Token::BitmapRes => ParsingState::CostumeBitmapRes,
                Token::CenterX => ParsingState::CostumeCenterX,
                Token::CenterY => ParsingState::CostumeCenterY,
                _ => throw_error(format!(
                    "Unexpected {} in costume scope",
                    get_token_name(token)
                )),
            }
        }
        ParsingState::CostumeName => {
            if let Some(unwrapped) = costume_data {
                unwrapped.push(Node::CostumeName(handle_string_lit(token)));
                *state = ParsingState::Costume;
            }
        }
        ParsingState::CostumePath => {
            if let Some(unwrapped) = costume_data {
                unwrapped.push(Node::CostumePath(handle_string_lit(token)));
                *state = ParsingState::Costume;
            }
        }
        ParsingState::CostumeFormat => {
            if let Some(unwrapped) = costume_data {
                unwrapped.push(Node::CostumeFormat(handle_string_lit(token)));
                *state = ParsingState::Costume;
            }
        }
        ParsingState::CostumeBitmapRes => {
            if let Some(unwrapped) = costume_data {
                unwrapped.push(Node::BitmapRes(handle_num_lit(token)));
                *state = ParsingState::Costume;
            }
        }
        ParsingState::CostumeCenterX => {
            if let Some(unwrapped) = costume_data {
                unwrapped.push(Node::XCenter(handle_num_lit(token)));
                *state = ParsingState::Costume;
            }
        }
        ParsingState::CostumeCenterY => {
            if let Some(unwrapped) = costume_data {
                unwrapped.push(Node::YCenter(handle_num_lit(token)));
                *state = ParsingState::Costume;
            }
        }
        _ => unimplemented!(),
    }
    todo!("Finish parsing for single token")
}

pub fn parse() {
    todo!("Actually parse")
}
