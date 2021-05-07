use std::path::PathBuf;

use svg::node::element::path::{Command, Data};
use svg::node::element::tag::Path;
use svg::parser::Event;
fn get_svg_path(path: &PathBuf) -> Data {
    let mut content = String::new();
    let mut data_to_return = Data::new();

    for event in svg::open(path, &mut content).unwrap() {
        match event {
            Event::Tag(Path, _, attributes) => {
                let data = attributes.get("d").unwrap();

                let data = Data::parse(data).unwrap();
                data_to_return = data.clone();
            }
            _ => {}
        }
    }
    data_to_return
}

use svg::node::element::{Circle, Path};
use svg::Document;

struct CircleConfig {
    center_x: f32,
    center_y: f32,
    fill_color: String,
    radius: f32,
}

fn create_circle(config: CircleConfig) -> Circle {
    Circle::new()
        .set("fill", config.fill_color)
        .set("cx", config.center_x)
        .set("cy", config.center_y)
        .set("stroke", "gray")
        .set("stroke-width", ".35")
        .set("r", config.radius)
}

fn create_mdi_path(data: Data) -> Path {
    Path::new().set("fill", "white").set("d", data)
}

struct Bounds {
    width: f32,
    height: f32,
    x: f32,
    y: f32,
}

impl Bounds {
    fn tuple(&self) -> (f32, f32, f32, f32) {
        (self.x, self.y, self.width, self.height)
    }
}

struct Point {
    x: f32,
    y: f32,
}

fn get_bounds() -> (Point, f32, Bounds) {
    // Expected bounds for material design icons.
    let mdi_bounds = Bounds {
        x: 0.0,
        y: 0.0,
        width: 24.0,
        height: 24.0,
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
    let center = Point {
        x: view_box_x + (view_box_width / 2.0),
        y: view_box_y + (view_box_height / 2.0),
    };

    let radius = (view_box_height / 2.0) - (room_for_border);
    let view_box = Bounds {
        x: view_box_x,
        y: view_box_y,
        width: view_box_width,
        height: view_box_height,
    };
    (center, radius, view_box)
}

pub fn create_svg(in_file: &PathBuf, out_file: &PathBuf) {
    let data3 = get_svg_path(in_file);
    let path3 = create_mdi_path(data3);

    let (center, radius, view_box) = get_bounds();

    let color_circle_config = CircleConfig {
        center_x: center.x,
        center_y: center.y,
        fill_color: String::from("#ED7014"),
        radius: radius,
    };

    let background_offset = 2.0;
    let background_circle_config = CircleConfig {
        center_x: center.x + background_offset,
        center_y: center.y + background_offset,
        fill_color: String::from("white"),
        radius: radius,
    };

    let circle = create_circle(color_circle_config);
    let under_circle = create_circle(background_circle_config);

    let document = Document::new()
        .set("viewBox", view_box.tuple())
        .add(under_circle)
        .add(circle)
        .add(path3);

    svg::save(out_file, &document).unwrap();
}
