pub enum Token {
    Block,
    Uid,
    Parent,
    Next,
    In,
    Field,
    Mut,
    NullT,
    StringT,
    DoubleT,
    IntT,
    ListIdxT,
    WaitT,
    BlockPtrT,
    SubstackT,
    RecievedBroadcastT,
    DataPtrT,
    End,
    Semicolon,
    MiscData(String)
}

#[allow(unused)]
fn tokenize(basm_code: &str) -> Vec<Token> {
    let split_code: Vec<&str> = basm_code.split_whitespace().collect();
    let mut tokens: Vec<Token> = Vec::new();

    for s_token in split_code.iter() {
        tokens.push( match s_token {
            &"block" => Token::Block,
            &"uid" =>Token::Uid,
            &"parent" => Token::Parent,
            &"next" => Token::Next,
            &"in" => Token::In,
            &"field" => Token::Field,
            &"mut" => Token::Mut,
            &"null" => Token::NullT,
            &"string" => Token::StringT,
            &"double" => Token::DoubleT,
            &"int" => Token::IntT,
            &"list_idx" => Token::ListIdxT,
            &"wait" => Token::WaitT,
            &"block_ptr" => Token::BlockPtrT,
            &"substack" => Token::SubstackT,
            &"recieved_broadcast" => Token::RecievedBroadcastT,
            &"data_ptr" => Token::DataPtrT,
            &"end" => Token::End,
            &";" => Token::Semicolon,
            misc => Token::MiscData(String::from(*misc))
        });
    }

    tokens
}