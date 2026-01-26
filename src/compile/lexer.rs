mod tests;

enum ScanState {
    Normal,
    Literal(char)
}

// Splits the source into symbols (like `block`, or `costume`, or `[Stage]`)
pub fn scan(source: impl Into<String>) -> Vec<String> {
    let mut string_source = source.into(); // Only mutable so that space can be added to the end
    string_source.push(' ');

    let mut symbols = Vec::new();
    let mut symbol = String::new();
    let mut state = ScanState::Normal;
    let mut escaped = false;

    for c in string_source.chars() {
        match state {
            ScanState::Normal => {
                // Adds the symbol the vector of symbols if encountering whitespace
                if " \t\n\r".contains(c) {
                    // In separate if statement so that it doesn't push the whitespace to the symbol
                    // even if the symbol is empty
                    if !symbol.trim().is_empty() {
                        symbols.push(symbol.clone());
                        symbol = String::new();
                    }
                    
                    continue;
                }

                // Checks if the symbol is starting with a character that starts a literal
                // Not for number literals and boolean literals, they do not have closing delimiters
                if symbol.is_empty() {
                    state = match c {
                        '"' => ScanState::Literal('"'),
                        '[' => ScanState::Literal(']'),
                        '{' => ScanState::Literal('}'),
                        _ => ScanState::Normal
                    };
                    escaped = false;
                }

                symbol.push(c);
            },
            ScanState::Literal(closing_delimiter) => {
                if escaped {
                    symbol.push(c);
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

                    symbol.push(c);
                }
            }
        }
    }

    symbols
}