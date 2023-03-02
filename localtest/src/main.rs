use clap::Parser;
use std::path::Path;
use std::{env, fs};
use std::panic::catch_unwind;
use std::path::Prefix::Verbatim;
use log::*;
use zip;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[arg(short, long)]
    zipfile: String,
    #[arg(short, long, default_value=".")]
    dest_extraction: String
}

fn main(){

    env::set_var("RUST_LOG", "info");
    env_logger::init();
    std::process::exit(run());
}

fn run() -> i32 {
    let args = Cli::parse();
    println!("{:?}", args);
    if ! (Path::new(&args.zipfile).exists()){
        error!("File doesn't exist");
        return 1;
    }
    let zipfile = &args.zipfile;
    info!("Opening {}", args.zipfile);s

    let data = match fs::read(zipfile){
        Ok(data) => data,
        Err(e) => Vec::new(),
    };
    info!("{:?}", data);

    let file = fs::File::open(zipfile).unwrap();

    let mut archive = zip::ZipArchive::new(file).unwrap();
    for index in 0..archive.len(){
        let mut file = archive.by_index(index).expect("Failed to read archive");
        info!("{}", file.name());

        fs::write("asd.ax", file.name()).unwrap();
    }
    0
}