mod kml;
mod gpx;

use clap::Parser;
use std::fs;
use quick_xml::de::from_str;
use quick_xml::se::Serializer;
use serde::Serialize;


use kml::Kml;
use crate::gpx::{Gpx, Track, TrackSegment, Metadata, Waypoint};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Input KML file
    #[arg(short, long, required = true)]
    input_file: String,

    /// Out file name
    #[arg(short, long)]
    out_file: Option<String>,

    /// Print to Stdout
    #[arg(short, long, default_value = "false")]
    stdout: bool,

    /// Name of Track for Metadata
    #[arg(short, long, required = true)]
    name: String,
}

fn main() {
    let args = Args::parse();
    let src = fs::read_to_string(&args.input_file).unwrap();
    let kml_item: Kml = from_str(&src).unwrap();
    let mut gpx_item: Gpx = Gpx {
        metadata: Metadata {
            name: args.name.to_string(),
            desc: "Converted from KML via kml2gpx".to_string(),
        },
        wpt: Some(Vec::new()),
        trk: Option::from(Track {
            name: args.name.to_string(),
            type_: "Walking".to_string(),
            trkseg: TrackSegment {
                trkpt: Vec::new(),
            },
        })
    };
    for placemark in kml_item.document.placemark {
        let name = placemark.name;
        let coords = placemark.location.coordinates.split(' ').collect::<Vec<&str>>();

        if coords.len() == 1 {
            // Waypoint
            let coord = coords[0].split(',').collect::<Vec<&str>>();
            //println!("{}: {}, {}", name, coord[0], coord[1]);
            if let Some(ref mut mywpt) = gpx_item.wpt {
                mywpt.push(Waypoint {
                    name,
                    lon: coord[0].to_string(),
                    lat: coord[1].to_string(),
                });
            }
        } else {
            // Route
            for coord in coords {
                let coord = coord.split(',').collect::<Vec<&str>>();
                //println!("{}: {}, {}", name, coord[0], coord[1]);
                if let Some(ref mut mytrk) = gpx_item.trk {
                    mytrk.trkseg.trkpt.push(crate::gpx::TrackPoint {
                        lon: coord[0].to_string(),
                        lat: coord[1].to_string(),
                    });
                }
            }
        }
    }
    //original unindented xml
    //let gpx = to_string(&gpx_item).unwrap();

    let mut gpx = String::new();
    let mut ser = Serializer::new(&mut gpx);
    ser.indent(' ', 2);
    gpx_item.serialize(ser).unwrap();


    if args.stdout {
        println!("{}", gpx);
    } else {
        if let Some(out_file) = args.out_file {
            fs::write(out_file, gpx.clone()).unwrap();
        } else {
            fs::write(format!("{}", &args.input_file.replace("kml", "gpx")), gpx.clone()).unwrap();
        }
    }
}
