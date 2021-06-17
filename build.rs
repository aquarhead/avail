use std::{env, fs};

fn main() {
  println!("cargo:rerun-if-changed=templates/index.html");

  let tmpl = fs::read_to_string("templates/index.html").unwrap();

  let path = format!("{}/index.html", env::var("OUT_DIR").unwrap());
  fs::write(
    path,
    tmpl
      // escape raw HTML template
      .replace("{", "{{")
      .replace("}", "}}")
      // insert parameters for `format!`
      .replace("Date('')", "Date('{date}')")
      .replace("[]", "[{schedules}]"),
  )
  .unwrap();
}
