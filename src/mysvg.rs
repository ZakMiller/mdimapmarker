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
    let path = "images/airplane.svg";
    let mut content = String::new();
    let mut dataToReturn = Data::new();
    
    for event in svg::open(path, &mut content).unwrap() {
        match event {
            Event::Tag(Path, _, attributes) => {
                let data = attributes.get("d").unwrap();
                
                let data = Data::parse(data).unwrap();
                dataToReturn = data.clone();
                
                for command in data.iter() {
                    match command {
                        &Command::Move(..) => println!("Move!"),
                        &Command::Line(..) => println!("Line!"),
                        _ => println!("other")
                    }
                }
            }
            _ => {}
        }
    }
   return dataToReturn;
}

use svg::Document;
use svg::node::element::Path;

fn create_border() -> Data {
    return Data::new()
    .move_to((0, 0))
    .line_by((0, 25))
    .line_by((25, 0))
    .line_by((0, -25))
    .close();
}

fn create_border_path(data: Data) -> Path {
    return Path::new()
    .set("fill", "none")
    .set("stroke", "black")
    .set("stroke-width", 2)
    .set("d", data);
}

fn create_mdi_path(data: Data) -> Path {
    return Path::new()
    .set("d", data);
}

pub fn create_svg() {
    let data = create_border();
    let path = create_border_path(data);

    let data3 = get_svg_path();
    let path3 = create_mdi_path(data3);

let document = Document::new()
    .set("viewBox", (0, 0, 50, 50))
    .add(path)
    .add(path3);

svg::save("image.svg", &document).unwrap();
}