use std::io::{self, Write};

pub struct REPL {
    input_prompt: String,
    output_prompt: String,
    message_prompt: String,
}

impl REPL {
    pub fn new(input_prompt: &str, output_prompt: &str, message_prompt: &str) -> REPL {
        REPL {
            input_prompt: input_prompt.to_string(),
            output_prompt: output_prompt.to_string(),
            message_prompt: message_prompt.to_string(),
        }
    }

    pub fn input(&self) -> String {
        let mut result = String::new();
        print!("{} ", self.input_prompt);
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut result).unwrap();
        result.trim().to_string()
    }

    pub fn output(&self, output: String) {
        print!("{} ", self.output_prompt);
        io::stdout().flush().unwrap();
        println!("{}", output);
    }

    pub fn message(&self, output: String) {
        print!("{} ", self.message_prompt);
        io::stdout().flush().unwrap();
        println!("{}", output);
    }
}