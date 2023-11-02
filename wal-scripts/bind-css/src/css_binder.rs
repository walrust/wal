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

        let mut stylesheet_str = Self::apply_binding(Self::read_file(path)?, "a");
        // self.write_to_output(Self::apply_binding(stylesheet_str, &stylesheet_name))?;

        Self::trim_front_whitespaces(&mut stylesheet_str);
        Self::handle_instruction(&mut stylesheet_str);
        Self::trim_front_whitespaces(&mut stylesheet_str);
        Self::handle_instruction(&mut stylesheet_str);
        Self::trim_front_whitespaces(&mut stylesheet_str);
        Self::handle_instruction(&mut stylesheet_str);

        Ok(())
    }

    pub fn bind_global_stylesheet(&mut self, path: PathBuf) -> Result<(), Box<dyn Error>> {
        if !path.exists() {
            panic!("found path does not exist: {}", path.display());
        }

        let global_stylesheet_str = Self::read_file(path)?;
        self.write_to_output(global_stylesheet_str)?;

        Ok(())
    }

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

    fn apply_binding(file_str: String, component_class: &str) -> String {
        // let mut bound_css = String::new();
        // bound_css.push('.');
        // bound_css.push_str(component_class);
        // bound_css.push_str(" {\n");
        // bound_css.push_str(&file_str);
        // bound_css.push('}');
        // bound_css

        let mut str = Self::skip_comments(file_str);
        str = Self::format_whitespaces(str);
        str
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
    fn format_whitespaces(str: String) -> String {
        // TODO: add spaces and \t collapsing
        let rgx = Regex::new(r"[\r\n]{2,}").unwrap();
        rgx.replace_all(&str, "\r\n").into_owned()
    }
    fn append_attribute(selector: &mut String, c_name: &str) {
        selector.push_str("[data-component=");
        selector.push_str(c_name);
        selector.push(']');
    }
    fn get_selector(css_str: &mut String) -> String {
        let mut selector = String::new();
        while !css_str.is_empty() && !css_str.starts_with('{') {
            selector.push(css_str.remove(0));
        }
        return selector;
    }

    pub fn get_instruction(css_str: &mut String) -> String {
        let end_idx = css_str
            .find(|c: char| c == '{' || c == ';')
            .expect("error getting instruction");
        css_str.drain(..end_idx).collect()
    }
    pub fn get_body(css_str: &mut String) -> String {
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

        if body_str.ends_with('}') {
            body_str.pop();
        }
        body_str
    }

    pub fn handle_instruction(css_str: &mut String) {
        let instruction = Self::get_instruction(css_str);
        println!("instruction: {}", instruction);
        let c = css_str.remove(0);
        let body;
        match c {
            '{' => {
                body = Self::get_body(css_str);
                println!("body: {}", body)
            }
            ';' => println!("semicolon end"),
            _ => println!("unexpected char"),
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
