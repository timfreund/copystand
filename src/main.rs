extern crate gphoto;

use clap::Clap;
use std::fs;
use std::path::Path;
use std::process::Command;
use rexiv2::Metadata;

#[derive(Clap)]
#[clap(version = "0.1.0", author = "Tim Freund <tim@freunds.net>")]
struct Opts {
    #[clap(long, default_value="./output")]
    outputdir: String,
    #[clap(long, default_value="scan-0000.jpg")]
    base_filename: String,
    #[clap(long, default_value="0")]
    start_page: i32,
    #[clap(long, default_value="1")]
    page_increment: i32,
}

fn main() {
    // output_directory
    // base_filename / filename_template
    // start_page
    // page_increment

    let opts: Opts = Opts::parse();
    // println!("base_filename: {}", opts.base_filename)


    let mut context = gphoto::Context::new().unwrap();

    let mut camera = gphoto::Camera::autodetect(&mut context).unwrap();
    let capture = camera.capture_image(&mut context).unwrap();
    // let mut file = gphoto::FileMedia::create(Path::new(&*capture.basename())).unwrap();
    let mut file = gphoto::FileMedia::create(Path::new("capture.cr2")).unwrap();

    camera.download(&mut context, &capture, &mut file).unwrap();
    Command::new("dcraw").arg("-T").arg("capture.cr2").status();
    Command::new("convert").arg("capture.tiff").arg("-compress").arg("Zip").arg("final.tiff").status();

    if let Ok(meta) = Metadata::new_from_path("capture.cr2") {
        if let Some(location) = meta.get_gps_info() {
            println!("Location: {:?}", location);
        }

        for tag_name in meta.get_exif_tags().unwrap(){
            let tag_value = meta.get_tag_string(&tag_name).unwrap();
            println!("{} == {}", tag_name, tag_value);
        }

        for tag_name in meta.get_iptc_tags().unwrap(){
            let tag_value = meta.get_tag_string(&tag_name).unwrap();
            println!("{} == {}", tag_name, tag_value);
        }

        meta.set_tag_numeric("Exif.Image.PageNumber", 17);
        meta.set_tag_string("Exif.Image.DocumentName", "The Demonstration Document");
        meta.save_to_file("final.tiff").expect("Couldn't save metadata");
    }
    fs::remove_file("capture.cr2");
}
