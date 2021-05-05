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

struct CircleConfig {
    center_x: f32,
    center_y: f32,
    fill_color: String,
    radius: f32
}

fn create_circle(config: CircleConfig) -> Circle {
    return Circle::new()
    .set("fill", config.fill_color)
    .set("cx", config.center_x)
    .set("cy", config.center_y)
    .set("stroke", "gray")
    .set("stroke-width", ".35")
    .set("r", config.radius);
}

fn create_mdi_path(data: Data) -> Path {
    return Path::new()
    .set("fill", "white")
    .set("d", data);
}

struct Bounds {
    width: f32,
    height: f32,
    x: f32,
    y: f32
}

pub fn create_svg() {
    // Expected bounds for material design icons.
    let mdi_bounds = Bounds {
        x: 0.0,
        y: 0.0,
        width: 24.0,
        height: 24.0
    };

    let room_for_border = 3.0;
    let room_for_3d_height = 10.0;
    let room_for_3d_width = 8.0;

    let view_box_x = mdi_bounds.x - 5.0;
    let view_box_y = mdi_bounds.y - 4.0;
    let view_box_width = mdi_bounds.width + room_for_border + room_for_3d_width;
    let view_box_height = mdi_bounds.height + room_for_border + room_for_3d_height;

    let _bottom_right_x = view_box_height + view_box_y;
    let _bottom_right_y = view_box_width + view_box_x;

    let center_x = view_box_x + (view_box_width / 2.0);
    let center_y = view_box_y + (view_box_height / 2.0); 

    let view_box = (view_box_x, view_box_y, view_box_width, view_box_height);

    let data3 = get_svg_path();
    let path3 = create_mdi_path(data3);

    let radius = (view_box_height / 2.0) - (room_for_border);
    println!("{}", radius);

    let color_circle_config = CircleConfig {
        center_x: center_x,
        center_y: center_y,
        fill_color: String::from("orange"),
        radius: radius
    };

    println!("{} {}", center_x, center_y);

    let background_offset = 2.0;
    let background_circle_config = CircleConfig {
        center_x: center_x + background_offset,
        center_y: center_y + background_offset,
        fill_color: String::from("white"),
        radius: radius
    };

    let circle = create_circle(color_circle_config);
    let under_circle = create_circle(background_circle_config);

    let document = Document::new()
        .set("viewBox", view_box)
        .add(under_circle)
        .add(circle)
        .add(path3);

    svg::save("image.svg", &document).unwrap();
}