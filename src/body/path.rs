use svg::node::element::path::{Command, Data};

use super::color;

pub mod show_para;
pub mod show_posi;

pub fn main(data: svg::node::Attributes) -> Option<String> {
  let fill_data_opt = data.get("fill");
  let d_data_opt = data.get("d");

  let fill_string = match fill_data_opt {
    Some(fill_data) => {
      let data: String = fill_data.to_string();
      let color_opt = color::read_css_color(&data);
      match color_opt {
        Some(c) => format!("Some({})", color::to_satysfi_string(c)),
        None => "None".to_string(),
      }
    }
    None => "None".to_string(),
  };
  match d_data_opt {
    Some(d) => {
      let data_list = Data::parse(d).unwrap();
      let mut d_string = String::new();
      for command in data_list.iter() {
        match command {
          Command::Move(posi, para) => d_string.push_str(&format!(
            "    Move({}, {});\n",
            show_posi::show_posi(posi),
            show_para::show_para_move(para)
          )),
          Command::Line(posi, para) => d_string.push_str(&format!(
            "    Line({}, {});\n",
            show_posi::show_posi(posi),
            show_para::show_para_line(para)
          )),
          Command::HorizontalLine(posi, para) => d_string.push_str(&format!(
            "    HorizontalLine({}, {});\n",
            show_posi::show_posi(posi),
            show_para::show_para_horizontal_line(para)
          )),
          Command::VerticalLine(posi, para) => d_string.push_str(&format!(
            "    VerticalLine({}, {});\n",
            show_posi::show_posi(posi),
            show_para::show_para_vertical_line(para)
          )),
          Command::QuadraticCurve(posi, para) => d_string.push_str(&format!(
            "    QuadraticCurve({}, {});\n",
            show_posi::show_posi(posi),
            show_para::show_para_quadratic_curve(para)
          )),
          Command::SmoothQuadraticCurve(posi, para) => d_string.push_str(&format!(
            "    SmoothQuadraticCurve({}, {});\n",
            show_posi::show_posi(posi),
            show_para::show_para_smooth_quadratic_curve(para)
          )),
          Command::CubicCurve(posi, para) => d_string.push_str(&format!(
            "    CubicCurve({}, {});\n",
            show_posi::show_posi(posi),
            show_para::show_para_cubic_curve(para)
          )),
          Command::SmoothCubicCurve(posi, para) => d_string.push_str(&format!(
            "    SmoothCubicCurve({}, {});\n",
            show_posi::show_posi(posi),
            show_para::show_para_smooth_cubic_curve(para)
          )),
          // 楕円曲線命令 ちょっとどころではなく難しいので一旦放置
          //          Command::EllipticalArc(posi, para) => d_string.push_str(&format!(
          //            "    EllipticalArc({}, {});\n",
          //            show_posi::show_posi(posi),
          //            show_para::show_para_elliptical_arc(para)
          //          )),
          Command::Close => d_string.push_str(&format!("    Close;\n")),
          _ => {}
        }
      }
      Some(format!("Path({},\n  [\n{}  ])", fill_string, d_string))
    }
    None => None,
  }
}
