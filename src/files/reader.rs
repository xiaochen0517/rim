use std::collections::HashMap;
use std::io::{BufRead};
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

#[derive(Debug, Clone)]
pub(crate) struct FileContent {
    pub(crate) content: HashMap<usize, Line>,
}

impl FileContent {
    fn new(content: HashMap<usize, Line>) -> Self {
        Self { content }
    }

    pub fn read(file_path: &str) -> Self {
        let mut content_map = HashMap::new();
        let file = std::fs::File::open(file_path).unwrap();
        let mut reader = std::io::BufReader::new(file);
        let mut line_number = 0;
        let mut real_line_number = 0;
        loop {
            let mut line = String::new();
            let len = reader.read_line(&mut line).unwrap();
            if len == 0 {
                break;
            }
            line_number += 1;
            let mut line_str_len = 0;
            let mut is_wrapped = false;
            let mut line_text = String::new();
            for grapheme in line.graphemes(true) {
                line_text.push_str(grapheme);
                line_str_len += 1;
                if line_str_len == 40 {
                    content_map.insert(real_line_number, Line::new(line_text, line_number, is_wrapped));
                    is_wrapped = true;
                    real_line_number += 1;
                    line_text = String::new();
                    line_str_len = 0;
                }
            }
            if line_str_len > 0 {
                content_map.insert(real_line_number, Line::new(line_text, line_number, is_wrapped));
                real_line_number += 1;
            }
        }
        Self::new(content_map)
    }
}