pub fn make_satysfi_file_text(package_list: Vec<&str>, fun_name: &str, body: &str) -> String {
  let mut package_str = String::new();
  for p in package_list.iter() {
    package_str.push_str(&format!("@require: {}\n", p))
  }
  format!("{}\nlet {} =\n{}\n", package_str, fun_name, body)
}
