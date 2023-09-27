use std::fs::File;
use std::io::BufRead;

use unicode_segmentation::UnicodeSegmentation;

#[derive(Debug, Clone)]
pub struct Line {
    pub text: String,
    pub line_number: usize,
    pub is_wrapped: bool,
}

impl Line {
    pub(crate) fn new(text: String, line_number: usize, is_wrapped: bool) -> Self {
        Self {
            text,
            line_number,
            is_wrapped,
        }
    }
}

pub fn read(file_path: &str) -> Vec<Line> {
    // 获取终端宽度
    let (terminal_width, _terminal_height) = crossterm::terminal::size().unwrap();
    let mut content_list = Vec::new();
    let file = std::fs::File::open(file_path).unwrap();
    let mut reader = std::io::BufReader::new(file);
    let mut line_number = 1;
    let mut per_line_end_warp = false;
    loop {
        let line = match read_line(&mut reader) {
            Some(line) => line,
            None => break,
        };
        // 判断最后一行是否以换行符结尾
        per_line_end_warp = line.ends_with('\n');
        let mut line_str_len = 0;
        let mut is_wrapped = false;
        let mut line_text = String::new();
        for grapheme in line.graphemes(true) {
            line_text.push_str(grapheme);
            line_str_len += 1;
            if line_str_len == terminal_width {
                content_list.push(Line::new(line_text, line_number, is_wrapped));
                is_wrapped = true;
                line_text = String::new();
                line_str_len = 0;
            }
        }
        if line_str_len > 0 {
            content_list.push(Line::new(line_text, line_number, is_wrapped));
        }
        line_number += 1;
    }
    // 判断最后一行是否以换行符结尾，如果以换行符结尾，则添加一行空行
    if per_line_end_warp {
        content_list.push(Line::new(String::new(), line_number, false));
    }
    content_list
}

fn read_line(reader: &mut std::io::BufReader<File>) -> Option<String> {
    let mut line = String::new();
    let len = reader.read_line(&mut line).unwrap();
    if len == 0 {
        return None;
    }
    Some(line)
}
