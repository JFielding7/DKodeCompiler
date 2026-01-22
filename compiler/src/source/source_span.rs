use crate::source::source_file::SourceFile;

// TODO: multiline span
#[derive(Debug, Copy, Clone)]
pub struct SourceSpan {
    pub start: SourceLocation,
    pub end: SourceLocation,
}

impl SourceSpan {
    pub fn single_line(line_index: usize, start: usize, end: usize) -> Self {
        Self {
            start: SourceLocation::new(line_index, start),
            end: SourceLocation::new(line_index, end)
        }
    }

    pub fn new(start: SourceLocation, end: SourceLocation) -> Self {
        Self {
            start,
            end
        }
    }

    pub fn format(&self, source: SourceFile) -> String {
        let line_num_length = (self.end.line_index + 1).to_string().len();
        let pre_underline_space = " ".repeat(line_num_length);

        let mut formatted_span = format!(
            "File:  {file}:{line_num_header}:{col}\n",
            file = source.path_display(),
            line_num_header = self.start.line_index + 1,
            col = self.start.char_index
        );

        let start_char_index = self.start.char_index;
        let first_line_str = source.get_line(self.start.line_index);

        let single_line = self.start.line_index == self.end.line_index;

        let end_char_index = if single_line {
            self.end.char_index
        } else {
            first_line_str.len()
        };

        let formatted_start = format!(
            "{line_num} | {line_content}\n{pre_underline_space} | {underline}\n",
            line_num = format!("{:line_num_length$}", self.start.line_index + 1),
            line_content = first_line_str,
            pre_underline_space = pre_underline_space,
            underline = " ".repeat(start_char_index) + &"^".repeat(end_char_index - start_char_index)
        );

        formatted_span.push_str(&formatted_start);

        if !single_line {
            for line_num in self.start.line_index + 1..self.end.line_index {
                let line_content = source.get_line(line_num);

                let formatted = format!(
                    "{line_num} | {line_content}\n{pre_underline_space} | {underline}\n",
                    line_num = format!("{:line_num_length$}", line_num + 1),
                    line_content = line_content,
                    pre_underline_space = pre_underline_space,
                    underline = "^".repeat(line_content.len())
                );

                formatted_span.push_str(&formatted)
            }

            let end_char_index = self.end.char_index;
            let end_line_str = source.get_line(self.end.line_index);

            let formatted_end = format!(
                "{line_num} | {line_content}\n{pre_underline_space} | {underline}\n",
                line_num = self.end.line_index + 1,
                line_content = end_line_str,
                pre_underline_space = pre_underline_space,
                underline = "^".repeat(end_char_index) + &" ".repeat(end_line_str.len() - end_char_index)
            );

            formatted_span.push_str(&formatted_end);
        }

        formatted_span
    }
}

#[derive(Debug, Copy, Clone)]
pub struct SourceLocation {
    pub line_index: usize,
    pub char_index: usize,
}

impl SourceLocation {
    pub fn new(line_index: usize, char_index: usize) -> Self {
        Self {
            line_index,
            char_index,
        }
    }

    pub fn shift_right(&self) -> Self {
        Self {
            line_index: self.line_index,
            char_index: self.char_index + 1,
        }
    }
}
