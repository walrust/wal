use std::{
    collections::HashSet,
    error::Error,
    fs::{self, OpenOptions},
    io::Write,
    path::{Path, PathBuf},
};

use regex::Regex;

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
        if self.output_path.exists() {
            fs::remove_file(&self.output_path).unwrap();
        }
        self.bound_stylesheets.clear();
    }
    pub fn bind_stylesheet(&mut self, path: PathBuf) -> Result<(), Box<dyn Error>> {
        if !path.exists() {
            panic!("found path does not exist: {}", path.display());
        }

        let stylesheet_name = Self::get_component_name_from_path(&path);

        if !self.bound_stylesheets.insert(stylesheet_name.to_owned()) {
            panic!("error inserting stylesheet name: {}", stylesheet_name);
        }

        let mut stylesheet_str = Self::read_file(path)?;
        stylesheet_str = Self::skip_comments(stylesheet_str);
        stylesheet_str = Self::collapse_whitespaces(stylesheet_str);

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
        global_stylesheet_str = Self::skip_comments(global_stylesheet_str);
        global_stylesheet_str = Self::collapse_whitespaces(global_stylesheet_str);
        self.write_to_output(global_stylesheet_str)?;

        Ok(())
    }

    // utility private methods
    fn get_component_name_from_path(path: &Path) -> String {
        let path_str = path.display().to_string();
        let rgx = Regex::new(r"/|\\").unwrap();
        let parts = rgx.split(&path_str);
        parts
            .last()
            .unwrap()
            .strip_suffix(".wal.css")
            .unwrap()
            .to_owned()
    }
    fn trim_front_whitespaces(str: &mut String) {
        while str.starts_with(|c: char| c.is_whitespace()) {
            str.remove(0);
        }
    }
    fn skip_comments(str_w_comments: String) -> String {
        let rgx = Regex::new(r"/\*([^*]|[\r\n]|(\*+([^*/]|[\r\n])))*\*+/").unwrap();
        rgx.replace_all(&str_w_comments, "").into_owned()
    }
    fn collapse_whitespaces(str: String) -> String {
        let rgx = Regex::new(r"[\r\n]{2,}").unwrap();
        rgx.replace_all(&str, "\r\n").into_owned()
    }
    fn append_attribute(selector: &str, c_name: &str) -> String {
        //check for complex selector
        let mut s_selectors = selector.split(',').peekable();

        let mut result = String::new();
        while let Some(s) = s_selectors.next() {
            result.push_str(s.trim_end());
            result.push_str("[data-component=\"");
            result.push_str(c_name);
            result.push_str("\"]");
            if s_selectors.peek().is_some() {
                result.push(',');
            }
            result.push(' ');
        }

        result
    }
    fn get_instruction(css_str: &mut String) -> String {
        let end_idx = css_str
            .find(|c: char| c == '{' || c == ';')
            .expect("error getting instruction");
        css_str.drain(..end_idx).collect()
    }
    fn get_body(css_str: &mut String) -> String {
        let mut nest_lvl = 1;
        let mut body_str = String::new();

        while !css_str.is_empty() && nest_lvl > 0 {
            let c = css_str.remove(0);
            match c {
                '{' => nest_lvl += 1,
                '}' => nest_lvl -= 1,
                _ => (),
            }
            body_str.push(c);
        }
        body_str.pop();
        body_str
    }
    fn wrap_in_nesting(str: &str) -> String {
        let mut wrapped = String::new();
        wrapped.push_str("\n{");
        wrapped.push_str(str);
        wrapped.push_str("}\n");
        wrapped
    }
    fn parse_instruction(css_str: &mut String, comp_name: &str) -> String {
        let mut parsed_str = String::new();

        Self::trim_front_whitespaces(css_str);
        if css_str.is_empty() {
            return parsed_str;
        }

        let instruction = Self::get_instruction(css_str);
        let c = css_str.remove(0);
        let mut body: Option<String> = None;
        match c {
            '{' => body = Some(Self::get_body(css_str)),
            ';' => (),
            _ => panic!("unexpected char"),
        }

        if !instruction.trim().starts_with('@') {
            parsed_str.push_str(&Self::append_attribute(&instruction, comp_name));
        } else {
            parsed_str.push_str(&instruction);
            if body.is_none() {
                parsed_str.push_str(";\n")
            };
        }

        if let Some(mut body) = body {
            if instruction.starts_with("@media") || instruction.starts_with("@supports") {
                body = Self::parse_instruction(&mut body, comp_name);
            }
            parsed_str.push_str(&Self::wrap_in_nesting(&body));
        }
        parsed_str
    }

    // files i/o methods
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
