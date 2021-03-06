extern crate clap;
extern crate notify;
extern crate config;

use std::sync::mpsc::channel;
use std::path::Path;
use clap::{Arg, App, SubCommand};
use notify::{RecommendedWatcher, Watcher};
use config::reader::from_file;

fn watch<P: AsRef<Path>>(clean_path: P, dirty_path: P) -> notify::Result<()> {
    //create channel to get events
    let (tx, rx) = channel();

    //auto select best implementation for specific platform
    let mut watcher: RecommendedWatcher = try!(Watcher::new(tx));

    //add path to be watched
    try!(watcher.watch(clean_path));
    try!(watcher.watch(dirty_path));

    loop {
        match rx.recv() {
            Ok(notify::Event{ path: Some(path), op: Ok(op) }) => {
                println!("{:?} {:?}", op, path);
            },
            Err(e) => {
                println!("watch error {}", e);
            },
            _ => ()
        }
    }
}

fn main() {
    let config_file = from_file(Path::new("./mill.conf")).unwrap();

    let matches = App::new("mill")
        .version("0.1")
        .author("Mateusz Mrowiec <matt.mrowiec@gmail.com>")
        .about("Syncs two folders")
        .arg(Arg::with_name("clean")
             .short("c")
             .long("clean")
             .value_name("FOLDER")
             .takes_value(true)
             .help("Sets the clean folder"))
        .arg(Arg::with_name("dirty")
             .short("d")
             .long("dirty")
             .value_name("FOLDER")
             .takes_value(true)
             .help("Sets the dirty folder"))
        .subcommand(SubCommand::with_name("watch")
                    .about("Watches folders for changes"))
        .get_matches();

    match config_file {
        Ok(c) => {
            println!("clean and dirty from config file");
        }
        _ => println!("yolo")
    }
    // } else {
    //     if matches.is_present("clean") && matches.is_present("dirty") {
    //         println!("clean and dirty from arguments");
    //     }
    // }
    println!("{:?}", matches);

    if matches.is_present("watch") {
        println!("Watch mode enabled");
        if let Err(err) = watch(matches.value_of("clean").unwrap(), matches.value_of("dirty").unwrap()) {
            println!("Error! {:?}", err)
        }
    }
}
