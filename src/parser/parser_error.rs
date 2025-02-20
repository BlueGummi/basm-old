use colored::*;
use std::fmt;

#[derive(Debug)]
pub struct ParserError {
    pub input: String,
    pub message: String,
    pub line: usize,   // Start index of the error in the input string
    pub column: usize, // End index of the error in the input string
}

impl fmt::Display for ParserError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.line >= self.input.len()
            || self.column > self.input.len()
            || self.line >= self.column
        {
            return writeln!(
                f,
                "Error: Indices {}:{} are out of bounds or invalid.",
                self.line, self.column
            );
        }
        let lines: Vec<&str> = self.input.lines().collect();
        for (line_number, line) in lines.iter().enumerate() {
            let line_start = self
                .input
                .lines()
                .take(line_number)
                .map(|l| l.len() + 1)
                .sum::<usize>();
            let line_end = line_start + line.len();

            if (line_start <= self.line && self.line < line_end)
                || (line_start <= self.column && self.column < line_end)
            {
                let error_start = if self.line >= line_start {
                    self.line - line_start
                } else {
                    0
                };
                let error_end = if self.column < line_end {
                    self.column - line_start
                } else {
                    line.len()
                };
                writeln!(
                    f,
                    "{}: {} at indices {}:{}",
                    "error".bright_red(),
                    self.message.bold(),
                    line_number + 1,
                    error_start,
                )?;
                writeln!(f, "{}", line)?;
                let spaces = " ".repeat(error_start);
                let arrows = "^".repeat(error_end.saturating_sub(error_start));
                write!(f, "{}{}", spaces, arrows.bright_red())?;
            }
        }

        Ok(())
    }
}
