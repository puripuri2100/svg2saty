use svg::node::element::tag::Path;
use svg::parser::Event;

pub mod color;

pub mod circle;
pub mod path;
pub mod svg_tag;

#[warn(non_snake_case)]
pub fn main(svg: svg::parser::Parser) -> String {
  let mut y_len = 0.0;
  let mut main_str = String::new();
  for event in svg {
    match event {
      Event::Tag(tag_name, _, attributes) => match tag_name {
        "svg" => {
          let f_len_opt = svg_tag::get_y_len(&attributes);
          match f_len_opt {
            Some(f) => y_len = f,
            None => {}
          }
        }
        "path" => {
          let s_opt = path::main(attributes);
          match s_opt {
            Some(s) => main_str.push_str(&format!("  {};\n", s)),
            None => {}
          }
        }
        "circle" => {
          let circle_s_opt = circle::main(&attributes);
          match circle_s_opt {
            Some(s) => main_str.push_str(&format!("  {};\n", s)),
            None => {}
          }
        }
        _ => {}
      },
      _ => {}
    }
  }
  format!("({:?},\n[\n{}])", y_len, main_str)
}
