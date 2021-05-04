use svg::node::element::path::{Command, Data};
use svg::node::element::tag::Path;
use svg::parser::Event;

fn get_svg_path() -> Data {
    let path = "images/account-group.svg";
    let mut content = String::new();
    let mut dataToReturn = Data::new();
    
    for event in svg::open(path, &mut content).unwrap() {
        match event {
            Event::Tag(Path, _, attributes) => {
                let data = attributes.get("d").unwrap();
                
                let data = Data::parse(data).unwrap();
                dataToReturn = data.clone();
            }
            _ => {}
        }
    }
   return dataToReturn;
}

use svg::Document;
use svg::node::element::{Path, Circle};

fn create_filled_circle() -> Circle {
    return Circle::new()
    .set("fill", "orange")
    .set("cx", "11.5")
    .set("cy", "13")
    .set("stroke", "gray")
    .set("stroke-width", ".35")
    .set("r", "16");
}

fn create_under_circle() -> Circle {
    return Circle::new()
    .set("fill", "white")
    .set("cx", "13.5")
    .set("cy", "16")
    .set("stroke", "gray")
    .set("stroke-width", ".35")
    .set("r", "16");
}

fn create_mdi_path(data: Data) -> Path {
    return Path::new()
    .set("fill", "white")
    .set("d", data);
}

struct Bounds {
    width: i32,
    height: i32,
    x: i32,
    y: i32
}

impl Bounds {
    pub fn new(x: i32, y: i32, width: i32, height: i32) -> Self {
        Bounds {
            x,
            y,
            width,
            height
        }
    }
}

pub fn create_svg() {
    let mdi_bounds = Bounds::new(0, 0, 24, 24);

    let room_for_border = 5;
    let room_for_3d_height = 10;
    let room_for_3d_width = 6;

    let view_box_x = mdi_bounds.x - room_for_border;
    let view_box_y = mdi_bounds.y - room_for_border;
    let view_box_width = mdi_bounds.width + room_for_border + room_for_3d_width;
    let view_box_height = mdi_bounds.height + room_for_border + room_for_3d_height;

    let view_box = (view_box_x, view_box_y, view_box_width, view_box_height);

    let data3 = get_svg_path();
    let path3 = create_mdi_path(data3);

    let circle = create_filled_circle();
    let under_circle = create_under_circle();

    let document = Document::new()
        .set("viewBox", view_box)
        .add(under_circle)
        .add(circle)
        .add(path3);

    svg::save("image.svg", &document).unwrap();
}