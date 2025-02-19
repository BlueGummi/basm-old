use std::fmt;
#[derive(Debug)]
pub struct ParserError {
    pub input: String,
    pub message: String,
    pub line: usize,
    pub column: usize,
}

impl fmt::Display for ParserError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(
            f,
            "at line {}:{} - {}",
            self.line, self.column, self.message
        )?;
        let mut nlc = 0;
        for (index, character) in self.input.chars().enumerate() {
            if character == '\n' {
                nlc += 1;
            }
            if index == self.line {
                break;
            }
        }
        let mut linedata = self.input.lines();
        for _ in 0..nlc {
            linedata.next();
        }
        writeln!(f, "{}", linedata.next().unwrap())?;
        write!(
            f,
            "{}{}",
            " ".repeat(self.line),
            "^".repeat(self.column - self.line)
        )
    }
}
