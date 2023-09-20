use std::io::{BufRead, SeekFrom};
use unicode_segmentation::UnicodeSegmentation;

#[derive(Debug, Clone)]
pub struct Line {
    pub text: String,
    pub line_number: usize,
    pub is_wrapped: bool,
}

impl Line {
    fn new(text: String, line_number: usize) -> Self {
        Self {
            text,
            line_number,
            is_wrapped: false,
        }
    }
}

#[derive(Debug, Clone)]
pub(crate) struct FileContent {
    pub(crate) content: Vec<Line>,
}

impl FileContent {
    fn new(content: Vec<Line>) -> Self {
        Self { content }
    }

    pub fn read(file_path: &str) -> Self {
        let mut line_vec = Vec::new();
        let file = std::fs::File::open(file_path).unwrap();
        let mut reader = std::io::BufReader::new(file);
        let mut line_number = 0;
        loop {
            let mut line = String::new();
            let len = reader.read_line(&mut line).unwrap();
            if len == 0 {
                break;
            }
            line_number += 1;
            let mut current_line_len = 0;
            let mut current_line = String::new();
            for grapheme in line.graphemes(true) {
                current_line.push_str(grapheme);
                current_line_len += 1;

                if current_line_len == 40 {
                    line_vec.push(Line::new(current_line, line_number));
                    current_line = String::new();
                    current_line_len = 0;
                }
            }
            if current_line_len > 0 {
                line_vec.push(Line::new(current_line, line_number));
            }
        }
        Self::new(line_vec)
    }
}