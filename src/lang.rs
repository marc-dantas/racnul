pub enum Token {
    Inc(i16, char),
    Nxt(i16, char),
    Lst(i16, char),
    New(i16, char),
    Cpy(i16, char),
    Pst(i16, char),
    Zro(i16, char),
    Other,
}

pub fn lex_string(string: String) -> Vec<Token> {
    let chrs = string.chars();
    let mut cmds = Vec::new();
    for (i, chr) in chrs.enumerate() {
        cmds.push(match chr {
            '+' => Token::Inc(i as i16, '+'),
            '>' => Token::Nxt(i as i16, '>'),
            '<' => Token::Lst(i as i16, '<'),
            '#' => Token::New(i as i16, '#'),
            ',' => Token::Cpy(i as i16, ','),
            '.' => Token::Pst(i as i16, '.'),
            '%' => Token::Zro(i as i16, '%'),
            _ => Token::Other
        });
    }
    return cmds;
}

fn format_err(pos: i16, character: char, msg: &str) -> String {
    format!("error at column {} ('{}'): {}", pos, character, msg)
}

pub fn exec_tokens(cursor_position: &mut usize, digits: &mut Vec<i32>, tokens: Vec<Token>) -> Result<String, String> {
    let mut clipboard: i32 = 0;
    for token in tokens {
        match token {
            Token::Inc(_, _) => {
                digits[*cursor_position] += 1;
                if digits[*cursor_position] >= 10 {
                    digits[*cursor_position] = 0;
                }
            },
            Token::Nxt(i, c) => {
                *cursor_position += 1;
                if *cursor_position >= digits.len() {
                    *cursor_position -= 1;
                    return Err(format_err(i, c, "Cannot go forward since the cursor position is in the highest digit."));
                }
            },
            Token::Lst(i, c) => {
                if *cursor_position > 0 {
                    *cursor_position -= 1;
                } else {
                    return Err(format_err(i, c, "Cannot go backward since the cursor position is in the lowest digit."));
                }
            },
            Token::New(_, _) => { digits.push(0) },
            Token::Cpy(_, _) => { clipboard = digits[*cursor_position] },
            Token::Pst(_, _) => { digits[*cursor_position] = clipboard as i32 },
            Token::Zro(_, _) => { digits[*cursor_position] = 0 },
            Token::Other  => { },
        }
    }
    let mut res = String::new();
    for digit in digits { res.push_str(&digit.to_string()) }
    return Ok(res);
}