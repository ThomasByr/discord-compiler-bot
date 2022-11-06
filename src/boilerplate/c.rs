use crate::boilerplate::generator::BoilerPlateGenerator;
use std::fmt::Write as _;

use crate::utls::constants::C_LIKE_MAIN_REGEX;

pub struct CGenerator {
    input: String,
}

impl BoilerPlateGenerator for CGenerator {
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
            if trimmed.starts_with("#i") || trimmed.starts_with("#d") {
                // header.push_str(&format!("{}\n", trimmed));
                writeln!(header, "{}", trimmed).unwrap();
            } else {
                // main_body.push_str(&format!("{}\n", trimmed));
                writeln!(main_body, "{}", trimmed).unwrap();
            }
        }

        if (main_body.contains("printf") || main_body.contains("scanf"))
            && !header.contains("stdio.h")
        {
            header.push_str("#include <stdio.h>")
        }
        if (main_body.contains("malloc")
            || main_body.contains("free")
            || main_body.contains("realloc")
            || main_body.contains("calloc")
            || main_body.contains("EXIT_FAILURE")
            || main_body.contains("EXIT_SUCCESS"))
            && !header.contains("stdlib.h")
        {
            header.push_str("#include <stdlib.h>")
        }
        if (main_body.contains("strlen") || main_body.contains("strcmp"))
            && !header.contains("string.h")
        {
            header.push_str("#include <string.h>")
        }
        if (main_body.contains("time") || main_body.contains("ctime")) && !header.contains("time.h")
        {
            header.push_str("#include <time.h>")
        }

        format!("{}\nint main(void) {{\n{}return 0;\n}}", header, main_body)
    }

    fn needs_boilerplate(&self) -> bool {
        for m in C_LIKE_MAIN_REGEX.captures_iter(&self.input) {
            if m.name("main").is_some() {
                return false;
            }
        }
        true
    }
}
