mod repl;
mod lang;
use lang::Executor;
use repl::REPL;

const INPUT_PROMPT  : &str = "racnul>";
const OUTPUT_PROMPT : &str = "output>";
const MESSAGE_PROMPT: &str = "cursor>";

fn start_repl(input_prompt: &str, output_prompt: &str, message_prompt: &str) {
    let repl = REPL::new(input_prompt, output_prompt, message_prompt);
    let mut ex = Executor::new(
        vec![],
        String::from("(REPL input)")
    );
    loop {
        let src = repl.input();
        ex.ops = lang::parse(&src);
        let mut cur_string = String::new();
        cur_string.push_str(" ".repeat(ex.target.len()).as_str());
        cur_string.replace_range(ex.cursor..ex.cursor+1, "^");
        let r = ex.run();
        match r {
            Ok(r) => {
                if src == "exit" {
                    println!("{r}");
                    return;
                }
                repl.output(r.to_string());
                repl.message(cur_string);
            },
            Err(e) => {
                repl.output(e);
            },
        }
    }
}

fn main() {
    let mut argv = std::env::args();
    argv.next();
    
    if let Some(src) = argv.next() {
        let mut ex: Executor = Executor::new(
            lang::parse(&src),
            String::from("(command line input)")
        );
        match ex.run() {
            Ok(r) => {
                println!("{r}");
            },
            Err(e) => {
                eprintln!("{e}");
            },
        }
        return;
    }
    
    start_repl(INPUT_PROMPT, OUTPUT_PROMPT, MESSAGE_PROMPT);
}
