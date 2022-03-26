use clc_core::Calculator;
use reedline::{Reedline, Signal};

use crate::prompt::ClcPrompt;

pub fn repl() -> std::io::Result<()> {
    let calculator = Calculator::new();
    let prompt = ClcPrompt::new();
    let mut line_editor = Reedline::create()?.with_edit_mode(Box::new(reedline::Vi::default()));

    print_initial_help();

    loop {
        let sig = line_editor.read_line(&prompt)?;
        match sig {
            Signal::Success(line) => {
                if line == "quit" {
                    break;
                }
                match calculator.calculate_line(&line) {
                    Ok(eval) => println!("{eval}"),
                    Err(err) => eprintln!("{err}"),
                }
            }
            Signal::CtrlD | Signal::CtrlC => {
                break;
            }
            Signal::CtrlL => {
                line_editor.clear_screen()?;
            }
        }
    }

    println!("bye");

    Ok(())
}

fn print_initial_help() {
    println!("Version: v{}", env!("CARGO_PKG_VERSION"));
    println!("To quit, press Ctrl+C or type quit");
}