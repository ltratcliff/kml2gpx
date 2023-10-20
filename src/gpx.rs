use serde::{Serialize};

#[derive(Debug, Serialize)]
#[serde(rename = "gpx")]
pub struct Gpx {
    pub metadata: Metadata,
    pub wpt: Option<Vec<Waypoint>>,
    pub trk: Option<Track>,
}

#[derive(Debug, Serialize)]
pub struct Metadata {
    pub name: String,
    pub desc: String,
}

#[derive(Debug, Serialize)]
pub struct Waypoint {
    pub name: String,
    #[serde(rename = "@lat")]
    pub lat: String,
    #[serde(rename = "@lon")]
    pub lon: String,
}

#[derive(Debug, Serialize)]
#[serde(rename = "trk")]
pub struct Track {
    pub name: String,
    #[serde(rename = "type")]
    pub type_: String,
    pub trkseg: TrackSegment,
}

#[derive(Debug, Serialize)]
pub struct TrackSegment {
    pub trkpt: Vec<TrackPoint>,
}

#[derive(Debug, Serialize)]
pub struct TrackPoint {
    #[serde(rename = "@lat")]
    pub lat: String,
    #[serde(rename = "@lon")]
    pub lon: String,
}