use std::collections::HashMap;

use super::errors::*;
use super::lexer::Token;
use super::*;

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
    BlockInListVal(String, String),
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

struct ParseData<'a> {
    root_data: &'a mut Vec<Node>,
    sprite_data: Option<SpriteReferences<'a>>,
    block_data: Option<BlockReferences<'a>>,
}

fn parse_token<'a>(token: &Token, data: &'a mut ParseData<'a>, state: &mut ParsingState) -> Node {
    let mut root_data = &mut data.root_data;
    let mut sprite_data = &mut data.sprite_data;
    let mut block_data = &mut data.block_data;

    match state {
        ParsingState::Root => {
            if let Token::SpriteHeader(name) = token {
                root_data.push(Node::Sprite(name.clone(), vec![], false, 100.0, 1));
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
                //throw_error("Error message") todo!("Add 'Unexpected [token name]' error message") }
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
                        lexer::get_token_name(token)
                    )),
                }
            }
        }
        _ => unimplemented!(),
    }

    /*match token {
        Token::StringLit(data) => match state {
            ParsingState::BlockInVal(name, sb3_type) => {
                if [SB3Type::Broadcast, SB3Type::Var, SB3Type::List].contains(&sb3_type) {
                    *state = match sb3_type {
                        SB3Type::Broadcast => {
                            ParsingState::BlockInBroadcastName(name.clone(), data.clone())
                        }
                        SB3Type::Var => ParsingState::BlockInVarVal(name.clone(), data.clone()),
                        SB3Type::List => ParsingState::BlockInListVal(name.clone(), data.clone()),
                        _ => unreachable!(),
                    };
                } else {
                    if let Some(unwrapped) = block_data {
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
                            _ => unreachable!(),
                        });
                    }
                }
            }
            ParsingState::BlockInBroadcastName(in_name, uid) => {
                if let Some(unwrapped) = block_data {
                    unwrapped.data.push(Node::In(
                        in_name.clone(),
                        Box::new(Node::BroadcastData(uid.clone(), data.clone())),
                    ));
                }
                *state = ParsingState::Block;
            }
            ParsingState::BlockInVarVal(in_name, uid) => {
                if let Some(unwrapped) = block_data {
                    unwrapped.data.push(Node::In(
                        in_name.clone(),
                        Box::new(Node::VarInData(uid.clone(), data.clone())),
                    ))
                }
            }
            ParsingState::BlockInListVal(in_name, uid) => {
                if let Some(unwrapped) = block_data {
                    unwrapped.data.push(Node::In(
                        in_name.clone(),
                        Box::new(Node::ListInData(uid.clone(), data.clone())),
                    ))
                }
            }
            _ => throw_error("String literal used in invalid context".to_string()),
        },
        _ => {
            if [
                Token::PrototypeAnnotation,
                Token::BlockPtrAnnotation,
                Token::SubstackAnnotation,
                Token::DoubleAnnotation,
                Token::PosDoubleAnnotation,
                Token::PosIntAnnotation,
                Token::IntAnnotation,
                Token::AngleAnnotation,
                Token::ColorAnnotation,
                Token::StringAnnotation,
                Token::BroadcastAnnotation,
                Token::VarAnnotation,
                Token::ListAnnotation,
            ]
            .contains(token)
            {
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
                            _ => unreachable!(),
                        },
                    )
                } else {
                    throw_error(
                        "Type annotations must be used as second argument of input".to_string(),
                    )
                }
            } else {
                throw_error("Must use type as second argument of input".to_string());
            }
        }
    }*/

    todo!("Finish parsing for single token")
}

pub fn parse() {
    todo!("Actually parse")
}
