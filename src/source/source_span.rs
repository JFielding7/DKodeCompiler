use crate::source::source_file::SourceFile;

#[derive(Debug, Copy, Clone)]
pub struct SourceSpan {
    pub line_index: usize,
    pub start: usize,
    pub end: usize,
}

impl SourceSpan {
    pub fn new(
        line_index: usize,
        start: usize,
        end: usize
    ) -> Self {
        Self {
            line_index,
            start,
            end
        }
    }

    pub fn format_source_span(&self, source: SourceFile) -> String {
        let line_num = self.line_index + 1;
        let line_num_str = line_num.to_string();

        format!("File:  {file}:{line_num_header}:{col}\n{line_num} | {line_content}\n{pre_underline_space} | {underline}",
                file = source.path_display(),
                line_num_header = line_num,
                col = self.start,
                line_num = line_num_str,
                line_content = source.get_line(self.line_index),
                pre_underline_space = " ".repeat(line_num_str.len()),
                underline = " ".repeat(self.start) + &"^".repeat(self.end - self.start)
        )
    }
}
