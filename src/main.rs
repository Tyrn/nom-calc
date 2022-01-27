use rustyline::config::{Config, EditMode};
use rustyline::error::ReadlineError;
use rustyline::Editor;

fn main() {
    let config = Config::builder().edit_mode(EditMode::Vi).build();
    // `()` can be used when no completer is required
    let mut rl = Editor::<()>::with_config(config);
    if rl.load_history(".calc-history").is_err() {
        eprintln!("No previous history.");
    }

    loop {
        let readline = rl.readline("user> ");
        match readline {
            Ok(line) => {
                rl.add_history_entry(&line);
                rl.save_history(".calc-history").unwrap();
                if line.len() > 0 {
                    println!("{}", line);
                }
            }
            Err(ReadlineError::Interrupted) => continue,
            Err(ReadlineError::Eof) => break,
            Err(err) => {
                println!("Error: {:?}", err);
                break;
            }
        }
    }
}
