use std::fmt::Write as _;

use crate::boilerplate::generator::BoilerPlateGenerator;
use crate::utls::constants::C_LIKE_MAIN_REGEX;

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
        // header.push_str(&format!("{}\n", trimmed));
        writeln!(header, "{}", trimmed).unwrap();
      } else {
        // main_body.push_str(&format!("{}\n", trimmed));
        writeln!(main_body, "{}", trimmed).unwrap();
      }
    }

    // if they included nothing, we can just manually include everything
    if !header.contains("#include") {
      header.push_str("#include <bits/stdc++.h>\n");
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
