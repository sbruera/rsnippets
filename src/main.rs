use std::ffi::OsStr;
use std::fs;
use std::path::Path;
use syntect::easy::HighlightLines;
use syntect::highlighting::{Style, ThemeSet};
use syntect::parsing::SyntaxSet;
use syntect::util::{as_24_bit_terminal_escaped, LinesWithEndings};

#[allow(dead_code)]
struct Snippet<'a> {
    filename: &'a str,
    file_extension: &'a str,
    code: String,
}

impl<'a> From<&'a str> for Snippet<'a> {
    fn from(filename: &'a str) -> Self {
        Self {
            filename: &filename,
            code: read_code_snippet(filename).unwrap(),
            file_extension: get_extension_from_filename(filename).unwrap(),
        }
    }
}

impl<'a> Snippet<'a> {
    fn print_code(&self) {
        // Load these once at the start of your program
        let ps = SyntaxSet::load_defaults_newlines();
        let ts = ThemeSet::load_defaults();
        let file_extension: &str;
        if self.file_extension == "ts" {
            file_extension = "js";
        } else {
            file_extension = self.file_extension;
        }
        let syntax = ps
            .find_syntax_for_file(self.filename)
            .unwrap() // for IO errors, you may want to use try!() or another plain text fallback
            .unwrap_or_else(|| {
                ps.find_syntax_by_extension(file_extension)
                    .unwrap_or_else(|| {
                        ps.find_syntax_by_token(self.file_extension)
                            .unwrap_or_else(|| ps.find_syntax_plain_text())
                    })
            });
        // let syntax = ps.find_syntax_by_extension(self.file_extension).unwrap();
        let mut h = HighlightLines::new(syntax, &ts.themes["base16-eighties.dark"]);
        for line in LinesWithEndings::from(&self.code) {
            let ranges: Vec<(Style, &str)> = h.highlight_line(line, &ps).unwrap();
            let escaped = as_24_bit_terminal_escaped(&ranges[..], false);
            print!("{}", escaped);
        }
    }
}

fn read_code_snippet(filename: &str) -> Option<String> {
    let code: String = fs::read_to_string(filename).unwrap();
    return Some(code);
}

fn get_extension_from_filename(filename: &str) -> Option<&str> {
    Path::new(filename).extension().and_then(OsStr::to_str)
}

fn main() {
    let snippet = Snippet::from("src/main.rs");
    snippet.print_code();
}
