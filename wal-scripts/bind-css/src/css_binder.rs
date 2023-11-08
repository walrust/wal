use std::{
    collections::HashSet,
    error::Error,
    fs::{self, OpenOptions},
    io::Write,
    path::{Path, PathBuf},
};

use regex::Regex;
const COMMENTS_REGEX: &str = r"/\*([^*]|[\r\n]|(\*+([^*/]|[\r\n])))*\*+/";
const MULTIPLE_NEWLINES_REGEX: &str = r"[\r\n]{2,}";
const PATH_SEPARATOR_REGEX: &str = r"/|\\";

pub const COMPNENT_STYLE_SUFFIX: &str = ".wal.css";
const HTML_COMPONENT_ATTTRIBUTE: &str = "data-component";

pub struct CssBinder {
    output_path: PathBuf,
    bound_stylesheets: HashSet<String>,
}

impl CssBinder {
    pub fn new(output_path: &str) -> CssBinder {
        let mut binder = CssBinder {
            output_path: PathBuf::from(output_path),
            bound_stylesheets: HashSet::new(),
        };
        binder.reset();
        binder
    }

    // Public methods
    pub fn reset(&mut self) {
        Self::remove_file_if_exists(self.output_path.as_path());
        self.bound_stylesheets.clear();
    }

    pub fn bind_stylesheet(&mut self, path: PathBuf) -> Result<(), Box<dyn Error>> {
        if !path.exists() {
            panic!("found path does not exist: {}", path.display());
        }

        let stylesheet_name = Self::get_component_name_from_path(&path);

        // mark stylesheet_name as bound
        if !self.bound_stylesheets.insert(stylesheet_name.to_owned()) {
            panic!("error inserting stylesheet name: {}", stylesheet_name);
        }

        // read the file
        let mut stylesheet_str = Self::read_file(path)?;
        stylesheet_str = Self::clean_file_string(stylesheet_str);

        // parse file instuctions and write them to output
        while !stylesheet_str.is_empty() {
            let parsed_instruction = Self::parse_instruction(&mut stylesheet_str, &stylesheet_name);
            self.write_to_output(parsed_instruction)?;
        }

        Ok(())
    }

    pub fn bind_global_stylesheet(&mut self, path: PathBuf) -> Result<(), Box<dyn Error>> {
        if !path.exists() {
            panic!("found path does not exist: {}", path.display());
        }

        let mut global_stylesheet_str = Self::read_file(path)?;
        global_stylesheet_str = Self::clean_file_string(global_stylesheet_str);
        self.write_to_output(global_stylesheet_str)?;

        Ok(())
    }

    // utility private methods
    fn get_component_name_from_path(path: &Path) -> String {
        let path_str = path.display().to_string();
        let rgx = Regex::new(PATH_SEPARATOR_REGEX).unwrap();
        let parts = rgx.split(&path_str);
        parts
            .last()
            .unwrap()
            .strip_suffix(COMPNENT_STYLE_SUFFIX)
            .unwrap()
            .to_owned()
    }

    fn trim_front_whitespaces(str: &mut String) {
        if let Some(first_non_whitespace) = str.find(|c: char| !c.is_whitespace()) {
            *str = str[first_non_whitespace..].to_string();
        } else {
            str.clear();
        }
    }

    fn skip_comments(str_w_comments: String) -> String {
        let rgx = Regex::new(COMMENTS_REGEX).unwrap();
        rgx.replace_all(&str_w_comments, "").into_owned()
    }

    fn collapse_newlines(str: String) -> String {
        let rgx = Regex::new(MULTIPLE_NEWLINES_REGEX).unwrap();
        rgx.replace_all(&str, "\r\n").into_owned()
    }

    fn append_attribute(selector: &str, c_name: &str) -> String {
        let appendix = format!("[{}=\"{}\"]", HTML_COMPONENT_ATTTRIBUTE, c_name);

        selector
            .split(',')
            .map(|s| format!("{}{}", s.trim_end(), &appendix))
            .collect::<Vec<String>>()
            .join(", ")
    }

    fn extract_instruction(css_str: &mut String) -> String {
        let end_idx = css_str
            .find(|c: char| c == '{' || c == ';')
            .expect("error getting instruction");
        css_str.drain(..end_idx).collect()
    }

    fn extract_body(css_str: &mut String) -> String {
        let mut nest_lvl = 1;
        let mut end_idx = 0;

        // find the ending } on level 0
        for (i, c) in css_str.chars().enumerate() {
            if nest_lvl == 0 {
                end_idx = i;
                break;
            }
            match c {
                '{' => nest_lvl += 1,
                '}' => nest_lvl -= 1,
                _ => (),
            }
        }

        // if found delete the body from css_str and return it without top lvl brackets
        if end_idx > 0 {
            let body_with_bracket: String = css_str.drain(..end_idx).collect();
            return body_with_bracket[0..body_with_bracket.len() - 1].to_owned();
        }

        // if not found panic
        panic!("incorrect css file")
    }

    fn wrap_in_nesting(str: &str) -> String {
        format!(" {{{}}}\n", str)
    }

    fn parse_instruction(css_str: &mut String, comp_name: &str) -> String {
        // new string that will be written to output
        let mut parsed_str = String::new();

        Self::trim_front_whitespaces(css_str);
        if css_str.is_empty() {
            return parsed_str;
        }

        // extract instruction
        let instruction = Self::extract_instruction(css_str);
        let c = css_str.remove(0);

        // extract instruction body
        let mut body: Option<String> = None;
        match c {
            '{' => body = Some(Self::extract_body(css_str)),
            ';' => (),
            _ => panic!("unexpected char"),
        }

        // handle instruction
        if !instruction.trim().starts_with('@') {
            // append instruction selectors with component attribute
            parsed_str.push_str(&Self::append_attribute(&instruction, comp_name));
        } else {
            // copy instruction unchanged
            parsed_str.push_str(&instruction);
            if body.is_none() {
                parsed_str.push_str(";\n")
            };
        }

        // handle body
        if let Some(mut body) = body {
            // for special instruction parse its body recursively
            if instruction.starts_with("@media") || instruction.starts_with("@supports") {
                body = Self::parse_instruction(&mut body, comp_name);
            }
            // copy and wrap the body in nesting
            parsed_str.push_str(&Self::wrap_in_nesting(&body));
        }
        parsed_str
    }

    fn clean_file_string(str: String) -> String {
        let clean_str = Self::skip_comments(str);
        Self::collapse_newlines(clean_str)
    }

    // files i/o methods
    fn remove_file_if_exists(path: &Path) {
        if path.exists() {
            fs::remove_file(path).expect("error deleting file.");
        }
    }
    fn read_file(path: PathBuf) -> Result<String, Box<dyn Error>> {
        let file_str = fs::read_to_string(path)?.parse()?;
        Ok(file_str)
    }
    fn write_to_output(&self, new_content: String) -> Result<(), Box<dyn Error>> {
        let mut file = OpenOptions::new()
            .append(true)
            .create(true)
            .open(&self.output_path)?;

        file.write_all(new_content.as_bytes())?;
        Ok(())
    }
}
