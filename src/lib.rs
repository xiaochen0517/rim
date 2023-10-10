#[macro_use]
extern crate log;

use std::env;
use std::fs::OpenOptions;
use std::io::Write;

use env_logger::WriteStyle;
use env_logger::{Builder, Target};
use log::{info, LevelFilter};

use crate::editor::Editor;

mod args;
mod editor;
mod events;
mod files;

const LOG_PATH: &str = "logs/";

pub fn run() {
    // 创建日志目录
    std::fs::create_dir_all(LOG_PATH).unwrap();

    // 初始化env_logger
    env::set_var("RUST_LOG", "trace");
    let mut builder = Builder::new();
    builder.filter(None, LevelFilter::Trace);

    // 生成日志名称 log-{time}.txt
    let log_file_name = format!("log-{}.txt", chrono::Local::now().format("%Y-%m-%d-%H"));

    // 打开日志文件
    let file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(LOG_PATH.to_string() + log_file_name.as_str())
        .unwrap();

    // 将日志记录到文件中
    builder
        .write_style(WriteStyle::Always)
        .target(Target::Pipe(Box::new(file)))
        .init();

    info!("starting up");
    let mut current_editor = Editor::new();
    current_editor.show_content().unwrap();
    events::run(&mut current_editor);
}
