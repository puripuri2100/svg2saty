use svg::node::element::path::Number;
use svg::node::element::path::Parameters;

fn data_to_vec(para: &svg::node::element::path::Parameters) -> Vec<Number> {
  let vec: Vec<Number> = Parameters::from(para.clone()).to_vec();
  vec
}

fn show_float(f: f32) -> String {
  if f <= 0.0 {
    format!("Float.neg {:?}", f.abs())
  } else {
    format!("{:?}", f)
  }
}

// パラメタの表記方法
// (): パラメタのグループ
// +: １個以上のパラメタが要求される

// パラメタが(x y)+
pub fn show_para_tuple_list(para: &svg::node::element::path::Parameters) -> String {
  let vec = data_to_vec(para);
  let len = vec.len();
  let mut main_string = String::new();
  for i in 0..len / 2 {
    let x = vec[i * 2];
    let y = vec[i * 2 + 1] * -1.0;
    main_string.push_str(&format!(" ({}, {});", show_float(x), show_float(y)))
  }
  format!("[{}]", main_string)
}

// パラメタがx+
pub fn show_para_x_list(para: &svg::node::element::path::Parameters) -> String {
  let vec = data_to_vec(para);
  let mut main_string = String::new();
  for x in vec.iter() {
    main_string.push_str(&format!(" {};", show_float(*x)))
  }
  format!("[{}]", main_string)
}

// パラメタがy+
pub fn show_para_y_list(para: &svg::node::element::path::Parameters) -> String {
  let vec = data_to_vec(para);
  let mut main_string = String::new();
  for y in vec.iter() {
    main_string.push_str(&format!(" {};", show_float(y * -1.0)))
  }
  format!("[{}]", main_string)
}

// moveto命令 パラメタは(x y)+
pub fn show_para_move(para: &svg::node::element::path::Parameters) -> String {
  show_para_tuple_list(para)
}

// lineto命令 パラメタは(x y)+
pub fn show_para_line(para: &svg::node::element::path::Parameters) -> String {
  show_para_tuple_list(para)
}

// 水平lineto命令 パラメタはx+
pub fn show_para_horizontal_line(para: &svg::node::element::path::Parameters) -> String {
  show_para_x_list(para)
}

// 水平lineto命令 パラメタはy+
pub fn show_para_vertical_line(para: &svg::node::element::path::Parameters) -> String {
  show_para_y_list(para)
}

// 2次ベジェ曲線命令 パラメタは(x1 y1 x y)+
pub fn show_para_quadratic_curve(para: &svg::node::element::path::Parameters) -> String {
  let vec = data_to_vec(para);
  let len = vec.len();
  let mut tuple_vec: Vec<String> = vec![];
  for i in 0..len / 2 {
    let x = vec[i * 2];
    let y = vec[i * 2 + 1] * -1.0;
    tuple_vec.push(format!("({}, {})", show_float(x), show_float(y)))
  }
  let len = tuple_vec.len();
  let mut main_string = String::new();
  for i in 0..len / 2 {
    let x1_y1 = &tuple_vec[i * 2];
    let x_y = &tuple_vec[i * 2 + 1];
    main_string.push_str(&format!(" ({}, {});", x1_y1, x_y))
  }
  format!("[{}]", main_string)
}

// 略式2次ベジェ曲線命令 パラメタは(x y)+
pub fn show_para_smooth_quadratic_curve(para: &svg::node::element::path::Parameters) -> String {
  show_para_tuple_list(para)
}

// 3次ベジェ曲線命令 パラメタは(x1 y1 x2 y2 x y)+
pub fn show_para_cubic_curve(para: &svg::node::element::path::Parameters) -> String {
  let vec = data_to_vec(para);
  let len = vec.len();
  let mut tuple_vec: Vec<String> = vec![];
  for i in 0..len / 2 {
    let x = vec[i * 2];
    let y = vec[i * 2 + 1] * -1.0;
    tuple_vec.push(format!("({}, {})", show_float(x), show_float(y)))
  }
  let len = tuple_vec.len();
  let mut main_string = String::new();
  for i in 0..len / 3 {
    let x1_y1 = &tuple_vec[i * 3];
    let x2_y2 = &tuple_vec[i * 3 + 1];
    let x_y = &tuple_vec[i * 3 + 2];
    main_string.push_str(&format!(" ({}, {}, {});", x1_y1, x2_y2, x_y))
  }
  format!("[{}]", main_string)
}

// 2次ベジェ曲線命令 パラメタは(x2 y2 x y)+
pub fn show_para_smooth_cubic_curve(para: &svg::node::element::path::Parameters) -> String {
  let vec = data_to_vec(para);
  let len = vec.len();
  let mut tuple_vec: Vec<String> = vec![];
  for i in 0..len / 2 {
    let x = vec[i * 2];
    let y = vec[i * 2 + 1] * -1.0;
    tuple_vec.push(format!("({}, {})", show_float(x), show_float(y)))
  }
  let len = tuple_vec.len();
  let mut main_string = String::new();
  for i in 0..len / 2 {
    let x2_y2 = &tuple_vec[i * 2];
    let x_y = &tuple_vec[i * 2 + 1];
    main_string.push_str(&format!(" ({}, {});", x2_y2, x_y))
  }
  format!("[{}]", main_string)
}
