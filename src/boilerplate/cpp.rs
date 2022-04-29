use crate::boilerplate::generator::BoilerPlateGenerator;
use regex::Regex;

pub struct CppGenerator {
    input: String,
}

impl BoilerPlateGenerator for CppGenerator {
    fn new(input: &str) -> Self {
        let mut formated = input.to_string();
        formated = formated.replace(';', ";\n"); // separate lines by ;

        Self { input: formated }
    }

    fn generate(&self) -> String {
        let mut main_body = String::default();
        let mut header = String::default();

        let lines = self.input.split('\n');
        for line in lines {
            let trimmed = line.trim();
            if trimmed.starts_with("using") || trimmed.starts_with("#i") {
                header.push_str(&format!("{}\n", trimmed));
            } else {
                main_body.push_str(&format!("{}\n", trimmed))
            }
        }

        // if they included nothing, we can just manually include everything
        if !header.contains("#include") {
            header.push_str("#include <bits/stdc++.h>");
        }
        format!("{}\nint main(void) {{\n{}return 0;\n}}", header, main_body)
    }

    fn needs_boilerplate(&self) -> bool {
        let cpp_main_regex: Regex = Regex::new("\"[^\"]+\"|(?P<main>main[\\s]*?\\()").unwrap();
        for m in cpp_main_regex.captures_iter(&self.input) {
            if m.name("main").is_some() {
                return false;
            }
        }
        true
    }
}
