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
        // Print the error message with start and end indices
        writeln!(
            f,
            "at indices {}:{} - {}",
            self.line, self.column, self.message
        )?;

        // Ensure the indices are valid
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

        // Print the lines containing the error range
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
                // Print the line
                writeln!(f, "{}", line)?;

                // Calculate the start and end positions of the error within this line
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

                // Print the arrows under the error range
                let spaces = " ".repeat(error_start);
                let arrows = "^".repeat(error_end.saturating_sub(error_start));
                writeln!(f, "{}{}", spaces, arrows)?;
            }
        }

        Ok(())
    }
}
