// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (C) 2025 KyteKode

use std::collections::HashMap;
use std::mem::take;

use super::errors::*;
use super::lexer::{self, Token, get_token_name};

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
    Opcode(String),
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
    BlockInDataVal(String, SB3Type, String),
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
        throw_error(error);
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

struct ParseData<'a> {
    root_data: &'a mut Vec<Node>,
    sprite_data: Option<SpriteReferences<'a>>,
    block_data: Option<BlockReferences<'a>>,
}

fn parse_token<'a>(token: &mut Token, data: &'a mut ParseData<'a>, state: &mut ParsingState) -> Node {
    let mut root_data = &mut data.root_data;
    let mut sprite_data = &mut data.sprite_data;
    let mut block_data = &mut data.block_data;

    match state {
        ParsingState::Root => {
            if let Token::SpriteHeader(name) = token {
                root_data.push(Node::Sprite(take(name), vec![], false, 100.0, 1));
                if let Node::Sprite(_, blocks, is_stage, volume, layer) =
                    root_data.last_mut().unwrap()
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
                if token == &Token::Block {
                    unwrapped.blocks.push(Node::Block(vec![], 0.0, 0.0));
                    if let Node::Block(data, x, y) = unwrapped.blocks.last_mut().unwrap() {
                        *block_data = Some(BlockReferences { data, x, y })
                    }
                } else {
                    todo!("Handle other sprite level tokens (var, list, costume, sound)")
                }
            }
        }
        ParsingState::Block => {
            if let Some(unwrapped) = block_data {
                *state = match token {
                    Token::Uid => ParsingState::BlockUid,
                    Token::Opcode => ParsingState::BlockOpcode,
                    Token::Parent => ParsingState::BlockParent,
                    Token::Next => ParsingState::BlockNext,
                    Token::In => ParsingState::BlockInKey,
                    Token::Field => ParsingState::BlockFieldVal,
                    Token::Mut => ParsingState::BlockMut,
                    Token::TopLevel => ParsingState::BlockTopLevel,
                    _ => throw_error(format!(
                        "Unexpected {} in block scope",
                        get_token_name(token)
                    )),
                }
            }
        }
        ParsingState::BlockUid => {
            if let Some(unwrapped) = block_data {
                if let Token::StringLit(mut strdata) = take(token) {
                    unwrapped.data.push(Node::Uid(take(&mut strdata)));
                } else {
                    throw_error(format!(
                        "Expected string literal, got {}",
                        get_token_name(token)
                    ))
                }
                *state = ParsingState::Block;
            }
        }
        ParsingState::BlockOpcode => {
            if let Some(unwrapped) = block_data {
                if let Token::StringLit(mut strdata) = take(token) {
                    unwrapped.data.push(Node::Opcode(take(&mut strdata)));
                } else {
                    throw_error(format!(
                        "Expected string literal, got {}",
                        get_token_name(token)
                    ))
                }
                *state = ParsingState::Block;
            }
        }
        ParsingState::BlockInKey => {
            if let Some(unwrapped) = block_data {
                if let Token::StringLit(mut strdata) = take(token) {
                    *state = ParsingState::BlockInType(take(&mut strdata));
                } else {
                    throw_error(format!(
                        "Expected string literal, got {}",
                        get_token_name(token)
                    ))
                }
            }
        }
        ParsingState::BlockInType(uid) => {
            if let Some(unwrapped) = block_data {
                *state = ParsingState::BlockInVal(take(uid), match token {
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
                    ))
                });
            }
        }
        ParsingState::BlockInVal(uid, sb3_type) => {
            if let Some(unwrapped) = block_data {
                if let Token::StringLit(mut strdata) = take(token) {
                    if [SB3Type::Broadcast, SB3Type::Var, SB3Type::List].contains(sb3_type) {
                        *state = ParsingState::BlockInDataVal(take(uid), *sb3_type, take(&mut strdata));
                    } else {
                        unwrapped.data.push(Node::In(take(uid), Box::new(match sb3_type {
                            SB3Type::Prototype => Node::PrototypeData(take(&mut strdata)),
                            SB3Type::BlockPtr => Node::BlockPtrData(take(&mut strdata)),
                            SB3Type::Substack => Node::SubstackData(take(&mut strdata)),
                            SB3Type::Double => Node::DoubleData(take(&mut strdata)),
                            SB3Type::Int => Node::IntData(take(&mut strdata)),
                            SB3Type::PosInt => Node::PosIntData(take(&mut strdata)),
                            SB3Type::PosDouble => Node::PosDoubleData(take(&mut strdata)),
                            SB3Type::Angle => Node::AngleData(take(&mut strdata)),
                            SB3Type::Color => Node::ColorData(take(&mut strdata)),
                            SB3Type::String => Node::StringData(take(&mut strdata)),
                            _ => unreachable!(),
                        })));
                    }
                } else {
                    throw_error(format!(
                        "Expected string literal, got {}",
                        get_token_name(token)
                    ))
                }
                *state = ParsingState::Block;
            }
        }
        _ => unimplemented!(),
    }
    todo!("Finish parsing for single token")
}

pub fn parse() {
    todo!("Actually parse")
}
