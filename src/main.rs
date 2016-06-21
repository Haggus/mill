extern crate clap;
extern crate notify;

use std::sync::mpsc::channel;
use clap::{Arg, App, SubCommand};
use notify::{RecommendedWatcher, Watcher};

fn watch() -> notify::Result<()> {
    //create channel to get events
    let (tx, rx) = channel();

    //auto select best implementation for specific platform
    let mut watcher: RecommendedWatcher = try!(Watcher::new(tx));

    //add path to be watched
    try!(watcher.watch("D:/CleanFolder"));
    try!(watcher.watch("D:/DirtyFolder"));

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

    println!("{:?}", matches);

    if matches.is_present("watch") {
        if let Err(err) = watch() {
            println!("Error! {:?}", err)
        }
    }
}
