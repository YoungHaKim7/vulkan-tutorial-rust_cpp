use std::io;

mod image_clear;
mod mandelbrot;

const EXAMPLES: [&str; 2] = ["image_clear", "mandelbrot"];

fn execute_example(selection: &str) {
    println!("Running '{}'", selection);
    match selection {
        "image_clear" => {
            image_clear::main();
        }
        "mandelbrot" => {
            mandelbrot::main();
        }
        _ => panic!(),
    }
}

pub fn select_example_to_run(examples: &[&str], execute: fn(&str)) {
    println!("Select example to run: (default 0)");

    for (i, example) in examples.iter().enumerate() {
        println!("{} {}", i, example);
    }

    let mut selection = String::new();
    io::stdin()
        .read_line(&mut selection)
        .expect("failed to read line");

    selection = selection.trim().to_string();

    if selection.is_empty() {
        execute(examples[0]);
    // else if selection is numeric
    } else if let Ok(i) = selection.parse::<usize>() {
        if i >= examples.len() {
            println!(
                "The given index \"{}\" doesn't correspond to any known example",
                selection
            );
        } else {
            execute(examples[i]);
        }
    } else {
        match examples.iter().position(|&s| s == selection) {
            Some(i) => {
                execute(examples[i]);
            }
            None => {
                println!("\"{}\" doesn't correspond to any known example", selection);
            }
        }
    }
}

fn main() {
    select_example_to_run(&EXAMPLES, execute_example);
}
