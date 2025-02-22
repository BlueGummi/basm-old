#[macro_export]
macro_rules! print_msg {
    ($($arg:tt)*) => {
        #[allow(unused_imports)]
        use colored::*;
        let full_width = term_size::dimensions().map(|(w, _)| w).unwrap_or(80);
        let box_width = full_width / 2;
        let padding = (full_width - box_width) / 2;

        let message = format!($($arg)*);

        let mut lines = Vec::new();
        for line in message.split('\n') {
            let mut words = line.split_whitespace().peekable();
            let mut current_line = String::new();

            if line.trim().is_empty() {
                lines.push(String::new());
                continue;
            }

            while let Some(word) = words.next() {
                if current_line.len() + word.len() + 1 > box_width.saturating_sub(4) {
                    lines.push(current_line);
                    current_line = String::new();
                }
                if !current_line.is_empty() {
                    current_line.push(' ');
                }
                current_line.push_str(word);
            }
            if !current_line.is_empty() {
                lines.push(current_line);
            }
        }

        let top_border = format!("{:padding$}{}{}{}", "", "╔".blue(), "═".repeat(box_width.saturating_sub(2)).blue(), "╗".blue());
        let bottom_border = format!("{:padding$}{}{}{}", "", "╚".blue(), "═".repeat(box_width.saturating_sub(2)).blue(), "╝".blue());

        println!("{}", top_border);

        for line in &lines {
            if line.is_empty() {
                println!("{:padding$}{}{:^width$}{}", "", "║".blue(), "", "║".blue(), width = box_width.saturating_sub(2));
            } else {
                println!("{:padding$}{}{:^width$}{}", "", "║".blue(), line, "║".blue(), width = box_width.saturating_sub(2));
            }
        }

        println!("{}", bottom_border);
    };
}
