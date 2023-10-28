use std::{
    collections::HashSet,
    error::Error,
    fs::{self, OpenOptions},
    io::Write,
    path::PathBuf,
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

        let stylesheet_str = Self::read_file(path)?;
        self.write_to_output(Self::apply_binding(stylesheet_str, &stylesheet_name))?;

        Ok(())
    }

    fn get_component_name_from_path<'a>(path: &PathBuf) -> String {
        let path_str = path.as_path().display().to_string();
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
        let mut bound_css = String::new();
        bound_css.push('.');
        bound_css.push_str(component_class);
        bound_css.push_str(" {\n");
        bound_css.push_str(&file_str);
        bound_css.push('}');
        bound_css
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
