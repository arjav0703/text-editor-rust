use anyhow::Result;

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
}
