use super::lexer::Token;

use super::errors::*;

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

pub fn parse() {
    todo!("Actually parse")
}
