/*
use cssparser;
use cssparser::{Color};

pub fn read_css_color (css:&str) -> Option<(f32,f32,f32)> {
  let color_opt = cssparser::parse_color_keyword(css);
  match color_opt {
    Ok(c) => {
      match c {
        Color::RGBA(c) => {
          let r = c.red_f32();
          let g = c.green_f32();
          let b = c.blue_f32();
          Some((r,g,b))
        }
        _ => {None}
      }
    }
    Err(_) => {None}
  }
}
*/

use css_color;

pub fn read_css_color(css: &str) -> Option<(f32, f32, f32)> {
  let c_opt: Result<css_color::Rgba, _> = css.parse();
  match c_opt {
    Err(_) => None,
    Ok(c) => {
      let r = c.red;
      let g = c.green;
      let b = c.blue;
      Some((r, g, b))
    }
  }
}

pub fn to_satysfi_string(rgb: (f32, f32, f32)) -> String {
  let (r, g, b) = rgb;
  format!("RGB({:?}, {:?}, {:?})", r, g, b)
}

#[test]
fn read_css_color_test() {
  assert_eq!(read_css_color("blue"), Some((0., 0., 1.)));
  assert_eq!(read_css_color("red"), Some((1., 0., 0.)));
  assert_eq!(read_css_color("#269"), Some((0.13333334, 0.4, 0.6)));
  assert_eq!(
    read_css_color("#DD2E44"),
    Some((0.8666667, 0.18039216, 0.26666668))
  );
}
