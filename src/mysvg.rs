use svg::node::element::path::{Command, Data};
use svg::node::element::tag::Path;
use svg::parser::Event;

// TODO
// get bounding box
// get fill
// get stroke
// get stroke width
// get data

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

pub fn create_svg() {

    let data3 = get_svg_path();
    let path3 = create_mdi_path(data3);

    let circle = create_filled_circle();
    let under_circle = create_under_circle();

    let document = Document::new()
        .set("viewBox", (-5, -5, 35, 40))
        .add(under_circle)
        .add(circle)
        .add(path3);

    svg::save("image.svg", &document).unwrap();
}