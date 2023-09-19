use std::fs;
use std::io::{stdout, Write};
use clap::Parser;
use crossterm::{cursor, execute, QueueableCommand};
use crossterm::terminal::{Clear, ClearType};

mod args;

pub fn run() {
    let args = args::Args::parse();
    let file_path = args.file_path.unwrap_or_else(|| "world".to_string());
    println!("file path : {}", file_path);
    let file_content = fs::read_to_string(file_path)
        .expect("Something went wrong reading the file");
    println!("file content : \n{}", file_content);

    let mut stdout = stdout();
    // 清空屏幕
    execute!(stdout, Clear(ClearType::All)).expect("Failed to clear screen");
    stdout.queue(cursor::MoveTo(0, 0)).expect("Failed to move cursor");
    println!("Hello, world!");
    stdout.flush().unwrap();
}