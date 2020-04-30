use super::color;

fn show_float(f: f32) -> String {
  if f <= 0.0 {
    format!("Float.neg {:?}", f.abs())
  } else {
    format!("{:?}", f)
  }
}

pub fn main(data: &svg::node::Attributes) -> Option<String> {
  let fill_data_opt = data.get("fill");
  let cx_data_opt = data.get("cx");
  let cy_data_opt = data.get("cy");
  let r_data_opt = data.get("r");

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
  match (cx_data_opt, cy_data_opt, r_data_opt) {
    (Some(cx), Some(cy), Some(r)) => {
      let cx_opt: Result<f32, _> = cx.to_string().parse();
      let cx_string = match cx_opt {
        Ok(f) => format!("Some({})", show_float(f)),
        Err(_) => "None".to_string(),
      };
      let cy_opt: Result<f32, _> = cy.to_string().parse();
      let cy_string = match cy_opt {
        Ok(f) => format!("Some({})", show_float(f * -1.0)),
        Err(_) => "None".to_string(),
      };
      let r_opt: Result<f32, _> = r.to_string().parse();
      let r_string = match r_opt {
        Ok(f) => format!("Some({})", show_float(f)),
        Err(_) => "None".to_string(),
      };
      Some(format!(
        "Circle({}, {}, {}, {})",
        fill_string, cx_string, cy_string, r_string
      ))
    }
    _ => None,
  }
}
