use anyhow::Result;
use std::ops::Range;
use unicode_segmentation::UnicodeSegmentation;

#[derive(Debug, Clone, Default)]
pub struct Buffer {
    pub content: Vec<String>,
}

impl Buffer {
    pub fn is_empty(&self) -> bool {
        self.content.is_empty()
    }

    pub fn read_from_file(file: &str) -> Result<Buffer> {
        let content = std::fs::read_to_string(file)?
            .lines()
            .map(|line| line.to_string())
            .collect();

        Ok(Buffer { content })
    }

    pub fn get(&self, range: Range<usize>) -> String {
        let mut result = String::new();
        for i in range {
            if let Some(line) = self.content.get(i) {
                if !result.is_empty() {
                    result.push('\n');
                }
                result.push_str(line);
            } else {
                break;
            }
        }
        result
    }

    pub fn get_line_length(&self, line_number: usize) -> usize {
        let line = self.content.get(line_number);
        match line {
            Some(text) => {
                let graphemes = text.graphemes(true).collect::<Vec<&str>>();
                graphemes.len()
            }
            None => 0,
        }
    }
}
