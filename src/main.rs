mod repl;
mod lang;
use repl::REPL;

const INPUT_PROMPT  : &str = "racnul>";
const OUTPUT_PROMPT : &str = "output>";
const MESSAGE_PROMPT: &str = "cursor>";

fn main() {
    println!("Welcome to RaCNuL REPL");
    println!("By marc-dantas");
    let repl = REPL::new(INPUT_PROMPT, OUTPUT_PROMPT, MESSAGE_PROMPT);
    let mut digits = vec![0];
    let mut cur: usize = 0;
    loop {
        let code = repl.input();
        let result = lang::exec_tokens(&mut cur, &mut digits, lang::lex_string(code));
        let mut cur_string = String::new();
        cur_string.push_str(" ".repeat(digits.len()).as_str());
        cur_string.replace_range(cur..cur+1, "^");
        match result {
            Ok(r) => {
                repl.output(r);
                repl.message(cur_string);
            },
            Err(e) => {
                repl.output(e);
            },
        };
    }
}
