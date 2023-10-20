use serde::{Deserialize};

#[derive(Debug, Deserialize)]
#[serde(rename = "kml")]
pub struct Kml {
    #[serde(rename = "Document")]
    pub document: Document,
}
#[derive(Debug, Deserialize)]
pub struct Document {
    #[serde(rename = "Placemark")]
    pub placemark: Vec<Placemark>,
}

#[derive(Debug, Deserialize)]
pub struct Placemark {
    pub name: String,
    #[serde(rename = "Point", alias = "LineString")]
    pub location: Coordinates,
}

#[derive(Debug, Deserialize)]
pub struct Coordinates {
    pub coordinates: String,
}

// Struct working

// #[derive(Debug, Deserialize)]
// pub struct Placemark {
//     pub name: String,
//     #[serde(rename = "Point")]
//     pub point: Option<Point>,
//     #[serde(rename = "LineString")]
//     pub line_string: Option<LineString>,
// }
//
// #[derive(Debug, Deserialize)]
// pub struct Point {
//     #[serde(rename = "coordinates")]
//     pub coordinate: String,
// }
// #[derive(Debug, Deserialize)]
// pub struct LineString {
//     pub coordinates: String,
//}


// Enum working

// #[derive(Debug, Deserialize)]
// pub struct Placemark {
//     pub name: String,
//     #[serde(rename = "Point", alias = "LineString")]
//     pub location: Location,
// }
//
// #[derive(Debug, Deserialize)]
// pub enum Location {
//     #[serde(rename = "coordinates")]
//     Coordinates(String),
//}