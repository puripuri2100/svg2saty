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
      Event::Tag(Path, _, attributes) => {
        let s_opt = path::main(attributes);
        match s_opt {
          Some(s) => main_str.push_str(&format!("  {};\n", s)),
          None => {}
        }
      }
      /*
            Event::Tag(Pattern, _, attributes) => {}
            Event::Tag(Polygon, _, attributes) => {}
            Event::Tag(Polyline, _, attributes) => {}
            Event::Tag(RadialGradient, _, attributes) => {}
            Event::Tag(Script, _, attributes) => {}
            Event::Tag(Stop, _, attributes) => {}
            Event::Tag(Style, _, attributes) => {}
            Event::Tag(Symbol, _, attributes) => {}
            Event::Tag(Text, _, attributes) => {}
            Event::Tag(TextPath, _, attributes) => {}
            Event::Tag(Title, _, attributes) => {}
            Event::Tag(Use, _, attributes) => {}
      */
      Event::Tag(_, _, attributes) => {
        let f_len_opt = svg_tag::get_y_len(&attributes);
        match f_len_opt {
          Some(f) => y_len = f,
          None => {}
        }
        let circle_s_opt = circle::main(&attributes);
        match circle_s_opt {
          Some(s) => main_str.push_str(&format!("  {};\n", s)),
          None => {}
        }
      }
      _ => {}
    }
  }
  format!("({:?},\n[\n{}])", y_len, main_str)
}
