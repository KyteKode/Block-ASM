use super::error::{throw_errors, BasmError};

use std::collections::HashSet;

#[derive(Debug, Clone, PartialEq, Eq)]
enum ScanState {
    Normal,
    Literal(char, u32)
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Symbol {
    pub data: String,
    pub line: u32
}

// Splits the source into symbols (like `block`, or `costume`, or `[Stage]`)
pub fn scan(input_source: impl Into<String>) -> Vec<Symbol> {
    let mut source = input_source.into(); // Only mutable so that space can be added to the end
    source.push(' ');

    let mut symbols = Vec::new();
    let mut symbol = Symbol { data: String::new(), line: 1 };
    let mut state = ScanState::Normal;
    let mut escaped = false;

    let mut line: u32 = 1;
    let mut errors: Vec<BasmError> = Vec::new();

    for c in source.chars() {
        match state {
            ScanState::Normal => {
                // Adds the symbol the vector of symbols if encountering whitespace
                if " \t\n\r".contains(c) {
                    // In separate if statement so that it doesn't push the whitespace to the symbol
                    // even if the symbol is empty
                    if !symbol.data.trim().is_empty() {
                        symbols.push(symbol.clone());
                        symbol = Symbol { data: String::new(), line };
                    }

                    if c == '\n' {
                        line += 1;
                    }

                    continue;
                }

                // Checks if the symbol is starting with a character that starts a literal
                // Not for number literals and boolean literals, they do not have closing delimiters
                if symbol.data.is_empty() {
                    state = match c {
                        '"' => ScanState::Literal('"', line),
                        '[' => ScanState::Literal(']', line),
                        '{' => ScanState::Literal('}', line),
                        _ => ScanState::Normal
                    };
                    escaped = false;
                }

                symbol.data.push(c);
            },
            ScanState::Literal(closing_delimiter, _) => {
                if c == '\n' {
                    errors.push(match closing_delimiter {
                        '"' => BasmError::UnclosedStringLiteral { line },
                        '[' => BasmError::UnclosedTargetHeader { line },
                        '{' => BasmError::UnclosedMonitorHeader { line },
                        _ => unreachable!()
                    });
                    state = ScanState::Normal;
                    symbol = Symbol { data: String::new(), line };
                } else if escaped {
                    symbol.data.push(c);
                    escaped = false;
                } else {
                    // Escapes the next character
                    if c == '\\' {
                        escaped = true;
                        continue;
                    }

                    // Ends literal if it reaches the closing delimiter
                    if c == closing_delimiter {
                        state = ScanState::Normal;
                    }

                    symbol.data.push(c);
                }
            }
        }
    }

    if !errors.is_empty() {
        throw_errors(errors);
    }

    symbols
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Token {
    pub token_type: TokenType,
    pub data: String,
    pub line: u32
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum TokenType {
    Keyword,
    Literal,
    Punctuator,
    
    #[default]
    Placeholder
}

fn lex_symbol(symbol: Symbol) -> Result<Token, BasmError> {
    // massive list of keywords
    let keywords = HashSet::from([
        // Metadata
        "sem_ver", "vm", "agent",

        // Targets
        "is_stage", "costume_num", "layer", "volume",

        // Targets (Stage)
        "tempo", "video_state", "video_transparency", "tts_language",

        // Targets (Sprite)
        "visible", "x_pos", "y_pos", "size", "direction", "rotation_style",

        // Blocks
        "block", "uid", "opcode", "parent", "next", "input",
        "field", "mutation", "shadow", "top_level", // "x_pos", "y_pos",

        // Blocks (Type Annotations)
        "prototype", "block_ptr", "substack", "double", "pos_double", "pos_int", "int", "angle",
        "color", "string", "broadcast", "variable", "list",

        // Costumes
        "name", "path", "format", "bitmap_res", "center_x", "center_y",

        // Sounds
        "rate", "samples", //"name", "path", "format",

        // Variables
        "value", "is_cloud", // "variable", "name",

        // Lists
        "item", // "name",

        // Broadcasts
        // "uid", "name",

        // Monitors
        "mode", "param", "sprite_name", "width", "height", "slider_min", "slider_max",
        "is_discrete", // "opcode", "uid", "value", "x_pos", "y_pos", "visible",

        // Misc
        "null",
    ]);

    // Checks if the symbol is a keyword
    if keywords.contains(symbol.data.as_str()) {
        return Ok(Token {
            token_type: TokenType::Keyword,
            data: symbol.data,
            line: symbol.line
        });
    }

    let mut chars = symbol.data.chars();

    // Both assumes there is at least one character in the symbol
    let first_char = chars.next().unwrap();
    let last_char = chars.last().unwrap_or(first_char);

    if (first_char == '"' && last_char == '"') || // Is string literal?
        (first_char == '[' && last_char == ']') || // Is target header?
        (first_char == '{' && last_char == '}') || // Is monitor header?
        (symbol.data == "true" || symbol.data == "false") || // Is bool literal?
        symbol.data == "null" || // Is null literal?
        symbol.data.parse::<f64>().is_ok() // Is number literal?
    {
        return Ok(Token {
            token_type: TokenType::Literal,
            data: symbol.data,
            line: symbol.line
        });
    }

    // Checks if the symbol is a punctuator
    if symbol.data == ";" || symbol.data == "end" || symbol.data == "!end" {
        return Ok(Token {
            token_type: TokenType::Punctuator,
            data: symbol.data,
            line: symbol.line
        });
    }

    Err(BasmError::UnknownSymbol { line: symbol.line, data: symbol.data })
}

pub fn lex(source: impl Into<String>) -> Vec<Token> {
    let symbols = scan(source);

    let mut tokens = Vec::new();
    let mut errors = Vec::new();

    for symbol in symbols {
        let result = lex_symbol(symbol);
        match result {
            Ok(token) => tokens.push(token),
            Err(e) => errors.push(e)
        }
    }

    if !errors.is_empty() {
        throw_errors(errors);
    }
    
    tokens
}





#[cfg(test)]
mod tests {
    use super::*;

    fn scanning_template(inputs: Vec<&str>, expected: Vec<Vec<&str>>) {
        let pairs = inputs.iter().zip(expected.iter());

        for (input, expected) in pairs {
            let result = scan(*input);

            // Converts expected from a &Vec<&str> to a Vec<Symbol>
            let owned_expected: Vec<String> = expected.iter()
                .map(|x| String::from(*x))
                .collect();

            for (expected_data, result_symbol) in owned_expected.iter().zip(result.iter()) {
                assert_eq!(*expected_data, *result_symbol.data);
            }
        }
    }

    #[test]
    fn scanning() {
        let inputs = vec![
            "a",
            "b cd",
            " e fg",
            "hij kl ",
            " mn op ",
            "qrs tuv",
            " w  x yz"
        ];

        let expected = vec![
            vec!["a"],
            vec!["b", "cd"],
            vec!["e", "fg"],
            vec!["hij", "kl"],
            vec!["mn", "op"],
            vec!["qrs", "tuv"],
            vec!["w", "x", "yz"]
        ];

        scanning_template(inputs, expected);
    }

    #[test]
    fn string_scanning() {
        let inputs = vec![
            "\"a\"",
            "b \"cd\"",
            " \"e fg\"",
            "\"hij\" \"kl\" ",
            " \"mn\" o \"p\" ",
            "\"qr\" s \"tu\" v",
            " \"w  \" x \"y\" z "
        ];

        let expected = vec![
            vec!["\"a\""],
            vec!["b", "\"cd\""],
            vec!["\"e fg\""],
            vec!["\"hij\"", "\"kl\""],
            vec!["\"mn\"", "o", "\"p\""],
            vec!["\"qr\"", "s", "\"tu\"", "v"],
            vec!["\"w  \"", "x", "\"y\"", "z"]
        ];

        scanning_template(inputs, expected);
    }

    #[test]
    fn target_scanning() {
        let inputs = vec![
            "[a]",
            "b [cd]",
            " [e fg]",
            "[hij] [kl] ",
            " [mn] o [p] ",
            "[qr] s [tu] v",
            " [w  ] x [y] z "
        ];

        let expected = vec![
            vec!["[a]"],
            vec!["b", "[cd]"],
            vec!["[e fg]"],
            vec!["[hij]", "[kl]"],
            vec!["[mn]", "o", "[p]"],
            vec!["[qr]", "s", "[tu]", "v"],
            vec!["[w  ]", "x", "[y]", "z"]
        ];

        scanning_template(inputs, expected);
    }

    #[test]
    fn monitor_scanning() {
        let inputs = vec![
            "{a}",
            "b {cd}",
            " {e fg}",
            "{hij} {kl} ",
            " {mn} o {p} ",
            "{qr} s {tu} v",
            " {w  } x {y} z "
        ];

        let expected = vec![
            vec!["{a}"],
            vec!["b", "{cd}"],
            vec!["{e fg}"],
            vec!["{hij}", "{kl}"],
            vec!["{mn}", "o", "{p}"],
            vec!["{qr}", "s", "{tu}", "v"],
            vec!["{w  }", "x", "{y}", "z"]
        ];

        scanning_template(inputs, expected);
    }

    fn lexing_template(inputs: Vec<&str>, expected_type: TokenType) {
        for input in inputs {
            let lexed = lex_symbol(Symbol { data: input.to_owned(), line: 0 });
            assert_eq!(lexed, Ok(Token {
                token_type: expected_type.clone(),
                data: input.to_owned(),
                line: 0
            }));
        }
    }

    #[test]
    fn keyword_lexing() {
        let inputs = vec!["block", "uid", "opcode", "parent", "next", "input", "field", "mutation", "shadow", "top_level"];
        lexing_template(inputs, TokenType::Keyword);
    }

    #[test]
    fn literal_lexing() {
        let inputs = vec![
            "\"string\"",
            "\" string with \\\" escape\"",
            "[target]",
            "[ target with \\[ escape]",
            "{monitor}",
            "{ monitor with \\{ escape}",
            "true",
            "false",
            "12345",
            "6.789",
            "1.357e9"
        ];
        lexing_template(inputs, TokenType::Literal);
    }

    #[test]
    fn punctuator_lexing() {
        lexing_template(vec![";", "end"], TokenType::Punctuator);
    }
}