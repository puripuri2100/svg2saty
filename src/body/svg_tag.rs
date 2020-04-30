pub fn get_y_len(data: &svg::node::Attributes) -> Option<f32> {
  match data.get("viewBox") {
    Some(v) => {
      let string = v.to_string();
      let f: Vec<&str> = (&string).split(' ').collect();
      match f.get(3) {
        Some(s) => match s.parse() {
          Ok(f) => Some(f),
          Err(_) => None,
        },
        None => None,
      }
    }
    None => None,
  }
}
