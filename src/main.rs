extern crate gphoto;

use clap::Clap;
use std::path::Path;

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

    // let mut camera = gphoto::Camera::autodetect(&mut context).unwrap();
    // let capture = camera.capture_image(&mut context).unwrap();
    // // let mut file = gphoto::FileMedia::create(Path::new(&*capture.basename())).unwrap();
    // let mut file = gphoto::FileMedia::create(Path::new("demofile.raw")).unwrap();

    // camera.download(&mut context, &capture, &mut file).unwrap();
}

