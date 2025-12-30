// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (C) 2025 KyteKode

use super::errors::*;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
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
fn lex_string(s_token: String) -> Token {
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

                    #[derive(PartialEq, Eq)]
                    enum EscapeState {
                        Normal,
                        Escape,
                    }

                    let mut state = EscapeState::Normal;

                    let mut final_result = String::new();
                    for c in result.chars() {
                        if c == '\\' {
                            if state == EscapeState::Normal {
                                state = EscapeState::Escape;
                            } else {
                                state = EscapeState::Normal;
                                final_result.push('\\');
                            }
                        } else {
                            final_result.push(c);
                            state = EscapeState::Normal;
                        }
                    }
                    return Token::StringLit(final_result.to_string());
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

pub fn lex(basm_code: &str) -> Vec<Token> {
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
                tokens.push(lex_string(s_token));
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lex_block_tokens() {
        let token_pairs = [
            ("block", Token::Block),
            ("uid", Token::Uid),
            ("opcode", Token::Opcode),
            ("parent", Token::Parent),
            ("next", Token::Next),
            ("in", Token::In),
            ("field", Token::Field),
            ("mut", Token::Mut),
            ("shadow", Token::Shadow),
            ("top_level", Token::TopLevel),
        ];
        for (input, expected) in token_pairs.iter() {
            let token = lex_string(input.to_string());
            assert_eq!(&token, expected);
        }
    }

    #[test]
    fn lex_strings() {
        let pairs = [
            ("\"Hello, world!\"", "Hello, world!"),
            ("\"\"", ""),
            ("\"12345\"", "12345"),
            ("\"Special !@#$%^&*()\"", "Special !@#$%^&*()"),
            (
                "\"Backslash \\\\ Quotation \\\"\"",
                "Backslash \\ Quotation \"",
            ),
        ];
        for (input, expected) in pairs.iter() {
            let token = lex_string(input.to_string());
            assert_eq!(&token, &Token::StringLit(expected.to_string()))
        }
    }
}
